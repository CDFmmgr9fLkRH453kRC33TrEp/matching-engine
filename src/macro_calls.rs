use std::collections::HashMap;
use std::net::Ipv4Addr;
pub type TraderIp = std::net::Ipv4Addr;
use std::io;
use actix::Addr;
use crate::websockets::MyWebSocketActor;


// TODO: clean up this mess!!!!!
pub fn ip_to_id (ip: Ipv4Addr) -> Result<crate::macro_calls::TraderId, io::Error> {
    if (ip == Ipv4Addr::new(127,16,123,1)) {
        return Ok(crate::macro_calls::TraderId::Columbia_A);
    } else if (ip == Ipv4Addr::new(127,16,123,2)){
        return Ok(crate::macro_calls::TraderId::Columbia_B);
    } else if (ip == Ipv4Addr::new(127,16,123,0)){
        return Ok(crate::macro_calls::TraderId::Columbia_Viz);
    } else {
        panic!("not a known ip");
    }
}

use strum_macros::EnumIter;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use crate::accounts::TraderAccount;
use crate::orderbook::OrderBook;
// use crate::orderbook::TraderId;
// hello

macro_rules! generate_enum {
    ([$($name:ident),*]) => {
        #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
        pub enum TickerSymbol {
            $($name, )*
        }
    };
}

macro_rules! generate_accounts_enum {
    ([$($name:ident),*]) => {
        #[derive(Debug, Copy, Clone, Deserialize, Serialize, EnumIter)]
        pub enum TraderId {
            $($name, )*
        }       
    };
}

macro_rules! generate_account_balances_struct {
    ([$($name:ident),*]) => {
        #[derive(Debug)]
        pub struct AssetBalances {
            $($name: Mutex<usize>, )*
        }    

        impl AssetBalances {
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &Mutex<usize>{
                match symbol {
                    $(TickerSymbol::$name => {&self.$name}, )*
                }
            }     
            
            pub fn new() -> Self {
                Self { 
                    $($name: Mutex::new(0), )*
                 }
            }
               
        }
    };
}

macro_rules! generate_global_state {
    ([$($name:ident),*], [$($account_id:ident),*]) => {
        #[derive(Debug, Serialize)]
        pub struct GlobalOrderBookState {
            $(pub $name: Mutex<crate::orderbook::OrderBook>, )*
        }
        
        impl GlobalOrderBookState {
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &Mutex<crate::orderbook::OrderBook>{
                match symbol {
                    $(TickerSymbol::$name => {&self.$name}, )*
                }
            }
        }
        
        pub struct GlobalAccountState {
            $(pub $account_id: Mutex<crate::accounts::TraderAccount>, )*
        }

        impl GlobalAccountState {
            pub fn index_ref (&self, account_id:crate::macro_calls::TraderId,) -> &Mutex<crate::accounts::TraderAccount>{
                match account_id {
                    $(TraderId::$account_id => {&self.$account_id}, )*
                }
            }       
                    
        }
    };
}  
generate_enum!([
        AAPL,
        JNJ
    ]);
generate_account_balances_struct!([
        AAPL,
        JNJ
    ]);
generate_global_state!([
        AAPL,
        JNJ
    ], [
        Columbia_A,
        Columbia_B,
        Columbia_Viz
    ]);
generate_accounts_enum!([
        Columbia_A,
        Columbia_B,
        Columbia_Viz
    ]);