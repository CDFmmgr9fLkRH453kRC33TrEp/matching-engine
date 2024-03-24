use serde::Deserialize;
use serde::Serialize;

use crate::accounts;
use crate::config;
use crate::config::TraderId;
use crate::orderbook::Order;
use crate::orderbook::OrderID;
use crate::orderbook::OrderType;
use crate::orderbook::Price;
use actix_web::{error, Result};
use derive_more::{Display, Error};

// Client -> Server Messages
#[derive(Serialize, Deserialize)]
#[serde(tag = "MessageType")]
enum IncomingMessage {
    OrderRequest(OrderRequest),
    CancelRequest(CancelRequest),
}
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct OrderRequest {
    /// Struct representing an incoming request which has not yet been added to the orderbook
    pub amount: usize,
    pub price: Price,
    pub order_type: OrderType,
    pub trader_id: TraderId,
    pub symbol: config::TickerSymbol,
    pub password: accounts::Password,
}


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelRequest {
    pub order_id: OrderID,
    pub trader_id: TraderId,
    pub price: Price,
    pub symbol: config::TickerSymbol,
    pub side: OrderType,
    pub password: accounts::Password,
}

// Server -> Client Messages
// should all impl error::ResponseError to play nice with Actix

// private server -> client
#[derive(Debug, Serialize)]
pub struct OrderConfimMessage {
    /// sent to trader when their order is added to the orderbook
    pub order_info: Order,
}

#[derive(Debug, Serialize)]
pub struct CancelConfimMessage {
    /// sent to trader when their order is removed from the orderbook due to cancel message
    pub order_info: Order
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct OrderFillMessage {
    /// sent to buyers/sellers of contract on execution
    pub order_id: OrderID,
    pub amount_filled: usize,
}

#[derive(Debug, Serialize)]
pub struct CancelErrorMessage {
    /// sent to trader if cancelling order results in error
    pub order_id: OrderID,
    pub side: OrderType,
    pub price: Price,
    pub symbol: config::TickerSymbol,
    pub error_details: str
}

#[derive(Debug, Serialize)]
pub struct OrderPlaceErrorMessage {
    /// sent to trader if adding order results in error
    pub order_id: OrderID,
    pub side: OrderType,
    pub price: Price,
    pub symbol: config::TickerSymbol,
    pub error_details: str
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
// public server -> client
pub struct TradeOccurredMessage {
    /// sent to all traders' mailboxes when a trade occurs
    // true if resting order is completely filled and removed from book
    pub order_dead: bool,
    pub amount: usize,
    pub symbol: config::TickerSymbol,
    // price at which trade occurred (should be resting order's price)
    pub price: Price
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct CancelOccurredMessage {
    /// sent to all traders' mailboxes when a cancel occurs
    pub side: OrderType,
    pub amount: usize,
    pub symbol: config::TickerSymbol,
    pub price: Price,
}

pub enum ServerMessage {
    ErrorMessage,
}