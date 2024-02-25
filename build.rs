// build.rs
use bytes::Bytes;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, io, process};


fn main() {
    let symbols = "
    [
        AAPL,
        JNJ
    ]
    ";
    let account_ids = "
    [
        Columbia_A,
        Columbia_B,
        Columbia_C,
        Columbia_D,
        Columbia_Viz
    ]
    ";

    // let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "/Users/caidan/projects/exchange_simulator/matching-engine/src";
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f = File::create(&dest_path).unwrap();
    let content = format!(
        "
use std::collections::HashMap;
use std::net::Ipv4Addr;
pub type TraderIp = std::net::Ipv4Addr;
use std::io;
use actix::Addr;
use crate::websockets::MyWebSocketActor;

use strum_macros::EnumIter;
use core::fmt::Debug;
use serde::{{Deserialize, Serialize}};
use std::sync::Mutex;
use uuid::Uuid;
use crate::accounts::TraderAccount;
use crate::orderbook::OrderBook;
use std::str::FromStr;
// use crate::orderbook::TraderId;
// hello

macro_rules! generate_enum {{
    ([$($name:ident),*]) => {{
        #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
        pub enum TickerSymbol {{
            $($name, )*
        }}
        impl TryFrom<&'static str> for TickerSymbol {{
            type Error = &'static str;

            fn try_from(s: &'static str) -> Result<TickerSymbol, &'static str> {{
                match s {{
                    $(stringify!($name) => Ok(TickerSymbol::$name),)+
                    _ => Err(\"Invalid String\")
                }}
            }}
        }}

        impl FromStr for TickerSymbol {{
            type Err = &'static str;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {{
                match s {{
                    $(stringify!($name) => Ok(TickerSymbol::$name),)+
                    _ => Err(\"Invalid String\")
                }}
            }}
        }}

        impl TickerSymbol {{
            // type Err = &'static str;

            pub fn as_bytes(&self) -> &[u8] {{
                match &self {{
                    $(TickerSymbol::$name => stringify!($name).as_bytes(),)+
                    // _ => Err(\"Invalid String\")
                }}
            }}
        }}
        
    }};
}}

macro_rules! generate_accounts_enum {{
    ([$($name:ident),*]) => {{
        #[derive(Debug, Copy, Clone, Deserialize, Serialize, EnumIter)]
        pub enum TraderId {{
            $($name, )*
        }}
        impl TryFrom<&'static str> for TraderId {{
            type Error = &'static str;

            fn try_from(s: &'static str) -> Result<TraderId, &'static str> {{
                match s {{
                    $(stringify!($name) => Ok(TraderId::$name),)+
                    _ => Err(\"Invalid String\")
                }}
            }}            
        }}    

        impl FromStr for TraderId {{
            type Err = &'static str;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {{
                match s {{
                    $(stringify!($name) => Ok(TraderId::$name),)+
                    _ => Err(\"Invalid String\")
                }}
            }}
        }}

        impl TraderId {{
            // type Err = &'static str;

            pub fn as_bytes(&self) -> &[u8] {{
                match &self {{
                    $(TraderId::$name => stringify!($name).as_bytes(),)+
                    // _ => Err(\"Invalid String\")
                }}
            }}
        }}   
    }};
}}

macro_rules! generate_account_balances_struct {{
    ([$($name:ident),*]) => {{
        #[derive(Debug)]
        pub struct AssetBalances {{
            $($name: Mutex<usize>, )*
        }}    

        impl AssetBalances {{
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &Mutex<usize>{{
                match symbol {{
                    $(TickerSymbol::$name => {{&self.$name}}, )*
                }}
            }}     
            
            pub fn new() -> Self {{
                Self {{ 
                    $($name: Mutex::new(0), )*
                 }}
            }}
               
        }}
    }};
}}

macro_rules! generate_global_state {{
    ([$($name:ident),*], [$($account_id:ident),*]) => {{
        #[derive(Debug, Serialize)]
        pub struct GlobalOrderBookState {{
            $(pub $name: Mutex<crate::orderbook::OrderBook>, )*
        }}
        
        impl GlobalOrderBookState {{
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &Mutex<crate::orderbook::OrderBook>{{
                match symbol {{
                    $(TickerSymbol::$name => {{&self.$name}}, )*
                }}
            }}
        }}
        
        pub struct GlobalAccountState {{
            $(pub $account_id: Mutex<crate::accounts::TraderAccount>, )*
        }}

        impl GlobalAccountState {{
            pub fn index_ref (&self, account_id:crate::config::TraderId,) -> &Mutex<crate::accounts::TraderAccount>{{
                match account_id {{
                    $(TraderId::$account_id => {{&self.$account_id}}, )*
                }}
            }}       
                    
        }}
    }};
}}  
generate_enum!({});
generate_account_balances_struct!({});
generate_global_state!({}, {});
generate_accounts_enum!({});

",
        symbols.trim(), symbols.trim(), symbols.trim(), account_ids.trim(),  account_ids.trim(),
    );
    let bytes = Bytes::from(content.trim());
    f.write_all(&bytes).unwrap();
}
