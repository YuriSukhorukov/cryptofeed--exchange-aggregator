mod exchanges;
mod parser;
mod types;

use exchanges::bybit::{BybitRestApi, BybitWebSocketApi};

fn main() {
    println!("hi");

    let bybit_rest = BybitRestApi::new();
    let bybit_ws = BybitWebSocketApi::new();
}
