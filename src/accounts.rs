use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use crate::orderbook;
use crate::config;
use crate::websockets;
use config::AssetBalances;
use actix::Addr;
use queues::IsQueue;
use std::sync::Arc;

use queues;

pub type Password = [char; 4];
#[derive(Debug, Serialize, Deserialize)]
pub struct TraderAccount {
    pub trader_id: config::TraderId,
    pub cents_balance: usize,
    // pub trader_ip: config::TraderIp,
    #[serde(skip, default="ret_none")]
    pub current_actor: Option<Addr<websockets::MyWebSocketActor>>,
    pub password: Password,
    // pub websocket actor: actix addr
    // pub fill_event_queue: fifo queue
    // pub fn send_message {
    //  try to send to websocket
    //  if no connection, add to end of queue
    // }
    // pub fn register connection {
    //  on connection, make sure xno other connections exist
    //  register actix actor and update addr
    //  send out all messages in fill event queue
    // }
    // in cents, equal to total of owned cents minus total value of outstanding buy orders

    // consider changing to Buffer instead of Queue to know size
    #[serde(skip)]
    #[serde(default = "empty_message_queue")]
    pub message_backup: queues::Queue<Arc<orderbook::Fill>>,

    pub net_cents_balance: usize,
    // asset_balances, net_asset_balances updated on fill event, and so should be current
    // in asset lots
    pub asset_balances: config::AssetBalances,
    // in shares, equal to the total of owned shares minus the total of outstanding sell orders' shares (i.e. should be \geq 0)
    pub net_asset_balances: config::AssetBalances,
}

fn ret_none() -> Option<Addr<websockets::MyWebSocketActor>> {
    None
}


fn empty_message_queue() -> queues::Queue<Arc<orderbook::Fill>> {
    queues::Queue::new()
}


impl TraderAccount {
    pub fn push_fill(&mut self, fill_event: Arc<orderbook::Fill>) {
        // maybe spawn async thread?
        match &self.current_actor {
            None => {
                self.message_backup.add(fill_event).unwrap();
            },
            Some(addr) =>{
                // todo: slow clone, switch paths?
                // todo: switch to cloning RC so not so expensive.
                // should only be unwrapped when sent. 
                match addr.try_send(fill_event.clone()) {
                    Ok(_) => {()},
                    Err(E) => {self.message_backup.add(fill_event).unwrap(); ()}
                }
            },
        }
    }
}

pub fn quickstart_trader_account (trader_id: config::TraderId, cents_balance: usize, password: Password) -> TraderAccount {
    TraderAccount {        
        trader_id: trader_id,
        // trader_ip: trader_ip,
        cents_balance: cents_balance,
        net_cents_balance: cents_balance,
        message_backup: queues::Queue::<Arc<orderbook::Fill>>::new(),
        // asset_balances, net_asset_balances updated on fill event, and so should be current
        // in asset lots
        asset_balances: config::AssetBalances::new(),
        // in cents
        net_asset_balances: config::AssetBalances::new(),
        current_actor: None,
        password: password
    }
}