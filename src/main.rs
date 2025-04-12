mod exchanges;
mod parser;
mod types;

use exchanges::{
    binance::{BinanceRestApi, BinanceWebSocketApi},
    bybit::{BybitRestApi, BybitWebSocketApi},
    traits::{create_rest_api, create_ws_api},
};

fn main() {
    println!("hi");

    let bybit_rest: BybitRestApi = create_rest_api();
    let binance_rest: BinanceRestApi = create_rest_api();
    let bybit_ws: BybitWebSocketApi = create_ws_api();
    let binance_ws: BinanceWebSocketApi = create_ws_api();
}
