mod exchanges;
mod parser;
mod types;

use std::sync::{Arc, Mutex, atomic::AtomicI32, mpsc};

use exchanges::{
    binance::{BinanceRestApi, BinanceWebSocketApi},
    bybit::{BybitRestApi, BybitWebSocketApi},
    traits::{WebSocketApi, create_rest_api, create_ws_api},
};

#[tokio::main]
async fn main() {
    println!("hi");

    // let mut bybit_rest: BybitRestApi = create_rest_api();
    // let binance_rest: BinanceRestApi = create_rest_api();
    let mut bybit_ws: BybitWebSocketApi = create_ws_api();
    // let binance_ws: BinanceWebSocketApi = create_ws_api();

    bybit_ws
        .connect("wss://stream.bybit.com", "/v5/public/linear")
        .await
        .unwrap();

    bybit_ws
        .subscribe_trades(vec![
            "publicTrade.BTCUSDT",
            "publicTrade.ETHUSDT",
            "publicTrade.SOLUSDT",
            "publicTrade.SUIUSDT",
        ])
        .await
        .unwrap();

    bybit_ws.run_loop().await.unwrap();
}
