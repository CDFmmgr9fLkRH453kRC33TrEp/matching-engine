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
    // let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "/Users/caidan/projects/orderbook/src";
    let dest_path = Path::new(&out_dir).join("macro_calls.rs");
    let mut f = File::create(&dest_path).unwrap();
    let content = format!(
        "
        use core::fmt::Debug;
        use serde::{{Deserialize, Serialize}};
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
                    pub fn index_ref (&self, symbol:TickerSymbol) -> &usize{{
                        match symbol {{
                            $($name => {{&self.$name}}, )*
                        }}
                    }}
                    pub fn index_ref_mut (&mut self, symbol:TickerSymbol) -> usize{{
                        match symbol {{
                            $($name => {{self.$name}}, )*
                        }}
                    }}
                }}
            }};
        }}
        
    generate_enum!({});
    generate_account_balances_struct!({});",
        symbols.trim(), symbols.trim()
    );
    let bytes = Bytes::from(content.trim());
    f.write_all(&bytes).unwrap();
}
