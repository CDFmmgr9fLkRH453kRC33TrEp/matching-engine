// todo: separate websocket handling actors from actors handling state/fill message queueing?
// i.e. actor handling state subscribes to fill messages, then checks if there is a websocket actor associated with the account and then sends fill message to them
// actors handling state have static lifetime and are spawned when  program starts
// actors handling websockets are spawned when new connection occurs, and end when connection is dropped.
// otherwise will add to fill message backlog.

use actix::prelude::*;
use actix_web::web::Bytes;
use actix_web::Error;
use actix_web_actors::ws;
use log::info;
use plotters::coord::types;
use std::env;
use std::f32::consts::E;
use std::fmt::format;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use std::sync::atomic::AtomicUsize;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use actix_broker::{ArbiterBroker, Broker, BrokerIssue, BrokerSubscribe, SystemBroker};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(4);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
use crate::message_types::OpenMessage;
use crate::orderbook::Fill;
use crate::websockets::ws::CloseCode::Policy;
use crate::websockets::ws::CloseReason;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
extern crate env_logger;

use queues::IsQueue;

use actix::prelude::*;

use std::any::type_name;

// mod orderbook;
// mod accounts;
// mod macro_calls;
// mod websockets;

pub use crate::accounts::TraderAccount;
// pub use crate::orderbook::TickerSymbol;
pub use crate::accounts::quickstart_trader_account;
use crate::macro_calls::TraderIp;
pub use crate::orderbook::quickstart_order_book;
pub use crate::orderbook::CancelRequest;
pub use crate::orderbook::OrderBook;
pub use crate::orderbook::OrderRequest;
pub use crate::orderbook::OrderType;
use crate::{macro_calls, orderbook};

use crate::macro_calls::AssetBalances;
use crate::macro_calls::TickerSymbol;

use crate::macro_calls::GlobalAccountState;
use crate::macro_calls::GlobalOrderBookState;

// use crate::parser;

struct GlobalState {
    orderbook_state: crate::macro_calls::GlobalOrderBookState,
    account_state: crate::macro_calls::GlobalAccountState,
}

fn add_order(
    order_request: crate::orderbook::OrderRequest,
    data: &web::Data<crate::macro_calls::GlobalOrderBookState>,
    accounts_data: &web::Data<crate::macro_calls::GlobalAccountState>,
    // start_time: &web::Data<SystemTime>,
    relay_server_addr: &web::Data<Addr<crate::connection_server::Server>>,
    order_counter: &web::Data<Arc<AtomicUsize>>,
) -> String {
    debug!("Add Order Triggered!");

    let order_request_inner = order_request;
    let symbol = &order_request_inner.symbol;

    // Todo: refactor into match statement, put into actix guard?
    if (order_request_inner.order_type == crate::orderbook::OrderType::Buy) {
        // ISSUE: This should decrement cents_balance to avoid racing to place two orders before updating cents_balance
        // check if current cash balance - outstanding orders supports order
        // nevermind, as long as I acquire and hold a lock during the entire order placement attempt, it should be safe
        let cent_value = &order_request_inner.amount * &order_request_inner.price;
        if (accounts_data
            .index_ref(order_request_inner.trader_id)
            .lock()
            .unwrap()
            .net_cents_balance
            < cent_value)
        {
            return String::from("Error Placing Order: The total value of order is greater than current account balance");
        }
        accounts_data
            .index_ref(order_request_inner.trader_id)
            .lock()
            .unwrap()
            .net_cents_balance -= order_request_inner.price * order_request_inner.amount;
    };
    if (order_request_inner.order_type == crate::orderbook::OrderType::Sell) {
        // ISSUE: This should decrement cents_balance to avoid racing to place two orders before updating cents_balance
        // check if current cash balance - outstanding orders supports order
        // nevermind, as long as I acquire and hold a lock during the entire order placement attempt, it should be safe
        if (*accounts_data
            .index_ref(order_request_inner.trader_id)
            .lock()
            .unwrap()
            .net_asset_balances
            .index_ref(symbol)
            .lock()
            .unwrap()
            < order_request_inner.amount)
        {
            debug!("Error: attempted short sell");
            return String::from(
                "Error Placing Order: The total amount of this trade would take your account short",
            );
        }

        *accounts_data
            .index_ref(order_request_inner.trader_id)
            .lock()
            .unwrap()
            .net_asset_balances
            .index_ref(symbol)
            .lock()
            .unwrap() -= order_request_inner.amount;
    };

    debug!(
        "Account has {:?} lots of {:?}",
        &accounts_data
            .index_ref(order_request_inner.trader_id)
            .lock()
            .unwrap()
            .asset_balances
            .index_ref(symbol)
            .lock()
            .unwrap(),
        symbol
    );
    debug!(
        "Account has {:?} cents",
        &accounts_data
            .index_ref(order_request_inner.trader_id)
            .lock()
            .unwrap()
            .cents_balance
    );

    // let orderbook = data.index_ref(symbol);
    // let jnj_orderbook = data.index_ref(&crate::macro_calls::TickerSymbol::JNJ);
    // jnj_orderbook.lock().unwrap().print_book_state();
    // ISSUE: need to borrow accounts as mutable without knowing which ones will be needed to be borrowed
    // maybe pass in immutable reference to entire account state, and only acquire the locks for the mutex's that it turns out we need
    let order_id = data
        .index_ref(&symbol.clone())
        .lock()
        .unwrap()
        .handle_incoming_order_request(order_request_inner.clone(), accounts_data, relay_server_addr, order_counter);
    // let book_state = data.index_ref(symbol).lock().unwrap();

    match order_id {
        Some(inner) => return inner.to_string(),
        None => String::from("Filled"),
    }
}

