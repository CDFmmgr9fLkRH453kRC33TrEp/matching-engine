use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
use log::{debug, trace, warn, info, error};

struct OrderbookState {
    orderbook : Mutex<orderbook::OrderBook>,
}

async fn add_order(order_request: web::Json<orderbook::OrderRequest>, data: web::Data<OrderbookState>) -> String {
    // println!("{:?}");
    let mut orderbook = data.orderbook.lock().unwrap();
    let order_id = orderbook.handle_incoming_order_request(order_request.into_inner());
    orderbook.print_book_state();
    match order_id {
        Some(inner) => return inner.hyphenated().to_string(),
        None => String::from("Filled"),
    }
}

async fn cancel_order(cancel_request: web::Json<orderbook::CancelRequest>, data: web::Data<OrderbookState>) -> String {
    // println!("{:?}");
    let mut orderbook = data.orderbook.lock().unwrap();
    let order_id = orderbook.handle_incoming_cancel_request(cancel_request.into_inner());
    orderbook.print_book_state();
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
        orderbook: Mutex::new(orderbook::quickstart_order_book(orderbook::TickerSymbol::AAPL, 0, 11)),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(orderbook.clone()) // <- register the created data
            .route("/orders/addOrder", web::post().to(add_order))
            .route("/orders/cancelOrder", web::post().to(cancel_order))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
