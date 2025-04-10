use crate::exchanges::models::{OrderBook, Ticker};
use std::fmt::Error;

pub trait RestApi {
    async fn get_orderbook(&self, symbol: &str) -> Result<(), Error>;
    async fn get_ticker(&self, ticker: &str) -> Result<(), Error>;
}

pub trait WebSocketApi {
    async fn subscribe_orderbook(&mut self, symbol: &str) -> Result<(), Error>;
    async fn run_loop(&mut self) -> Result<(), Error>;
}