async fn cancel_order(
    cancel_request: web::Json<crate::orderbook::CancelRequest>,
    data: web::Data<crate::macro_calls::GlobalOrderBookState>,
    order_counter: &web::Data<Arc<AtomicUsize>>,
    relay_server_addr: &web::Data<Addr<crate::connection_server::Server>>,
) -> String {
    let cancel_request_inner = cancel_request.into_inner();
    let symbol = &cancel_request_inner.symbol;
    let order_id = data
        .index_ref(symbol)
        .lock()
        .unwrap()
        .handle_incoming_cancel_request(cancel_request_inner, order_counter, relay_server_addr);
    // todo: add proper error handling/messaging
    match order_id {
        Some(inner) => String::from(format!("Successfully cancelled order {:?}", inner.order_id)),
        None => String::from(
            "Issue processing cancellation request, the order may have been already executed.",
        ),
    }
}

pub struct MyWebSocketActor {
    connection_ip: TraderIp,
    hb: Instant,
    global_account_state: web::Data<crate::macro_calls::GlobalAccountState>,
    global_orderbook_state: web::Data<crate::macro_calls::GlobalOrderBookState>,
    // for testing. 
    start_time: web::Data<SystemTime>,
    t_orders: usize,
    relay_server_addr: web::Data<Addr<crate::connection_server::Server>>,
    order_counter: web::Data<Arc<AtomicUsize>>,
}

impl MyWebSocketActor {
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                warn!("Client Timed Out :(");
                ctx.stop();
                return;
            }
            debug!("sent ping message");
            ctx.ping(b"");
        });
    }
}

pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    orderbook_data: web::Data<crate::macro_calls::GlobalOrderBookState>,
    accounts_data: web::Data<crate::macro_calls::GlobalAccountState>,
    start_time: web::Data<SystemTime>,
    relay_server_addr: web::Data<Addr<crate::connection_server::Server>>,
    order_counter: web::Data<Arc<AtomicUsize>>,
) -> Result<HttpResponse, Error> {
    let conninfo = req.connection_info().clone();
    log::info!(
        "New websocket connection with peer_addr {:?}",
        conninfo.peer_addr()
    );

    // todo: change ws:start to be adding a stream to existing actor.

    let id = macro_calls::ip_to_id(conninfo.peer_addr().unwrap().parse().unwrap()).unwrap();

    // let password_needed = req.take_payload( );

    ws::start(
        MyWebSocketActor {
            connection_ip: req
                .connection_info()
                .realip_remote_addr()
                .unwrap()
                .clone()
                .parse()
                .unwrap(),
            hb: Instant::now(),
            global_account_state: accounts_data.clone(),
            global_orderbook_state: orderbook_data.clone(),
            start_time: start_time.clone(),
            t_orders: 0,
            relay_server_addr: relay_server_addr.clone(),
            order_counter: order_counter.clone(),
        },
        &req,
        stream,
    )
}

