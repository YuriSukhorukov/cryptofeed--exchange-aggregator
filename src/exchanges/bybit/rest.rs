use crate::exchanges::models::{OrderBook, Ticker};
use crate::exchanges::traits::RestApi;
pub struct BybitRestApi {}

impl BybitRestApi {
    pub fn new() -> Self {
        BybitRestApi {}
    }
}

impl RestApi<BybitRestApi> for BybitRestApi {
    fn new() -> Self {
        BybitRestApi::new()
    }
    async fn get_orderbook(&self, symbol: &str) -> Result<(), std::fmt::Error> {
        Ok(())
    }
    async fn get_ticker(&self, ticker: &str) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
