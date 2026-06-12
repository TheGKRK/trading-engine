# Trading Engine

A limit order book trading engine written in Rust.

## Features

- Limit and market order placement
- Bid/ask order book with price-time priority matching
- Filled orders automatically removed from the book
- Best bid / best ask price discovery
- Multiple trading pairs via a central `TradingEngine`
- Precise decimal arithmetic using `rust_decimal`

## Project Structure

```
src/
├── main.rs
└── trading_engine/
    ├── mod.rs
    ├── orderbook.rs   # Order, Limit, Orderbook
    └── engine.rs      # TradingEngine, TradingPair
```

### Core Types

| Type | Description |
|---|---|
| `Order` | A single bid or ask with a `Decimal` size |
| `Limit` | A price level holding multiple orders |
| `Orderbook` | Bid and ask sides, each a `HashMap<Decimal, Limit>` |
| `TradingPair` | A base/quote pair (e.g. `SOL_USD`) |
| `TradingEngine` | Manages one `Orderbook` per `TradingPair` |


## Usage

### Standalone Orderbook

```rust
use trading_engine::orderbook::{Order, BidOrAsk, Orderbook};
use rust_decimal_macros::dec;

let mut orderbook = Orderbook::new();
orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, dec!(10)));
orderbook.add_limit_order(dec!(90),  Order::new(BidOrAsk::Bid, dec!(10)));

println!("best ask: {:?}", orderbook.best_ask()); // Some(100)
println!("best bid: {:?}", orderbook.best_bid()); // Some(90)

let mut market_order = Order::new(BidOrAsk::Bid, dec!(10));
orderbook.fill_market_order(&mut market_order);
println!("filled: {}", market_order.is_filled()); // true
```

### Multi-Pair Engine

```rust
use trading_engine::engine::{TradingPair, TradingEngine};

let mut engine = TradingEngine::new();
let pair = TradingPair::new("SOL".into(), "USD".into());
engine.add_new_market(pair.clone());

// Place a limit order
engine.place_limit_order(&pair, dec!(10), Order::new(BidOrAsk::Ask, dec!(6.4))).unwrap();

// Place a market order
let mut market_buy = Order::new(BidOrAsk::Bid, dec!(6.4));
engine.place_market_order(&pair, &mut market_buy).unwrap();
println!("filled: {}", market_buy.is_filled()); // true
```

## Build & Run

```bash
cargo build
cargo run
```

## Tests

```bash
cargo test
```

## Dependencies

- [`rust_decimal`](https://crates.io/crates/rust_decimal) - arbitrary-precision decimal arithmetic