impl Actor for MyWebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<orderbook::OrderBook>(ctx);
        // self.subscribe_system_async::<orderbook::LimLevUpdate>(ctx);
        self.relay_server_addr.do_send(OpenMessage{
            ip : self.connection_ip,
            id: crate::macro_calls::ip_to_id(self.connection_ip).unwrap(), 
            addr: ctx.address().recipient()
        });
        debug!("Subscribed");
        self.hb(ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let account_id = crate::macro_calls::ip_to_id(self.connection_ip).unwrap();
        let curr_actor = &mut self
            .global_account_state
            .index_ref(account_id)
            .lock()
            .unwrap()
            .current_actor;
        match curr_actor {
            Some(x) => {
                *curr_actor = None;
            }
            None => error!("Error, no websocket connected?"),
        }
        info!(
            "Websocket connection ended (peer_ip:{}).",
            self.connection_ip
        );
        info!("curr_order_count {:?}", self.order_counter.load(std::sync::atomic::Ordering::Relaxed))
    }
}

/// Define handler for `Fill` message
impl Handler<Arc<orderbook::Fill>> for MyWebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: Arc<orderbook::Fill>, ctx: &mut Self::Context) {
        let fill_event = msg;
        ctx.text(format!(
            "{:?} sells to {:?}: {:?} lots of {:?} @ ${:?}",
            fill_event.sell_trader_id,
            fill_event.buy_trader_id,
            fill_event.amount,
            fill_event.symbol,
            fill_event.price
        ));
    }
}

/// Define handler for `OrderBookUpdate` message
impl Handler<orderbook::OrderBook> for MyWebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: orderbook::OrderBook, ctx: &mut Self::Context) {
        debug!("Orderbook Message Received");
        // msg.print_book_state();
        ctx.text(format!("{:?}", &msg.get_book_state()));
    }
}

impl Handler<Arc<orderbook::LimLevUpdate>> for MyWebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: Arc<orderbook::LimLevUpdate>, ctx: &mut Self::Context) {
        // debug!("LimLevUpdate Message Received");
        // msg.print_book_state()
        ctx.text(serde_json::to_string(&(*msg).clone()).unwrap());
    }
}

