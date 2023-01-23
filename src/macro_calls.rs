use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use crate::accounts::TraderAccount;
use crate::orderbook::OrderBook;

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
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &usize{
                match symbol {
                    $($name => {&self.$name}, )*
                }
            }
            pub fn index_ref_mut (&mut self, symbol:&TickerSymbol) -> usize{
                match symbol {
                    $($name => {self.$name}, )*
                }
            }
        }
    };
}

macro_rules! generate_global_state {
    ([$($name:ident),*], [$($account_id:ident),*]) => {
        #[derive(Debug)]
        pub struct GlobalOrderBookState {
            $(pub $name: Mutex<crate::orderbook::OrderBook>, )*
        }
        
        impl GlobalOrderBookState {
            pub fn index_ref (&self, symbol:&TickerSymbol) -> &Mutex<crate::orderbook::OrderBook>{
                match symbol {
                    $($name => {&self.$name}, )*
                }
            }
        }
        
        pub struct GlobalAccountState {
            $($account_id: crate::accounts::TraderAccount, )*
        }

        impl GlobalAccountState {
            pub fn index_ref (&self, account_id:uuid::Uuid) -> &crate::accounts::TraderAccount{
                match account_id {
                    $($account_id => {&self.$account_id}, )*
                }
            }
            pub fn index_ref_mut (&mut self, account_id:uuid::Uuid) -> &mut crate::accounts::TraderAccount{
                match account_id {
                    $($account_id => {&mut self.$account_id}, )*
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
        Columbia_B
    ]);