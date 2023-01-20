use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
extern crate env_logger;


mod orderbook;
mod accounts;
mod macro_calls;
pub use crate::accounts::TraderAccount;
// pub use crate::orderbook::TickerSymbol;
pub use crate::orderbook::OrderBook;
pub use crate::orderbook::OrderType;
pub use crate::orderbook::OrderRequest;
pub use crate::orderbook::CancelRequest;
pub use crate::orderbook::quickstart_order_book;

use macro_calls::TickerSymbol;
use macro_calls::AssetBalances;


struct OrderbookState {
    orderbook : Mutex<orderbook::OrderBook>,
}

struct AccountState{
    account : Mutex<accounts::TraderAccount>
}

struct GlobalState {
    orderbooks_states: Vec<OrderbookState>,
    account_states: Vec<AccountState>
}

async fn add_order(order_request: web::Json<orderbook::OrderRequest>, data: web::Data<OrderbookState>) -> String {
    let mut orderbook = data.orderbook.lock().unwrap();
    let order_id = orderbook.handle_incoming_order_request(order_request.into_inner());
    orderbook.print_book_state();
    match order_id {
        Some(inner) => return inner.hyphenated().to_string(),
        None => String::from("Filled"),
    }
}

async fn cancel_order(cancel_request: web::Json<orderbook::CancelRequest>, data: web::Data<OrderbookState>) -> String {
    let mut orderbook = data.orderbook.lock().unwrap();
    let order_id = orderbook.handle_incoming_cancel_request(cancel_request.into_inner());
    orderbook.print_book_state();
    // todo: add proper error handling/messaging
    match order_id {        
        Some(inner) => String::from("Successfully Cancelled Order"),
        None => String::from("Error Processing Cancellation Request"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    // to do: add multiple orderbook routing etc. 
    let orderbook = web::Data::new(OrderbookState {
        orderbook: Mutex::new(orderbook::quickstart_order_book(macro_calls::TickerSymbol::AAPL, 0, 11)),
    });
    // to do: add actix guards to confirm correctly formed requests etc. 
    // to do: add actix guards to confirm credit checks etc. 
    HttpServer::new(move || {
        App::new().service(
            web::scope("/{symbol}")
            .app_data(orderbook.clone()) // <- register the created data
            .route("/addOrder", web::post().to(add_order))
            .route("/cancelOrder", web::post().to(cancel_order)), 
        )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