// The `StreamHandler` trait is used to handle the messages that are sent over the socket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocketActor {
    // started() function registers this with the trader account's actor addr
    // and sends out all fill messages in the trader account's queue

    fn finished(&mut self, ctx: &mut Self::Context) {
        ctx.stop()
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        // Broker::<SystemBroker>::issue_async(self.global_orderbook_state);
        let connection_ip = self.connection_ip;

        // if (connection_ip
        //     == env::var("GRAFANAIP")
        //         .expect("$GRAFANAIP is not set")
        //         .parse::<TraderIp>()
        //         .unwrap())
        // {
        //     info!("New grafana connection");
        //     let dur = env::var("GRAFANAPOLLDUR")
        //         .expect("$GRAFANAPOLLDUR is not set")
        //         .parse::<u64>()
        //         .unwrap();
        //     ctx.run_interval(Duration::new(dur, 0), |act, ctx| {
        //         // ctx.text("hello");
        //     });
        // } else {
        let account_id: macro_calls::TraderId = crate::macro_calls::ip_to_id(connection_ip).unwrap();
        debug!("Trader with id {:?} connected.", account_id);
        {
            let curr_actor = &mut self
                .global_account_state
                .index_ref(account_id)
                .lock()
                .unwrap()
                .current_actor;

            match curr_actor {
                Some(x) => {
                    if (connection_ip
                        != env::var("GRAFANAIP")
                            .expect("$GRAFANAIP is not set")
                            .parse::<TraderIp>()
                            .unwrap())
                    {
                        error!("Trader_id already has websocket connected");
                        ctx.stop();
                    }
                }
                None => *curr_actor = Some(ctx.address()),
            }
        }
        let message_queue = &mut self
            .global_account_state
            .index_ref(account_id)
            .lock()
            .unwrap()
            .message_backup;

        // ctx.text("new message");
        while (message_queue.size() != 0) {
            // println!("Message #{:?}", message_queue.size());
            let fill_event = message_queue.remove().unwrap();
            // println!("{:?} sells to {:?}: {:?} lots of {:?} @ ${:?}",
            // fill_event.sell_trader_id,
            // fill_event.buy_trader_id,
            // fill_event.amount,
            // fill_event.symbol,
            // fill_event.price);
            ctx.text(format!(
                "{:?} sells to {:?}: {:?} lots of {:?} @ ${:?}",
                fill_event.sell_trader_id,
                fill_event.buy_trader_id,
                fill_event.amount,
                fill_event.symbol,
                fill_event.price
            ));
            // }
            // should send global orderbook state.
        }
        debug!("Sending serialized orderbook state.");
        ctx.text(serde_json::to_string(&self.global_orderbook_state).unwrap());
    }
    // finished() function removes the trader account's actor addr

    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // if self.connection_ip != env::var("GRAFANAIP").expect("GRAFANAIP not set").parse::<TraderIp>().unwrap() {
        let t_start = SystemTime::now();
        self.t_orders += 1;
        debug!("total msgs received:{:?}", self.t_orders);

        match msg {
            Ok(ws::Message::Text(msg)) => {
                let t_start_o = SystemTime::now();
                // handle incoming JSON as usual.

                // this is very expensive, should implement more rigid parsing. 
                // TODO: switch to handling in fix/binary instead of json to improve speed
                let d = &msg.to_string();

                // NEED TO ALLOW FOR CANCEL REQUESTS!!
                // Should create meta "message" type which contains either cancel or order

                let t: OrderRequest = serde_json::from_str(d).unwrap();

                let connection_ip = self.connection_ip;

                let password_needed = self.global_account_state.index_ref(t.trader_id).lock().unwrap().password;

                let ip_needed = self
                    .global_account_state
                    .index_ref(t.trader_id)
                    .lock()
                    .unwrap()
                    .trader_ip;

                if (password_needed != t.password) {
                    warn!("Invalid password for provided trader_id: {}", connection_ip);
                    ctx.text("invalid password for provided trader id.");
                } else {
                
                // if (connection_ip != ip_needed) {
                //     // TODO: switch to login/password instead of id/ip verification to handle reconnections
                //     warn!("Invalid ip for provided trader_id: {}", connection_ip);
                //     warn!("connection_ip: {}", connection_ip);
                //     warn!("ip_needed: {}", ip_needed);
                //     ctx.text("invalid ip for provided trader id.");
                //     // ctx.close(None);
                // } else {

                    // should be passed to fix parser here


                    let symbol = t.clone().symbol;

                    let res =
                        add_order(t, &self.global_orderbook_state, &self.global_account_state, &self.relay_server_addr, &self.order_counter);

                    let t_elp = t_start_o.elapsed().unwrap();
                    debug!("secs for last order (inside match): {:?}", t_elp);
                    // elapsed is taking a non negligible time
                    let secs_elapsed = self
                        .start_time
                        .clone()
                        .into_inner()
                        .as_ref()
                        .elapsed()
                        .unwrap();
                    debug!(
                        "time_elapsed from start: {:?}",
                        usize::try_from(secs_elapsed.as_secs()).unwrap()
                    );
                    debug!("total orders processed:{:?}", self.order_counter.load(std::sync::atomic::Ordering::SeqCst));
                    debug!(
                        "orders/sec: {:?}",
                        self.order_counter.load(std::sync::atomic::Ordering::SeqCst) / usize::try_from(secs_elapsed.as_secs()).unwrap()
                    );

                    // println!("res: {}", res);
                    // let msg = self.global_orderbook_state.index_ref(&t.symbol).lock().unwrap().to_owned();
                    // debug!("Issuing Async Msg");
                    // Broker::<SystemBroker>::issue_async(msg);
                    // debug!("Issued Async Msg");
                    // println!("{:?}", serde_json::to_string_pretty(&t));

                    // measured @~14microseconds.
                    // for some reason goes up as more orders are added :(
                    ctx.text(res);
                }
            }

            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                // info!("Ping Received");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                // info!("Pong Received");
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                // let account_id = crate::macro_calls::ip_to_id(connection_ip).unwrap();

                //  self
                // .global_account_state
                // .index_ref(account_id)
                // .lock()
                // .unwrap()
                // .current_actor = None;
                info!("Received close message, closing context.");
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                error!("Error reading message, stopping context.");
                ctx.stop();
            } // }
        }
        let t_elp = t_start.elapsed().unwrap();
        debug!("secs for last request: {:?}", t_elp);
    }
}
