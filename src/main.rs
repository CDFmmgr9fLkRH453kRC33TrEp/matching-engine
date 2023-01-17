use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct OrderbookState {
    orderbook : Mutex<orderbook::OrderBook>,
}

async fn hello(data: web::Data<OrderbookState>) -> String {
    let mut orderbook = data.orderbook.lock().unwrap();
    let s = orderbook.symbol.to_string();
    println!("{:?}", s);
    s
}

async fn add_order(order_request: web::Json<orderbook::OrderRequest>, data: web::Data<OrderbookState>) -> String {
    // println!("{:?}");
    let mut orderbook = data.orderbook.lock().unwrap();
    orderbook.handle_incoming_order_request(order_request.into_inner());
    orderbook.print_book_state();
    let s = orderbook.symbol.to_string();
    s
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orderbook = web::Data::new(OrderbookState {
        orderbook: Mutex::new(orderbook::quickstart_order_book(orderbook::TickerSymbol::AAPL, 0, 11)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(orderbook.clone()) // <- register the created data
            .route("/", web::post().to(add_order))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
