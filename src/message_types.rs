/// Defines all main message types for internal actor communication
use actix::*;
use std::sync::Arc;

use crate::{config::TraderIp, orderbook::{TraderId, LimLevUpdate}};

#[derive(Message)]
#[rtype(result = "()")]
pub struct OpenMessage{
    pub ip: TraderIp,
    pub addr: Recipient<Arc<LimLevUpdate>>
}