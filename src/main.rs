mod exchanges;
mod parser;
mod types;

use exchanges::{
    binance::{BinanceRestApi, BinanceWebSocketApi},
    bybit::{BybitRestApi, BybitWebSocketApi},
    traits::{WebSocketApi, create_rest_api, create_ws_api},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bybit_ws: BybitWebSocketApi = create_ws_api();
    let mut binance_ws: BinanceWebSocketApi = create_ws_api();

    binance_ws
        .connect(
            "wss://data-stream.binance.vision",
            "stream?streams=btcusdt@trade&timeUnit=MICROSECOND",
        )
        .await
        .unwrap();

    bybit_ws
        .connect("wss://stream.bybit.com", "/v5/public/linear")
        .await
        .unwrap();
    bybit_ws
        .subscribe(serde_json::json!({
            "op": "subscribe",
            "args": vec![
              "publicTrade.BTCUSDT",
              "publicTrade.ETHUSDT",
              "publicTrade.SOLUSDT",
              "publicTrade.SUIUSDT",
              "orderbook.1.BTCUSDT",
              "orderbook.1.ETHUSDT",
              "orderbook.1.SOLUSDT",
              "orderbook.1.SUIUSDT",
              "orderbook.50.BTCUSDT",
              "orderbook.50.ETHUSDT",
              "orderbook.50.SOLUSDT",
              "orderbook.50.SUIUSDT",
              "orderbook.200.BTCUSDT",
              "orderbook.200.ETHUSDT",
              "orderbook.200.SOLUSDT",
              "orderbook.200.SUIUSDT",
              "orderbook.500.BTCUSDT",
              "orderbook.500.ETHUSDT",
              "orderbook.500.SOLUSDT",
              "orderbook.500.SUIUSDT",
            ]
        }))
        .await
        .unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Option<String>>(1000);
    let binance_tx = tx.clone();
    let bybit_tx = tx.clone();

    let binance_task = tokio::spawn(async move {
        binance_ws.run_loop(binance_tx).await.unwrap();
    });
    let bybit_task = tokio::spawn(async move {
        bybit_ws.run_loop(bybit_tx).await.unwrap();
    });

    let handler = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                Some(msg) => println!("{msg:?}"),
                None => {},
            }
        }
    });

    tokio::try_join!(binance_task, bybit_task, handler)?;

    Ok(())
}
