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

use macro_calls::GlobalOrderBookState;
use macro_calls::GlobalAccountState;

// impl GlobalOrderBookState {
//     pub fn index_ref (&self, symbol:&TickerSymbol) -> &Mutex<crate::orderbook::OrderBook>{
//         match symbol {
//             $($name => {&self.$name}, )*
//         }
//     }
//     pub fn index_ref_mut (&mut self, symbol:&TickerSymbol) -> &mut Mutex<crate::orderbook::OrderBook>{
//         match symbol {
//             $($name => { &mut self.$name}, )*
//         }
//     }
// }


async fn add_order(order_request: web::Json<orderbook::OrderRequest>, data: web::Data<macro_calls::GlobalOrderBookState>) -> String {
    let order_request_inner = order_request.into_inner();
    let symbol = &order_request_inner.symbol;
    let orderbook = data.index_ref(symbol);
    let order_id =  data.index_ref(symbol).lock().unwrap().handle_incoming_order_request(order_request_inner);
    orderbook.lock().unwrap().print_book_state();
    match order_id {
        Some(inner) => return inner.hyphenated().to_string(),
        None => String::from("Filled"),
    }
}

async fn cancel_order(cancel_request: web::Json<orderbook::CancelRequest>, data: web::Data<macro_calls::GlobalOrderBookState>) -> String {
    let cancel_request_inner = cancel_request.into_inner();
    let symbol = &cancel_request_inner.symbol;
    let order_id = data.index_ref(symbol).lock().unwrap().handle_incoming_cancel_request(cancel_request_inner);
    // todo: add proper error handling/messaging
    match order_id {        
        Some(inner) => String::from(format!("Successfully Cancelled Order {:?}", inner.order_id)),
        None => String::from("Error Processing Cancellation Request"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    // to do: add actix guards to confirm correctly formed requests etc. 
    // to do: add actix guards to confirm credit checks etc. 
    // to do: move this declaration to macro_calls file to generate fields automatically
    let global_order_book_state = web::Data::new(macro_calls::GlobalOrderBookState {
        AAPL: Mutex::new(quickstart_order_book(macro_calls::TickerSymbol::AAPL, 0, 11)), 
        JNJ: Mutex::new(quickstart_order_book(macro_calls::TickerSymbol::JNJ, 0, 11)), 
    });

    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });

    HttpServer::new(move || {
        App::new().service(
            web::scope("/orders")
            .app_data(global_order_book_state.clone()) // <- register the created data
            .route("/addOrder", web::post().to(add_order))
            .route("/cancelOrder", web::post().to(cancel_order)), 
        )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
