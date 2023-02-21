# OrderBook Aggregator

## Setup

```bash
$ git clone git@github.com:gruberb/order-book-aggregator.git
$ cd order-book-aggregator
```

## Run the project

1. Start the server

```bash
$ cargo run -p server 
```

The `server` crate is the default crate, therefore a simple `cargo run` is also enough.

2. Start the client

```bash
$ cargo run -p client
```

## Example output Client

When the server is started, shortly after new clients can connect via `cargo run -p client`,
which displays the following otuput on the command line:

```bash
❯ cargo run -p client
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/client`
Spread: 0.000001000000000001
Asks
Exchange: Binance, Price: 0.070049, Amount: 26.3711
Exchange: Binance, Price: 0.07005, Amount: 0.002
Exchange: Binance, Price: 0.070051, Amount: 0.002
Exchange: Binance, Price: 0.070052, Amount: 0.152
Exchange: Binance, Price: 0.070053, Amount: 2.3181
Exchange: Binance, Price: 0.070054, Amount: 3.8603
Exchange: Binance, Price: 0.070055, Amount: 0.002
Exchange: Binance, Price: 0.070056, Amount: 0.002
Exchange: Bitstamp, Price: 0.07005612, Amount: 0.6
Exchange: Binance, Price: 0.070057, Amount: 0.2337
Bids
Exchange: Binance, Price: 0.070048, Amount: 42.382
Exchange: Binance, Price: 0.070047, Amount: 4.2395
Exchange: Binance, Price: 0.070046, Amount: 0.002
Exchange: Binance, Price: 0.070045, Amount: 0.002
Exchange: Binance, Price: 0.070044, Amount: 0.002
Exchange: Binance, Price: 0.070043, Amount: 0.002
Exchange: Binance, Price: 0.070042, Amount: 0.002
Exchange: Binance, Price: 0.070041, Amount: 0.4315
Exchange: Binance, Price: 0.07004, Amount: 4.5752
Exchange: Binance, Price: 0.070039, Amount: 1.3896
```