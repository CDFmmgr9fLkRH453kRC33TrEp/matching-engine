use crate::accounts;
use crate::macro_calls;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use std::cmp;
use std::fmt;
// use std::fmt::Display;
use uuid::Uuid;
pub type OrderID = uuid::Uuid;
pub type Price = usize;
pub type TraderId = macro_calls::TraderId;
// for loading csv test files
// use std::env;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process::{self};
// use serde_json::Serialize;

// Logging
extern crate env_logger;
use log::{debug, error, info, trace, warn};

// Importing csv
type Record = (String, String, Option<u64>, f64, f64);

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}
// #[derive(Debug, Copy, Clone, Deserialize, Serialize, EnumString, EnumVariantNames)]
// pub enum macro_calls::TickerSymbol {
//     AAPL,
//     JNJ,
// }

// // for testing, remove later
// impl fmt::Display for macro_calls::TickerSymbol {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             macro_calls::TickerSymbol::AAPL => write!(f, "AAPL"),
//         }
//     }
// }

#[derive(Debug)]
pub struct OrderBook {
    /// Struct representing a double sided order book for a single product.
    // todo: add offset to allow for non 0 min prices
    pub symbol: macro_calls::TickerSymbol,
    // buy side in increasing price order
    buy_side_limit_levels: Vec<LimitLevel>,
    // sell side in increasing price order
    sell_side_limit_levels: Vec<LimitLevel>,
    current_high_buy_price: Price,
    current_low_sell_price: Price,
}


#[derive(Debug)]
struct LimitLevel {
    /// Struct representing one price level in the orderbook, containing a vector of Orders at this price
    price: Price,
    orders: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderRequest {
    /// Struct representing an incoming request which has not yet been added to the orderbook
    pub amount: usize,
    pub price: Price,
    pub order_type: OrderType,
    pub trader_id: TraderId,
    pub symbol: macro_calls::TickerSymbol,
}

#[derive(Debug)]
pub struct Order {
    /// Struct representing an existing order in the order book
    pub order_id: Uuid,
    pub trader_id: TraderId,
    pub symbol: macro_calls::TickerSymbol,
    pub amount: usize,
    pub price: Price,
    pub order_type: OrderType,
}
#[derive(Debug, Copy, Clone)]
struct Trader {
    id: TraderId,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelRequest {
    pub order_id: OrderID,
    price: Price,
    pub symbol: macro_calls::TickerSymbol,
    side: OrderType,
}

#[derive(Debug)]
struct Fill {
    /// Struct representing an order fill event, used to update credit limits, communicate orderbook status etc.
    sell_trader_id: TraderId,
    buy_trader_id: TraderId,
    amount: usize,
    price: Price,
    symbol: macro_calls::TickerSymbol,
    trade_time: u8,
}
impl OrderBook {
    fn add_order_to_book(&mut self, new_order_request: OrderRequest) -> Uuid {
        debug!("add_order_to_book trigger");
        let order_id = Uuid::new_v4();
        let new_order = Order {
            order_id: order_id,
            trader_id: new_order_request.trader_id,
            symbol: new_order_request.symbol,
            amount: new_order_request.amount,
            price: new_order_request.price,
            order_type: new_order_request.order_type,
        };
        match new_order.order_type {
            OrderType::Buy => {
                if self.current_high_buy_price < new_order.price {
                    self.current_high_buy_price = new_order.price;
                };
                self.buy_side_limit_levels[new_order.price]
                    .orders
                    .push(new_order);
            }
            OrderType::Sell => {
                if self.current_low_sell_price > new_order.price {
                    self.current_low_sell_price = new_order.price;
                };
                self.sell_side_limit_levels[new_order.price]
                    .orders
                    .push(new_order);
            }
        }
        order_id
    }

    pub fn handle_incoming_cancel_request(
        &mut self,
        cancel_request: CancelRequest,
    ) -> Option<Order> {
        debug!("remove_order_by_uuid trigger");
        match cancel_request.side {
            OrderType::Buy => {
                let mut index = 0;
                while index
                    < self.buy_side_limit_levels[cancel_request.price]
                        .orders
                        .len()
                {
                    if self.buy_side_limit_levels[cancel_request.price].orders[index].order_id
                        == cancel_request.order_id
                    {
                        return Some(
                            self.buy_side_limit_levels[cancel_request.price]
                                .orders
                                .remove(index),
                        );
                    }
                    index += 1;
                }
            }
            OrderType::Sell => {
                let mut index = 0;
                while index
                    < self.sell_side_limit_levels[cancel_request.price]
                        .orders
                        .len()
                {
                    if self.sell_side_limit_levels[cancel_request.price].orders[index].order_id
                        == cancel_request.order_id
                    {
                        return Some(
                            self.sell_side_limit_levels[cancel_request.price]
                                .orders
                                .remove(index),
                        );
                    }
                    index += 1;
                }
            }
        }
        None
    }

