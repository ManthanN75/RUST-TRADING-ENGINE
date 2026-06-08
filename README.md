# Rust Trading Matching Engine

A lightweight order matching engine written in Rust.  
This project models the core logic behind an exchange order book, including trading pairs, bid/ask order books, limit orders, market orders, price-level matching, and partial fills.

## Overview

The engine is designed to demonstrate how financial exchanges match buy and sell orders using price priority.

Each trading pair has its own order book. The order book stores buy orders and sell orders separately, grouped by price level. Market orders are matched against the best available opposite-side orders.

## Features

- Multiple trading pair support
- Separate bid and ask order books
- Limit order placement
- Market order execution
- Best-price matching
- Partial fills
- Multiple orders at the same price level
- Unit tests for core matching behavior
- Decimal-based price representation using `rust_decimal`

## Tech Stack

- Rust
- `BTreeMap` for sorted price levels
- `HashMap` for market-to-orderbook routing
- `rust_decimal` for precise price values
- `rust_decimal_macros` for decimal literals in tests/examples

## Project Structure

```txt
src/
├── main.rs
└── matching_engine/
    ├── mod.rs
    ├── engine.rs
    └── orderbook.rs
```
Core Components
1)MatchingEngine
   The MatchingEngine manages multiple markets.
   It stores a map of trading pairs to their respective order books.

2)TradingPair
   Represents a market such as BTC/USD.

3)OrderBook
   The OrderBook stores bids and asks separately.
   BTreeMap is used because price levels need to stay sorted.

     Ask orders are matched from lowest price to highest price.
     Bid orders are matched from highest price to lowest price.

4)
