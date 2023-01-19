use core::fmt::Debug;
        use serde::{Deserialize, Serialize};
        macro_rules! generate_enum {
            ([$($name:ident),*]) => {
                #[derive(Debug, Copy, Clone, Deserialize, Serialize)]
                pub enum TickerSymbol {
                    $($name, )*
                }
            };
        }
        
        macro_rules! generate_account_balances_struct {
            ([$($name:ident),*]) => {
                #[derive(Debug)]
                pub struct AssetBalances {
                    $($name: usize, )*
                }
            };
        }
        
    generate_enum!([
        AAPL,
        JNJ
    ]);