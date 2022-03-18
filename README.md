# Overview
- [orderbook](./orderbook) - orderbook module implementation
- [orderbook_derive](./orderbook_derive) - derive trait for orderbook
- [server](./server)

# Run server
```bash
cargo run -- --pair [pair]
# i.e: cargo run -- --pair sol-usdt
```

# Run all test
```bash
cargo test -- --nocapture
```

# Run specific test
```bash
cargo test --test [filename] -- --nocapture
```
ex:
```
cargo test --test price_level_test -- --nocapture
```


# Kafka Topic
- `[pair]-orders` topic: consume `Order` from this topic. i.e: sol-usdt-orders
- `trades` topic: produce `Trade` to this topic
- `[pair]-orderbook-data` topic: produce `proto::orderbook::OrderBook` to this topic. i.e: sol-usdt-orderbook-data
