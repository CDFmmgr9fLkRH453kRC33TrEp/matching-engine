// build.rs
use bytes::Bytes;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    // let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "/Users/caidan/projects/orderbook/src";
    let dest_path = Path::new(&out_dir).join("macro_calls.rs");
    let mut f = File::create(&dest_path).unwrap();
    let content = format!(
        "
use core::fmt::Debug;
use serde::{{Deserialize, Serialize}};
use std::sync::Mutex;
use uuid::Uuid;
use crate::accounts::TraderAccount;
use crate::orderbook::OrderBook;

macro_rules! generate_enum {{
    ([$($name:ident),*]) => {{
        #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
        pub enum TickerSymbol {{
            $($name, )*
        }}
    }};
}}

macro_rules! generate_account_balances_struct {{
    ([$($name:ident),*]) => {{
        #[derive(Debug)]
        pub struct AssetBalances {{
            $($name: usize, )*
        }}
        impl AssetBalances {{
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &usize{{
                match symbol {{
                    $($name => {{&self.$name}}, )*
                }}
            }}
            pub fn index_ref_mut (&mut self, symbol:&TickerSymbol) -> usize{{
                match symbol {{
                    $($name => {{self.$name}}, )*
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
                    $($name => {{&self.$name}}, )*
                }}
            }}
        }}
        
        pub struct GlobalAccountState {{
            $($account_id: crate::accounts::TraderAccount, )*
        }}

        impl GlobalAccountState {{
            pub fn index_ref (&self, account_id:uuid::Uuid) -> &crate::accounts::TraderAccount{{
                match account_id {{
                    $($account_id => {{&self.$account_id}}, )*
                }}
            }}
            pub fn index_ref_mut (&mut self, account_id:uuid::Uuid) -> &mut crate::accounts::TraderAccount{{
                match account_id {{
                    $($account_id => {{&mut self.$account_id}}, )*
                }}
            }}
        }}
    }};
}}
        
generate_enum!({});
generate_account_balances_struct!({});
generate_global_state!({}, {});",
        symbols.trim(), symbols.trim(), symbols.trim(), account_ids.trim()
    );
    let bytes = Bytes::from(content.trim());
    f.write_all(&bytes).unwrap();
}
