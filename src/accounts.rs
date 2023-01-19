use uuid::Uuid;
use crate::orderbook;
struct AssetBalances {
    AAPL: usize,
}

pub struct TraderAccount {
    trader_id: orderbook::TraderId,
    cents_balance: usize,
    // asset_balances, outstanding_order_balances updated on fill event, and so should be current
    // in asset lots
    asset_balances: AssetBalances,
    // in cents
    outstanding_order_balances: usize,
}

impl TraderAccount {
    pub fn check_asset_balance (&self, symbol: orderbook::TickerSymbol) -> &usize {
        self.asset_balances.index_ref(symbol)
        // check number of currently owned assets of a given symbol (i.e. to ensure no shorts)
    }
    pub fn check_outstanding_order_balance (&self, symbol: orderbook::TickerSymbol) -> &usize {
        &self.outstanding_order_balances
        // check total value of outstanding buy orders of given symbol (i.e. to ensure no buy orders placed with insufficient funds)
    }
}
// todo: write macro to avoid having to manually deal with new asset symbols
impl AssetBalances {
    fn index_ref(&self, symbol: orderbook::TickerSymbol) -> &usize {
        match symbol {
            orderbook::TickerSymbol::AAPL => &self.AAPL,
        }
    }
}