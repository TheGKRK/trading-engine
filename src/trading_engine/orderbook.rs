#![allow(dead_code)]
use std::collections::HashMap;
use rust_decimal::prelude::*;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    pub size: Decimal,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: Decimal) -> Order {
        Order { size, bid_or_ask }
    }

    pub fn is_filled(&self) -> bool {
        self.size == Decimal::ZERO
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Decimal,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn total_volume(&self) -> Decimal {
        self.orders.iter().map(|order| order.size).sum()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = Decimal::ZERO;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = Decimal::ZERO;
                }
            }
            if market_order.is_filled() {
                break;
            }
        }
        self.orders.retain(|o| !o.is_filled());
    }
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn best_ask(&self) -> Option<Decimal> {
        self.asks.keys().cloned().min()
    }

    pub fn best_bid(&self) -> Option<Decimal> {
        self.bids.keys().cloned().max()
    }

    pub fn ask_limits(&self) -> Vec<&Limit> {
        let mut limits = self.asks.values().collect::<Vec<&Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    pub fn bid_limits(&self) -> Vec<&Limit> {
        let mut limits = self.bids.values().collect::<Vec<&Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    fn ask_limits_mut(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    fn bid_limits_mut(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Ask => self.bid_limits_mut(),
            BidOrAsk::Bid => self.ask_limits_mut(),
        };

        for limit in limits {
            limit.fill_order(market_order);
            if market_order.is_filled() {
                break;
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn limit_order_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        limit.add_order(Order::new(BidOrAsk::Bid, dec!(100)));

        let mut market_sell_order = Order::new(BidOrAsk::Ask, dec!(99));
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, dec!(1));
    }

    #[test]
    fn limit_order_fill_multiple() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        limit.add_order(Order::new(BidOrAsk::Bid, dec!(100)));
        limit.add_order(Order::new(BidOrAsk::Bid, dec!(100)));

        let mut market_sell_order = Order::new(BidOrAsk::Ask, dec!(199));
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.len(), 1);
        assert_eq!(limit.orders.get(0).unwrap().size, dec!(1));
    }

    #[test]
    fn limit_total_volume() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        limit.add_order(Order::new(BidOrAsk::Bid, dec!(100)));
        limit.add_order(Order::new(BidOrAsk::Bid, dec!(100)));

        assert_eq!(limit.total_volume(), dec!(200));
    }

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = Orderbook::new();
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Ask, dec!(10)));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, dec!(10)));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, dec!(10)));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Ask, dec!(10)));

        let mut market_order = Order::new(BidOrAsk::Bid, dec!(10));
        orderbook.fill_market_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);
        assert_eq!(orderbook.best_ask(), Some(dec!(100)));

        let ask_limits = orderbook.ask_limits();
        let matched_limit = ask_limits.get(0).unwrap();
        assert_eq!(matched_limit.price, dec!(100));
        assert_eq!(matched_limit.orders.len(), 0);
    }

    #[test]
    fn orderbook_best_bid_ask() {
        let mut orderbook = Orderbook::new();
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, dec!(1)));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, dec!(1)));
        orderbook.add_limit_order(dec!(90), Order::new(BidOrAsk::Bid, dec!(1)));
        orderbook.add_limit_order(dec!(80), Order::new(BidOrAsk::Bid, dec!(1)));

        assert_eq!(orderbook.best_ask(), Some(dec!(100)));
        assert_eq!(orderbook.best_bid(), Some(dec!(90)));
    }
}
