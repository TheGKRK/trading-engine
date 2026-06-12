# Trading Engine

A limit order book trading engine written in Rust.

## Features

- Limit order placement for bid and ask sides
- Market order filling with price-time priority
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

```rust
use trading_engine::orderbook::{Order, BidOrAsk, Orderbook};
use trading_engine::engine::{TradingPair, TradingEngine};
use rust_decimal_macros::dec;

// Standalone orderbook
let mut orderbook = Orderbook::new();
orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Bid, dec!(10)));
orderbook.add_limit_order(dec!(101), Order::new(BidOrAsk::Ask, dec!(5)));

// Fill a market order
let mut market_order = Order::new(BidOrAsk::Bid, dec!(5));
orderbook.fill_market_order(&mut market_order);

// Multi-pair engine
let mut engine = TradingEngine::new();
let pair = TradingPair::new("SOL".into(), "USD".into());
engine.add_new_market(pair.clone());
engine.place_limit_order(pair, dec!(10), Order::new(BidOrAsk::Bid, dec!(6.4))).unwrap();
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

- [`rust_decimal`](https://crates.io/crates/rust_decimal) — arbitrary-precision decimal arithmetic
