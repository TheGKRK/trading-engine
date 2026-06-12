use std::collections::HashMap;
use rust_decimal::prelude::*;
use super::orderbook::{Orderbook, Order};

#[derive(Debug)]
pub struct TradingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl TradingEngine {
    pub fn new() -> TradingEngine {
        TradingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("opening new orderbook for market {}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: &TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("placed limit order at {}", price);
                Ok(())
            }
            None => Err(format!(
                "orderbook for pair {} does not exist",
                pair.to_string()
            )),
        }
    }

    pub fn place_market_order(
        &mut self,
        pair: &TradingPair,
        order: &mut Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(pair) {
            Some(orderbook) => {
                orderbook.fill_market_order(order);
                println!("filled market order, remaining size: {}", order.size);
                Ok(())
            }
            None => Err(format!(
                "orderbook for pair {} does not exist",
                pair.to_string()
            )),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}