    pub fn handle_incoming_order_request(
        &mut self,
        new_order_request: OrderRequest,
        accounts_data: web::Data<macro_calls::GlobalAccountState>
    ) -> Option<Uuid> {
        match new_order_request.order_type {
            OrderType::Buy => self.handle_incoming_buy(new_order_request, accounts_data),
            OrderType::Sell => self.handle_incoming_sell(new_order_request, accounts_data),
        }
    }
    fn handle_incoming_sell(&mut self, mut sell_order: OrderRequest, accounts_data: web::Data<macro_calls::GlobalAccountState>) -> Option<Uuid> {
        debug!(
            "Incoming sell, current high buy {:?}",
            self.current_high_buy_price
        );
        if sell_order.price <= self.current_high_buy_price {
            // println!("Cross");
            // println!("amount to be filled remaining: {:?}", sell_order.amount);
            let mut current_price_level = self.current_high_buy_price;
            while (sell_order.amount > 0) & (current_price_level >= sell_order.price) {
                // println!("amount to be filled remaining: {:?}", sell_order.amount);
                // println!("current price level: {:?}", self.buy_side_limit_levels
                // [current_price_level].price);
                // self.print_book_state();
                // println!("current price level orders: {:?}", self.buy_side_limit_levels[current_price_level].orders);
                // let mut order_index = 0;
                while (self.buy_side_limit_levels[current_price_level].orders.len() > 0)
                    & (sell_order.amount > 0)
                {
                    let trade_price =
                        self.buy_side_limit_levels[current_price_level].orders[0].price;
                    let buy_trader_id =
                        self.buy_side_limit_levels[current_price_level].orders[0].trader_id;

                    let amount_to_be_traded = cmp::min(
                        sell_order.amount,
                        self.buy_side_limit_levels[current_price_level].orders[0].amount,
                    );
                    
                    let buy_trader = accounts_data.index_ref(buy_trader_id);
                    let sell_trader = accounts_data.index_ref(sell_order.trader_id);
                    self.handle_fill_event(buy_trader, sell_trader, Fill {
                        sell_trader_id: sell_order.trader_id,
                        buy_trader_id: buy_trader_id,
                        symbol: self.symbol,
                        amount: amount_to_be_traded,
                        price: trade_price,
                        trade_time: 1,
                    });

                    sell_order.amount -= amount_to_be_traded;
                    self.buy_side_limit_levels[current_price_level].orders[0].amount -=
                        amount_to_be_traded;
                    debug!(
                        "orders: {:?}",
                        self.sell_side_limit_levels[current_price_level].orders
                    );
                    debug!("limit level: {:?}", current_price_level);
                    if self.buy_side_limit_levels[current_price_level].orders[0].amount == 0 {
                        self.buy_side_limit_levels[current_price_level]
                            .orders
                            .remove(0);
                    }
                    // order_index += 1;
                }
                current_price_level -= 1;
            }
            // To do: find a more elegant way to avoid "skipping" price levels on the way down.
            current_price_level += 1;

            while current_price_level > 0 {
                if self.buy_side_limit_levels[current_price_level].orders.len() > 0 {
                    self.current_high_buy_price = current_price_level;
                    break;
                }
                current_price_level -= 1;
            }
            self.current_high_buy_price = current_price_level;
        }

        if sell_order.amount > 0 {
            return Some(self.add_order_to_book(sell_order));
        } else {
            return None;
        }
    }
    fn handle_incoming_buy(&mut self, mut buy_order: OrderRequest, accounts_data: web::Data<macro_calls::GlobalAccountState>) -> Option<Uuid> {
        debug!(
            "Incoming Buy, current low sell {:?}",
            self.current_low_sell_price
        );
        if buy_order.price >= self.current_low_sell_price {
            let mut current_price_level = self.current_low_sell_price;
            while (buy_order.amount > 0) & (current_price_level <= buy_order.price) {
                // let mut order_index = 0;
                while (0 < self.sell_side_limit_levels[current_price_level]
                    .orders
                    .len())
                    & (buy_order.amount > 0)
                {
                    debug!("remain to fill {:?}", buy_order.amount);
                    debug!(
                        "{:?}",
                        self.sell_side_limit_levels[current_price_level].orders
                    );
                    let trade_price =
                        self.sell_side_limit_levels[current_price_level].orders[0].price;
                    let sell_trader_id =
                        self.sell_side_limit_levels[current_price_level].orders[0].trader_id;

                    let amount_to_be_traded = cmp::min(
                        buy_order.amount,
                        self.sell_side_limit_levels[current_price_level].orders[0].amount,
                    );
                    // could turn this into an associated function which does not need a reference to the orderbook, but I think its fine.
                    let buy_trader = accounts_data.index_ref(buy_order.trader_id);
                    let sell_trader = accounts_data.index_ref(sell_trader_id);
                    self.handle_fill_event(buy_trader, sell_trader, Fill {
                        buy_trader_id: buy_order.trader_id,
                        sell_trader_id: sell_trader_id,
                        symbol: self.symbol,
                        amount: amount_to_be_traded,
                        price: trade_price,
                        trade_time: 1,
                    });

                    // TODO: create "sell" function that can handle calls to allocate credit etc.
                    // also removing from the front seems pretty inefficient,
                    buy_order.amount -= amount_to_be_traded;
                    self.sell_side_limit_levels[current_price_level].orders[0].amount -=
                        amount_to_be_traded;
                    if self.sell_side_limit_levels[current_price_level].orders[0].amount == 0 {
                        self.sell_side_limit_levels[current_price_level]
                            .orders
                            .remove(0);
                    }
                    // order_index += 1;
                }
                current_price_level += 1;
            }
            current_price_level -= 1;
            // in the event that a price level has been completely bought, update lowest sell price
            while current_price_level < self.sell_side_limit_levels.len() {
                if self.sell_side_limit_levels[current_price_level]
                    .orders
                    .len()
                    > 0
                {
                    self.current_low_sell_price = current_price_level;
                    break;
                }
                current_price_level += 1;
            }
            self.current_low_sell_price = current_price_level;
        }
        if buy_order.amount > 0 {
            return Some(self.add_order_to_book(buy_order));
        } else {
            return None;
        }
    }

