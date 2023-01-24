use uuid::Uuid;
use crate::orderbook;
use crate::macro_calls;
use macro_calls::AssetBalances;

pub struct TraderAccount {
    pub trader_id: macro_calls::TraderId,
    pub cents_balance: usize,
    pub total_outstanding_cents_balance: usize,
    // asset_balances, outstanding_order_balances updated on fill event, and so should be current
    // in asset lots
    pub asset_balances: macro_calls::AssetBalances,
    // in cents
    pub outstanding_order_balances: macro_calls::AssetBalances,
}

pub fn quickstart_trader_account (trader_id: macro_calls::TraderId, cents_balance: usize) -> TraderAccount {
    TraderAccount {
        trader_id: trader_id,
        cents_balance: cents_balance,
        total_outstanding_cents_balance: 0,
        // asset_balances, outstanding_order_balances updated on fill event, and so should be current
        // in asset lots
        asset_balances: macro_calls::AssetBalances::new(),
        // in cents
        outstanding_order_balances: macro_calls::AssetBalances::new(),
    }
}


// impl TraderAccount {
//     pub fn asset_balance_geq (&self, symbol: macro_calls::TickerSymbol, amount: &usize) -> bool {
//         // check number of currently owned assets of a given symbol (i.e. to ensure no shorts)
//         if self.asset_balances.index_ref(symbol) >= amount {
//             return true;
//         } else {
//             return false;
//         }        
//     }
//     pub fn outstanding_order_balance_geq (&self, symbol: macro_calls::TickerSymbol, amount: &usize) -> bool {
//         // check number of currently owned assets of a given symbol (i.e. to ensure no shorts)
//         if self.outstanding_order_balances.index_ref(symbol) >= amount {
//             return true;
//         } else {
//             return false;
//         }        
//     }
// }
