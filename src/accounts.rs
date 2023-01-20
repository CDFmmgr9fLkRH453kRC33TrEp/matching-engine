use uuid::Uuid;
use crate::orderbook;
use crate::macro_calls;
use macro_calls::AssetBalances;

pub struct TraderAccount {
    trader_id: orderbook::TraderId,
    cents_balance: usize,
    // asset_balances, outstanding_order_balances updated on fill event, and so should be current
    // in asset lots
    asset_balances: macro_calls::AssetBalances,
    // in cents
    outstanding_order_balances: macro_calls::AssetBalances,
}

impl TraderAccount {
    pub fn check_asset_balance (&self, symbol: macro_calls::TickerSymbol) -> &usize {
        self.asset_balances.index_ref(symbol)
        // check number of currently owned assets of a given symbol (i.e. to ensure no shorts)
    }
    pub fn check_outstanding_order_balance (&mut self, symbol: macro_calls::TickerSymbol) -> usize {
        self.outstanding_order_balances.index_ref_mut(symbol)
        // check total value of outstanding buy orders of given symbol (i.e. to ensure no buy orders placed with insufficient funds)
    }
}
