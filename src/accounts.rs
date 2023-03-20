use uuid::Uuid;
use crate::orderbook;
use crate::macro_calls;
use crate::websockets;
use macro_calls::AssetBalances;
use actix::Addr;
use queues::IsQueue;

use queues;
pub struct TraderAccount {
    pub trader_id: macro_calls::TraderId,
    pub cents_balance: usize,
    pub trader_ip: macro_calls::TraderIp,
    pub current_actor: Option<Addr<websockets::MyWebSocketActor>>,
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

    // consider changing to Buffer instead of Queue to know size
    pub message_backup: queues::Queue<orderbook::Fill>,

    pub net_cents_balance: usize,
    // asset_balances, net_asset_balances updated on fill event, and so should be current
    // in asset lots
    pub asset_balances: macro_calls::AssetBalances,
    // in shares, equal to the total of owned shares minus the total of outstanding sell orders' shares (i.e. should be \geq 0)
    pub net_asset_balances: macro_calls::AssetBalances,
}

impl TraderAccount {
    pub fn push_fill(&mut self, fill_event: orderbook::Fill) {
        // maybe spawn async thread?
        match &self.current_actor {
            None => {
                self.message_backup.add(fill_event).unwrap();
            },
            Some(addr) =>{
                addr.try_send(fill_event).unwrap();
            },
        }
    }
}

pub fn quickstart_trader_account (trader_id: macro_calls::TraderId, cents_balance: usize, trader_ip: macro_calls::TraderIp) -> TraderAccount {
    TraderAccount {        
        trader_id: trader_id,
        trader_ip: trader_ip,
        cents_balance: cents_balance,
        net_cents_balance: cents_balance,
        message_backup: queues::Queue::<orderbook::Fill>::new(),
        // asset_balances, net_asset_balances updated on fill event, and so should be current
        // in asset lots
        asset_balances: macro_calls::AssetBalances::new(),
        // in cents
        net_asset_balances: macro_calls::AssetBalances::new(),
        current_actor: None,
    }
}