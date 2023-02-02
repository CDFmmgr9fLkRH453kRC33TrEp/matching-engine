use uuid::Uuid;
use crate::orderbook;
use crate::macro_calls;
use macro_calls::AssetBalances;

pub struct TraderAccount {
    pub trader_id: macro_calls::TraderId,
    pub cents_balance: usize,
    // pub websocket actor: actix addr
    // pub fill_event_queue: fifo queue
    // pub fn send_message {
    //  try to send to websocket
    //  if no connection, add to end of queue
    // }
    // pub fn register connection {
    //  on connection, make sure no other connections exist
    //  register actix actor and update addr
    //  send out all messages in fill event queue
    // }
    // in cents, equal to total of owned cents minus total value of outstanding buy orders
    pub net_cents_balance: usize,
    // asset_balances, net_asset_balances updated on fill event, and so should be current
    // in asset lots
    pub asset_balances: macro_calls::AssetBalances,
    // in shares, equal to the total of owned shares minus the total of outstanding sell orders' shares (i.e. should be \geq 0)
    pub net_asset_balances: macro_calls::AssetBalances,
}

pub fn quickstart_trader_account (trader_id: macro_calls::TraderId, cents_balance: usize) -> TraderAccount {
    TraderAccount {
        trader_id: trader_id,
        cents_balance: cents_balance,
        net_cents_balance: cents_balance,
        // asset_balances, net_asset_balances updated on fill event, and so should be current
        // in asset lots
        asset_balances: macro_calls::AssetBalances::new(),
        // in cents
        net_asset_balances: macro_calls::AssetBalances::new(),
    }
}