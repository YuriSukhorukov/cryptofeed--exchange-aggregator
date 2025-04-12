use crate::exchanges::models::{OrderBook, Ticker};
use std::fmt::Error;

pub trait RestApi<T> {
    fn new() -> T;
    async fn get_orderbook(&self, symbol: &str) -> Result<(), Error>;
    async fn get_ticker(&self, ticker: &str) -> Result<(), Error>;
}

pub trait WebSocketApi<T> {
    fn new() -> T;
    async fn subscribe_orderbook(&mut self, symbol: &str) -> Result<(), Error>;
    async fn run_loop(&mut self) -> Result<(), Error>;
}

pub fn create_rest_api<T: RestApi<T>>() -> T {
    T::new()
}

pub fn create_ws_api<T: WebSocketApi<T>>() -> T {
    T::new()
}
