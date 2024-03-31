use actix::Message;
use serde::Deserialize;
use serde::Serialize;
use core::fmt;
use crate::accounts;
use crate::config;
use crate::config::TraderId;
use crate::orderbook::Order;
use crate::orderbook::OrderID;
use crate::orderbook::OrderType;
use crate::orderbook::Price;
use derive_more::Error;

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
#[derive(Debug, Serialize, Clone, Message, Copy)]
#[rtype(result = "()")]
pub struct OrderConfirmMessage {
    /// sent to trader when their order is added to the orderbook
    pub order_info: Order,
}

#[derive(Debug, Serialize, Message, Clone, Copy)]
#[rtype(result = "()")]
pub struct CancelConfirmMessage {
    /// sent to trader when their order is removed from the orderbook due to cancel message
    pub order_info: Order
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Message)]
#[rtype(result = "()")]
pub struct OrderFillMessage {
    /// sent to buyers/sellers of contract on execution
    pub order_id: OrderID,
    pub amount_filled: usize,
}

#[derive(Debug, Serialize, Message, Clone, Copy)]
#[rtype(result = "()")]
pub struct CancelErrorMessage <'a>{
    /// sent to trader if cancelling order results in error
    pub order_id: OrderID,
    pub side: OrderType,
    pub price: Price,
    pub symbol: config::TickerSymbol,
    pub error_details: &'a str
}

#[derive(Debug, Serialize, Message, Clone, Copy)]
#[rtype(result = "()")]
pub struct OrderPlaceErrorMessage <'a> {
    /// sent to trader if adding order results in error
    pub side: OrderType,
    pub price: Price,
    pub symbol: config::TickerSymbol,
    pub error_details: &'a str
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Message)]
#[rtype(result = "()")]
// public server -> clients
pub struct TradeOccurredMessage {
    /// sent to all traders' mailboxes when a trade occurs
    // should it ignore the buyer/seller who already got a message about the trade? -> no, this should be handled client side
    pub amount: usize,
    pub symbol: config::TickerSymbol,
    // price at which trade occurred (should be resting order's price)
    pub price: Price
}

#[derive(Debug, Serialize, Message, Clone, Copy)]
#[rtype(result = "()")]
pub struct NewRestingOrderMessage {
    // sent to all traders to communicate that there has been a new order which now rests on the book
    // replaces party of LimLevUpdate
    pub side: OrderType,
    pub amount: usize,
    pub symbol: config::TickerSymbol,
    pub price: Price,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Message)]
#[rtype(result = "()")]
pub struct CancelOccurredMessage {
    /// sent to all traders' mailboxes when a cancel occurs
    pub side: OrderType,
    pub amount: usize,
    pub symbol: config::TickerSymbol,
    pub price: Price,
}

#[derive(Debug, Serialize, Clone)]
pub enum OrderPlaceResponse <'a> {
    OrderPlaceErrorMessage(OrderPlaceErrorMessage<'a>),
    OrderConfirmMessage(OrderConfirmMessage)
}

#[derive(Debug, Serialize, Clone)]
pub enum OrderCancelResponse <'a> {
    CancelConfirmMessage(CancelConfirmMessage),
    CancelErrorMessage(CancelErrorMessage<'a>)
}

#[derive(Message, Clone, Serialize)]
#[rtype(result = "()")]
pub enum OutgoingMessage{
    // To make implementing default Handler for actors easier
    TradeOccurredMessage(TradeOccurredMessage),
    NewRestingOrderMessage(NewRestingOrderMessage),
    CancelOccurredMessage(CancelOccurredMessage),
}

#[derive(Debug, Error, Clone, Serialize)]
pub struct CancelIDNotFoundError;

impl fmt::Display for CancelIDNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "order_id not found at specified price/side")
    }
}