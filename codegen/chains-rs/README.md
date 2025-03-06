# chains-rs

## Overview
`chains-rs` is a Rust library for defining blockchain networks and their associated assets. 
It provides a structured way to represent various blockchains and their respective assets, enabling 
easy integration into cryptocurrency-related applications.

## Features
- Standardized blockchain and asset definitions.
- Support for multiple blockchain networks (e.g., Bitcoin, Binance, Pactus).
- Extensible asset representation.
- Designed for interoperability with Web3 and crypto applications.

## Installation
To add `chains-rs` to your project, include it in your `Cargo.toml`:

```toml
[dependencies]
chains-rs = "0.1.0"
```

Or use Cargo to add it directly:
```sh
cargo add chains-rs
```

## Usage
Here's a basic example of how to use `chains-rs`:

```rust
use chains_rs::blockchain::Blockchain;
use chains_rs::binance::BinanceBlockchain;

fn main() {
    let binance = BinanceBlockchain;
    println!("Blockchain: {}", binance.name());
}
```

## Documentation
The complete API documentation is available at: [docs.rs/chains-rs](https://docs.rs/chains-rs)
