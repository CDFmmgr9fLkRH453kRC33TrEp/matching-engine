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
                impl AssetBalances {
                    pub fn index_ref (&self, symbol:TickerSymbol) -> &usize{
                        match symbol {
                            $($name => {&self.$name}, )*
                        }
                    }
                    pub fn index_ref_mut (&mut self, symbol:TickerSymbol) -> usize{
                        match symbol {
                            $($name => {self.$name}, )*
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