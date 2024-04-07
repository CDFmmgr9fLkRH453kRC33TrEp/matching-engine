use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use websockets::websocket;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
extern crate env_logger;
use std::net::Ipv4Addr;
use actix::Actor;
use crate::config::TraderId;


extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod orderbook;
mod api_messages;
mod accounts;
mod config;
mod websockets;
mod connection_server;
mod message_types;
// mod parser;

use std::sync::atomic::AtomicUsize;

use std::time::SystemTime;

use std::sync::Arc;

pub use crate::accounts::TraderAccount;
// pub use crate::orderbook::TickerSymbol;
pub use crate::orderbook::OrderBook;
pub use crate::orderbook::OrderType;
pub use crate::orderbook::quickstart_order_book;
pub use crate::accounts::quickstart_trader_account;

use config::TickerSymbol;
use config::AssetBalances;

use config::GlobalOrderBookState;
use config::GlobalAccountState;


#[derive(Debug, Serialize, Deserialize)]
struct GlobalState {    
    global_orderbook_state: GlobalOrderBookState,
    global_account_state: GlobalAccountState
}

impl GlobalState {
    fn dump_state(self, log_file: fs::File){
        println!("{:?}", json!(self))
    }
    fn load_state(self, log_file: fs::File){
        
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let start_time = web::Data::new(SystemTime::now());
    pretty_env_logger::init();
    info!("Starting...");

    // todo: convert to proper strongly typed deserialization
    // convert asset enum to be like AAPL(Asset) 
    // Asset as struct which includes long name, max price, etc. 
    // convert global state to an enum generated at compile time
    // let file = File::open(Path::new("./config.json")).unwrap();
    // let reader = BufReader::new(file);

    // let value: Value = serde_json::from_reader(reader).unwrap();

    // let assets_arr = value["assets"].as_array().unwrap();
    // let accounts_arr = value["accounts"].as_array().unwrap();
    
    // should start main server actor here, and pass in as cloned app data to websocket endpoint
    let relay_server = connection_server::Server::new().start();

    let order_count = Arc::new(AtomicUsize::new(0));

    // to do: add actix guards to confirm correctly formed requests etc. 
    // to do: add actix guards to confirm credit checks etc. 
    // to do: move this declaration to macro_calls file to generate fields automatically
    let global_orderbook_state =config::GlobalOrderBookState {
        AAPL: Mutex::new(quickstart_order_book(config::TickerSymbol::AAPL, 0, 11, 10000)), 
        JNJ: Mutex::new(quickstart_order_book(config::TickerSymbol::JNJ, 0, 11, 10000)), 
    };

    // todo: abstract to config file, this is disgusting (see config.json -> build.rs -> config.rs)
    let global_account_state = config::GlobalAccountState {        
        Columbia_A: Mutex::new(accounts::quickstart_trader_account(config::TraderId::Columbia_A, 100000, ['c','u','_','a'])),
        Columbia_B: Mutex::new(accounts::quickstart_trader_account(config::TraderId::Columbia_B, 100000,  ['c','u','_','b'])),
        Columbia_C: Mutex::new(accounts::quickstart_trader_account(config::TraderId::Columbia_C, 100000,  ['c','u','_','c'])),
        Columbia_D: Mutex::new(accounts::quickstart_trader_account(config::TraderId::Columbia_D, 100000,  ['c','u','_','d'])),
        Columbia_Viz: Mutex::new(accounts::quickstart_trader_account(config::TraderId::Columbia_Viz, 100000,  ['c','u','_','v'])),
        Price_Enforcer: Mutex::new(accounts::quickstart_trader_account(config::TraderId::Price_Enforcer, 100000,  ['p','e','n','f'])),
    };

    // todo: abstract to config file, this is disgusting (see config.json -> build.rs -> config.rs)
    *global_account_state.Columbia_A.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 30000;
    *global_account_state.Columbia_A.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 30000;
    *global_account_state.Columbia_A.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 30000; 
    *global_account_state.Columbia_A.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 30000;  
    *global_account_state.Columbia_B.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 30000;
    *global_account_state.Columbia_B.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 30000;  
    *global_account_state.Columbia_B.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 30000; 
    *global_account_state.Columbia_B.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 30000;    

    *global_account_state.Columbia_C.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 50000;
    *global_account_state.Columbia_C.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 50000;
    *global_account_state.Columbia_C.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 50000; 
    *global_account_state.Columbia_C.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 50000;  
    *global_account_state.Columbia_D.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 10000;
    *global_account_state.Columbia_D.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::AAPL).lock().unwrap() = 10000;  
    *global_account_state.Columbia_D.lock().unwrap().asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 80000; 
    *global_account_state.Columbia_D.lock().unwrap().net_asset_balances.index_ref(&config::TickerSymbol::JNJ).lock().unwrap() = 80000;

    
    let global_state = web::Data::new(GlobalState { 
        global_orderbook_state: global_orderbook_state, 
        global_account_state: global_account_state
    });

    


    // handlers discriminate based on type, so can safely pass both pieces of state here
    HttpServer::new(move || {
        App::new().service(
            web::scope("/orders")
            .app_data(global_state.clone())         
            // .app_data(global_orderbook_state.clone()) // <- register the created data
            // .app_data(global_account_state.clone()) // <- register the created data
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
