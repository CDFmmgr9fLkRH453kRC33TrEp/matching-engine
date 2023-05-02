use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use websockets::websocket;
use std::collections::HashMap;
use std::sync::Mutex;
extern crate env_logger;
use std::net::Ipv4Addr;
use actix::Actor;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod orderbook;
mod accounts;
mod macro_calls;
mod websockets;
mod connection_server;
mod message_types;
mod parser;

use std::sync::atomic::AtomicUsize;

use std::time::SystemTime;

use std::sync::Arc;

pub use crate::accounts::TraderAccount;
// pub use crate::orderbook::TickerSymbol;
pub use crate::orderbook::OrderBook;
pub use crate::orderbook::OrderType;
pub use crate::orderbook::OrderRequest;
pub use crate::orderbook::CancelRequest;
pub use crate::orderbook::quickstart_order_book;
pub use crate::accounts::quickstart_trader_account;

use macro_calls::TickerSymbol;
use macro_calls::AssetBalances;

use macro_calls::GlobalOrderBookState;
use macro_calls::GlobalAccountState;


struct GlobalState {    
    orderbook_state:macro_calls::GlobalOrderBookState,
    account_state: macro_calls::GlobalAccountState
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let start_time = web::Data::new(SystemTime::now());
    pretty_env_logger::init();
    info!("Starting");


    // should start main server actor here, and pass in as cloned app data to websocket endpoint
    let relay_server = connection_server::Server::new().start();

    let order_count = Arc::new(AtomicUsize::new(0));

    // to do: add actix guards to confirm correctly formed requests etc. 
    // to do: add actix guards to confirm credit checks etc. 
    // to do: move this declaration to macro_calls file to generate fields automatically
    let global_orderbook_state = web::Data::new(macro_calls::GlobalOrderBookState {
        AAPL: Mutex::new(quickstart_order_book(macro_calls::TickerSymbol::AAPL, 0, 11, 10000)), 
        JNJ: Mutex::new(quickstart_order_book(macro_calls::TickerSymbol::JNJ, 0, 11, 10000)), 
    });

    let global_account_state = web::Data::new(macro_calls::GlobalAccountState {        
        Columbia_A: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_A, 100000, Ipv4Addr::new(127,16,123,1))),
        Columbia_B: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_B, 100000,Ipv4Addr::new(127,16,123,2))),
        Columbia_C: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_C, 100000, Ipv4Addr::new(127,16,123,3))),
        Columbia_D: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_D, 100000, Ipv4Addr::new(127,16,123,4))),
        Columbia_Viz: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_Viz, 100000,Ipv4Addr::new(127,16,123,0))),
    });
    *global_account_state.Columbia_A.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 30000;
    *global_account_state.Columbia_A.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 30000;
    *global_account_state.Columbia_A.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 30000; 
    *global_account_state.Columbia_A.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 30000;  
    *global_account_state.Columbia_B.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 30000;
    *global_account_state.Columbia_B.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 30000;  
    *global_account_state.Columbia_B.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 30000; 
    *global_account_state.Columbia_B.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 30000;    

    *global_account_state.Columbia_C.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 50000;
    *global_account_state.Columbia_C.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 50000;
    *global_account_state.Columbia_C.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 50000; 
    *global_account_state.Columbia_C.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 50000;  
    *global_account_state.Columbia_D.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 10000;
    *global_account_state.Columbia_D.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 10000;  
    *global_account_state.Columbia_D.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 80000; 
    *global_account_state.Columbia_D.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::JNJ).lock().unwrap() = 80000;
    // handlers discriminate based on type, so can safely pass both pieces of state here
    HttpServer::new(move || {
        App::new().service(
            web::scope("/orders")            
            .app_data(global_orderbook_state.clone()) // <- register the created data
            .app_data(global_account_state.clone()) // <- register the created data
            .app_data(web::Data::new(relay_server.clone()))
            .app_data(start_time.clone())
            .app_data(web::Data::new(order_count.clone()))
            .route("/ws", web::get().to(websockets::websocket))
            // .route("/grafana", web::get().to(websockets::websocket))
            // .route("/viz", web::get().to(websockets::websocket))
            // .route("/addOrder", web::post().to(add_order))
            // .route("/cancelOrder", web::post().to(cancel_order)), 
        )
    })
    // todo: add multiple workers here
    // .workers(2)
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
