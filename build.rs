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
        Columbia_B
    ]
    ";
    // todo: make this load from csv?
    let account_ips = "
    [
        \"172.16.123.1\".parse::<Ipv4Addr>().unwrap(),
        \"172.16.123.2\".parse::<Ipv4Addr>().unwrap(),
    ]
    ";
    // let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "/Users/caidan/projects/exchange_simulator/matching-engine/src";
    let dest_path = Path::new(&out_dir).join("macro_calls.rs");
    let mut f = File::create(&dest_path).unwrap();
    let content = format!(
        "
use std::collections::HashMap;
use std::net::Ipv4Addr;
pub type TraderIp = std::net::Ipv4Addr;
use std::io;


// TODO: clean up this mess!!!!!
pub fn ip_to_id (ip: Ipv4Addr) -> Result<crate::macro_calls::TraderId, io::Error> {{
    if (ip == Ipv4Addr::new(172,16,123,1)) {{
        return Ok(crate::macro_calls::TraderId::Columbia_A);
    }} else if (ip == Ipv4Addr::new(172,16,123,2)){{
        return Ok(crate::macro_calls::TraderId::Columbia_B);
    }} else {{
        panic!(\"not a known ip\");
    }}
}}

use core::fmt::Debug;
use serde::{{Deserialize, Serialize}};
use std::sync::Mutex;
use uuid::Uuid;
use crate::accounts::TraderAccount;
use crate::orderbook::OrderBook;
// use crate::orderbook::TraderId;
// hello

macro_rules! generate_enum {{
    ([$($name:ident),*]) => {{
        #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
        pub enum TickerSymbol {{
            $($name, )*
        }}
    }};
}}

macro_rules! generate_accounts_enum {{
    ([$($name:ident),*]) => {{
        #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
        pub enum TraderId {{
            $($name, )*
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
        #[derive(Debug)]
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
            pub fn index_ref (&self, account_id:crate::macro_calls::TraderId,) -> &Mutex<crate::accounts::TraderAccount>{{
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
