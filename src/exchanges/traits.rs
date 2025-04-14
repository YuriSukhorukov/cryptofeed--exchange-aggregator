use crate::exchanges::{
    error::Error,
    models::{OrderBook, Ticker},
};
use serde_json::Value;

pub trait RestApi<T> {
    fn new() -> T;
    async fn get_orderbook(&self, symbol: &str) -> Result<(), Error>;
    async fn get_ticker(&self, ticker: &str) -> Result<(), Error>;
}

pub trait WebSocketApi<T> {
    fn new() -> T;
    async fn connect(&mut self, host: &str, target: &str) -> Result<(), Error>;
    async fn subscribe(&mut self, msg: Value) -> Result<(), Error>;
    async fn run_loop(&mut self, tx: tokio::sync::mpsc::Sender<Option<String>>) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn create_rest_api<T: RestApi<T>>() -> T {
    T::new()
}

pub fn create_ws_api<T: WebSocketApi<T>>() -> T {
    T::new()
}
