use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use websockets::websocket;
use std::sync::Mutex;
extern crate env_logger;
use std::net::Ipv4Addr;


mod orderbook;
mod accounts;
mod macro_calls;
mod websockets;

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


async fn add_order(order_request: web::Json<orderbook::OrderRequest>, data: web::Data<macro_calls::GlobalOrderBookState>, accounts_data: web::Data<macro_calls::GlobalAccountState>) -> String {
    let order_request_inner = order_request.into_inner();
    let symbol = &order_request_inner.symbol;
    // Todo: refactor into match statement, put into actix guard?
    if (order_request_inner.order_type == orderbook::OrderType::Buy) {
        // ISSUE: This should decrement cents_balance to avoid racing to place two orders before updating cents_balance
        // check if current cash balance - outstanding orders supports order
        // nevermind, as long as I acquire and hold a lock during the entire order placement attempt, it should be safe                
        let cent_value = &order_request_inner.amount * &order_request_inner.price;            
        if (accounts_data.index_ref(order_request_inner.trader_id).lock().unwrap().cents_balance < cent_value) {
            return String::from("Error Placing Order: The total value of order is greater than current account balance");
        }  
    };

    if (order_request_inner.order_type == orderbook::OrderType::Sell) {
        // ISSUE: This should decrement cents_balance to avoid racing to place two orders before updating cents_balance
        // check if current cash balance - outstanding orders supports order
        // nevermind, as long as I acquire and hold a lock during the entire order placement attempt, it should be safe                
        if (*accounts_data.index_ref(order_request_inner.trader_id).lock().unwrap().asset_balances.index_ref(symbol).lock().unwrap() < order_request_inner.amount) {
            return String::from("Error Placing Order: The total amount of this trade would take your account short");
        }  
    };
    let orderbook = data.index_ref(symbol);
    // ISSUE: need to borrow accounts as mutable without knowing which ones will be needed to be borrowed
    // maybe pass in immutable reference to entire account state, and only acquire the locks for the mutex's that it turns out we need
    let order_id =  data.index_ref(symbol).lock().unwrap().handle_incoming_order_request(order_request_inner, &accounts_data);
    orderbook.lock().unwrap().print_book_state();
    match order_id {
        Some(inner) => {
            return inner.hyphenated().to_string()
        },
        None => String::from("Filled"),
    }
}

async fn cancel_order(cancel_request: web::Json<orderbook::CancelRequest>, data: web::Data<macro_calls::GlobalOrderBookState>) -> String {
    let cancel_request_inner = cancel_request.into_inner();
    let symbol = &cancel_request_inner.symbol;
    let order_id = data.index_ref(symbol).lock().unwrap().handle_incoming_cancel_request(cancel_request_inner);
    // todo: add proper error handling/messaging
    match order_id {        
        Some(inner) => String::from(format!("Successfully cancelled order {:?}", inner.order_id)),
        None => String::from("Issue processing cancellation request, the order may have been already executed."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    // to do: add actix guards to confirm correctly formed requests etc. 
    // to do: add actix guards to confirm credit checks etc. 
    // to do: move this declaration to macro_calls file to generate fields automatically
    let global_orderbook_state = web::Data::new(macro_calls::GlobalOrderBookState {
        AAPL: Mutex::new(quickstart_order_book(macro_calls::TickerSymbol::AAPL, 0, 11)), 
        JNJ: Mutex::new(quickstart_order_book(macro_calls::TickerSymbol::JNJ, 0, 11)), 
    });

    let global_account_state = web::Data::new(macro_calls::GlobalAccountState {        
        Columbia_A: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_A, 10, Ipv4Addr::new(172,16,123,1))),
        Columbia_B: Mutex::new(accounts::quickstart_trader_account(macro_calls::TraderId::Columbia_B, 10,Ipv4Addr::new(172,16,123,2))),
    });
    *global_account_state.Columbia_A.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 10;
    *global_account_state.Columbia_A.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 10;
    *global_account_state.Columbia_B.lock().unwrap().asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 10;
    *global_account_state.Columbia_B.lock().unwrap().net_asset_balances.index_ref(&macro_calls::TickerSymbol::AAPL).lock().unwrap() = 10;
    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });
    // handlers discriminate based on type, so can safely pass both pieces of state here
    HttpServer::new(move || {
        App::new().service(
            web::scope("/orders")            
            .app_data(global_orderbook_state.clone()) // <- register the created data
            .app_data(global_account_state.clone()) // <- register the created data
            .route("/ws", web::get().to(websockets::websocket))
            // .route("/viz", web::get().to(websockets::websocket))
            .route("/addOrder", web::post().to(add_order))
            .route("/cancelOrder", web::post().to(cancel_order)), 
        )
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
