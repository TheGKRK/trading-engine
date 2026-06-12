mod trading_engine;
use trading_engine::orderbook::{Order,BidOrAsk,Orderbook};
use trading_engine::engine::{TradingPair, TradingEngine};
use rust_decimal_macros::dec;

fn main() {
 //   let price = Price::new(50.5);
   // println!("{:?}", price);

    let buy_order = Order::new(BidOrAsk::Bid, dec!(9.0));
   // let sell_order = Order::new(BidOrAsk::Ask, dec!(9.0));

    let buy_order_bob = Order::new(BidOrAsk::Bid, dec!(9.0));

    
    let mut order_book = Orderbook::new();
    order_book.add_limit_order(dec!(9.0), buy_order);
    order_book.add_limit_order(dec!(9.0), buy_order_bob);
 //   order_book.add_order(6.0, buy_order);
    println!("{:?}",order_book);

    let mut engine = TradingEngine::new();
    let pair = TradingPair::new("SOL".into(),"USD".into());
    engine.add_new_market(pair.clone());
    println!("{:?}",engine);

    let buy_order = Order::new(BidOrAsk::Bid, dec!(6.4));
  //  let eth_pair = TradingPair::new("ETH".into(),"USD".into());

    engine.place_limit_order(pair, dec!(10.0000), buy_order).unwrap();
}
