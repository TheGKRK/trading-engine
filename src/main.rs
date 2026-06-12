mod trading_engine;
use trading_engine::orderbook::{Order, BidOrAsk, Orderbook};
use trading_engine::engine::{TradingPair, TradingEngine};
use rust_decimal_macros::dec;

fn main() {
    // Standalone orderbook with best bid/ask
    let mut order_book = Orderbook::new();
    order_book.add_limit_order(dec!(9.0), Order::new(BidOrAsk::Bid, dec!(9.0)));
    order_book.add_limit_order(dec!(9.0), Order::new(BidOrAsk::Bid, dec!(9.0)));
    println!("best bid: {:?}", order_book.best_bid());
    println!("{:?}", order_book);

    // Multi-pair engine with limit and market orders
    let mut engine = TradingEngine::new();
    let pair = TradingPair::new("SOL".into(), "USD".into());
    engine.add_new_market(pair.clone());

    engine.place_limit_order(&pair, dec!(10.0), Order::new(BidOrAsk::Ask, dec!(6.4))).unwrap();

    let mut market_buy = Order::new(BidOrAsk::Bid, dec!(6.4));
    engine.place_market_order(&pair, &mut market_buy).unwrap();
    println!("market order filled: {}", market_buy.is_filled());

    println!("{:?}", engine);
}
