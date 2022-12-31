use std::cmp;
use uuid::Uuid;
type Price = usize;
#[derive(Debug, Clone, Copy)]
enum OrderType {
    Buy,
    Sell,
}
#[derive(Debug, Copy, Clone)]
enum TickerSymbol {
    AAPL,
}

#[derive(Debug)]
struct OrderBook {
    symbol: TickerSymbol,
    // buy side in increasing price order
    buy_side_limit_levels: Vec<LimitLevel>,
    // sell side in increasing price order
    sell_side_limit_levels: Vec<LimitLevel>,
    current_high_buy_price: Price,
    current_low_sell_price: Price,
}
#[derive(Debug)]
struct LimitLevel {
    price: Price,
    orders: Vec<Order>,
}

#[derive(Debug)]
struct OrderRequest {
    trader: Trader,
    symbol: TickerSymbol,
    amount: u8,
    price: Price,
    order_type: OrderType,
}

#[derive(Debug)]
struct Order {
    order_id: Uuid,
    trader: Trader,
    symbol: TickerSymbol,
    amount: u8,
    price: Price,
    order_type: OrderType,
}
#[derive(Debug, Copy, Clone)]
struct Trader {
    id: [char; 4],
}

#[derive(Debug)]
struct TradeResult {
    sell_trader: Trader,
    buy_trader: Trader,
    amount: u8,
    price: Price,
    symbol: TickerSymbol,
    trade_time: u8,
}
impl OrderBook {
    fn add_order_to_book(&mut self, new_order_request: &OrderRequest) -> Uuid {
        let order_id = Uuid::new_v4();
        let new_order = Order {
            order_id: order_id,
            trader: new_order_request.trader,
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

    fn remove_order_by_uuid(
        &mut self,
        order_id: Uuid,
        price: usize,
        side: OrderType,
    ) -> Option<Order> {
        match side {
            OrderType::Buy => {
                let mut pointer = 0;
                while pointer < self.buy_side_limit_levels[price].orders.len() {
                    if self.buy_side_limit_levels[price].orders[pointer].order_id == order_id {
                        return Some(self.buy_side_limit_levels[price].orders.remove(pointer));
                    }
                    pointer += 1;
                }
            }
            OrderType::Sell => {
                let mut pointer = 0;
                while pointer < self.sell_side_limit_levels[price].orders.len() {
                    if self.sell_side_limit_levels[price].orders[pointer].order_id == order_id {
                        return Some(self.sell_side_limit_levels[price].orders.remove(pointer));
                    }
                    pointer += 1;
                }
            }
        }
        None
    }
    fn handle_incoming_sell(&mut self, sell_order: &mut OrderRequest) -> Option<Uuid> {
        if sell_order.price <= self.current_high_buy_price {
            println!("Cross, beginning matching");
            let mut current_price_level = self.current_high_buy_price;
            while (sell_order.amount > 0) & (current_price_level > sell_order.price){
                let mut order_index = 0;
                while order_index < self.buy_side_limit_levels[current_price_level].orders.len() {
                    let order_considering = &mut self.buy_side_limit_levels[current_price_level].orders[order_index];
                    let amount_to_be_traded = cmp:min(sell_order.amount, order_considering.amount);
                    println!("Match found, trading {:?} lots of {:?} @ ${:?}",amount_to_be_traded, sell_order.symbol, sell_order.price);
                }

            }
        }
        if sell_order.amount > 0 {
            return Some(self.add_order_to_book(sell_order));
        } else {
            return None;
        }

    }
    fn handle_incoming_buy(&mut self, buy_order: &mut OrderRequest) -> Option<Uuid> {
        if buy_order.price >= self.current_low_sell_price {
            println!("Cross, beginning matching");
            let mut current_price_level = self.current_low_sell_price;
            while (buy_order.amount > 0) & (current_price_level < buy_order.price) {
                let mut order_index = 0;
                while order_index
                    < self.sell_side_limit_levels[current_price_level]
                        .orders
                        .len()
                {
                    let order_considering =
                        &mut self.sell_side_limit_levels[current_price_level].orders[order_index];
                    let amount_to_be_traded = cmp::min(buy_order.amount, order_considering.amount);
                    println!(
                        "Match found, trading {:?} lots of {:?} @ ${:?}",
                        amount_to_be_traded, buy_order.symbol, buy_order.price
                    );
                    // TODO: create "sell" function that can handle calls to allocate credit etc.
                    buy_order.amount -= amount_to_be_traded;
                    order_considering.amount -= amount_to_be_traded;
                    if order_considering.amount == 0 {
                        self.sell_side_limit_levels[current_price_level]
                            .orders
                            .remove(order_index);
                    }
                    order_index += 1;
                }
                current_price_level += 1;
            }
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
        // println!(
        //     "{:?} buys {:?} shares of {:?} from {:?} at {:?}",
        //     buy_order.trader, buy_order.amount, symbol, sell_trader, price
        // );
    }
}

fn main() {
    let mut order_book = OrderBook {
        symbol: TickerSymbol::AAPL,
        buy_side_limit_levels: (0..10)
            .map(|x| LimitLevel {
                price: x,
                orders: Vec::new(),
            })
            .collect(),
        sell_side_limit_levels: (0..10)
            .map(|x| LimitLevel {
                price: x,
                orders: Vec::new(),
            })
            .collect(),
        current_high_buy_price: 0,
        current_low_sell_price: 255,
    };
    let mut test_buy_order = OrderRequest {
        trader: Trader {
            id: ['t', 'e', 's', 't'],
        },
        symbol: TickerSymbol::AAPL,
        amount: 15,
        price: 6,
        order_type: OrderType::Buy,
    };
    let mut test_sell_order = OrderRequest {
        trader: Trader {
            id: ['t', 'e', 's', 't'],
        },
        symbol: TickerSymbol::AAPL,
        amount: 5,
        price: 4,
        order_type: OrderType::Sell,
    };
    let mut test_sell_order_2 = OrderRequest {
        trader: Trader {
            id: ['t', 'e', 's', 't'],
        },
        symbol: TickerSymbol::AAPL,
        amount: 5,
        price: 5,
        order_type: OrderType::Sell,
    };

    let sell_order_uuid = order_book.add_order_to_book(&test_sell_order_2);
    let sell_order_uuid = order_book.add_order_to_book(&test_sell_order);
    let buy_order_uuid = order_book.handle_incoming_buy(&mut test_buy_order);
    // let _removed_order =
    // order_book.remove_order(order_uuid, test_order.price, test_order.order_type);
    println!("{:#?}", order_book);
}