    fn handle_fill_event(&self, buy_trader: &Mutex<accounts::TraderAccount>, sell_trader: &Mutex<accounts::TraderAccount>, fill_event: Fill) {
        let cent_value = &fill_event.amount * &fill_event.price;            
        
        *buy_trader.lock().unwrap().asset_balances.index_ref(&fill_event.symbol).lock().unwrap() -= fill_event.amount;
        buy_trader.lock().unwrap().cents_balance += cent_value;
        
        *sell_trader.lock().unwrap().asset_balances.index_ref(&fill_event.symbol).lock().unwrap() += fill_event.amount;
        sell_trader.lock().unwrap().cents_balance -= cent_value;

        debug!(
            "{:?} sells to {:?}: {:?} lots of {:?} @ ${:?}",
            fill_event.sell_trader_id,
            fill_event.buy_trader_id,
            fill_event.amount,
            fill_event.symbol,
            fill_event.price
        );
    }

    pub fn print_book_state(&self) {
        for price_level_index in 0..self.buy_side_limit_levels.len() {
            let mut outstanding_sell_orders: usize = 0;
            let mut outstanding_buy_orders: usize = 0;
            for order in self.sell_side_limit_levels[price_level_index].orders.iter() {
                outstanding_sell_orders += order.amount;
            }
            for order in self.buy_side_limit_levels[price_level_index].orders.iter() {
                outstanding_buy_orders += order.amount;
            }
            let mut string_out = String::from("");
            for _ in 0..outstanding_sell_orders {
                string_out = string_out + "S"
            }
            for _ in 0..outstanding_buy_orders {
                string_out = string_out + "B"
            }
            println!(
                "${}: {}",
                self.buy_side_limit_levels[price_level_index].price, string_out
            );
            // book_ouput[price_level_index] = outstanding_orders;
        }
    }

    // pub fn load_csv_test_data(&mut self, file_path: OsString) -> Result<(), Box<dyn Error>> {
    //     info!("Loading order data from {:?}", file_path);
    //     let file = File::open(file_path)?;
    //     let mut rdr = csv::Reader::from_reader(file);
    //     for result in rdr.deserialize() {
    //         let new_order_request: OrderRequest = result?;
    //         self.handle_incoming_order_request(new_order_request);
    //     }
    //     Ok(())
    // }
}

pub fn quickstart_order_book(
    symbol: macro_calls::TickerSymbol,
    min_price: Price,
    max_price: Price,
) -> OrderBook {
    OrderBook {
        symbol: macro_calls::TickerSymbol::from(symbol),
        buy_side_limit_levels: (min_price..max_price)
            .map(|x| LimitLevel {
                price: x,
                orders: Vec::new(),
            })
            .collect(),
        sell_side_limit_levels: (min_price..max_price)
            .map(|x| LimitLevel {
                price: x,
                orders: Vec::new(),
            })
            .collect(),
        current_high_buy_price: min_price,
        current_low_sell_price: max_price,
    }
}

fn main() {
    let mut order_book = quickstart_order_book(macro_calls::TickerSymbol::AAPL, 0, 11);
    // if let Err(err) = order_book.load_csv_test_data("src/test_orders.csv".into()) {
    //     println!("{}", err);
    //     process::exit(1);
    // }
    order_book.print_book_state();
    // println!("{:#?}", order_book);
    // let o_req = OrderRequest{
    //     amount: 10,
    //     price: 10,
    //     order_type: OrderType::Buy,
    //     trader_id: 1,
    //     symbol: macro_calls::TickerSymbol::AAPL,
    // };
    // println!("Hello");
    // println!("{:?}", serde_json::to_string(&o_req));
    // Example request
    // curl localhost:3000 -XPOST -H "Content-Type: application/json" -d "{\"Amount\":10,\"Price\":1,\"OrderType\":\"Buy\",\"TraderId\":1,\"Symbol\":\"AAPL\"}"
}
