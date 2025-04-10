// pub trait RestApi {
//     async fn get_orderbook(&self, symbol: &str) -> Result<OrderBook, Error>;
//     async fn get_ticker(&self, ticker: &str) -> Result<Ticker, Error>;
// }

use reqwest::{Client, Response};

