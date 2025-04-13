use crate::exchanges::models::{OrderBook, Ticker};
use crate::exchanges::{error::Error, traits::RestApi};

pub struct BinanceRestApi {}

impl BinanceRestApi {
    pub fn new() -> Self {
        BinanceRestApi {}
    }
}

impl RestApi<BinanceRestApi> for BinanceRestApi {
    fn new() -> Self {
        BinanceRestApi::new()
    }
    async fn get_orderbook(&self, symbol: &str) -> Result<(), Error> {
        Ok(())
    }
    async fn get_ticker(&self, ticker: &str) -> Result<(), Error> {
        Ok(())
    }
}
