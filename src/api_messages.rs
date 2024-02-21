use serde::Deserialize;
use serde::Serialize;

use crate::macro_calls;
use crate::orderbook::OrderID;
use crate::orderbook::OrderType;
use crate::orderbook::Price;
use crate::CancelRequest;
use crate::OrderRequest;
use actix_web::{error, Result};
use derive_more::{Display, Error};

// Client -> Server Messages
#[derive(Serialize, Deserialize)]
#[serde(tag = "MessageType")]
enum IncomingMessage {
    OrderRequest(OrderRequest),
    CancelRequest(CancelRequest),
}

// Server -> Client Messages
pub struct CancelErrorMessage {
    order_id: OrderID,
    side: OrderType,
    price: Price,
    symbol: macro_calls::TickerSymbol,
}

pub enum ErrorMessage {
    CancelErrorMessage,
    OrderPlaceErrorMessage,
}

pub enum ServerMessage {
    ErrorMessage,
}