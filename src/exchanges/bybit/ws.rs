use crate::exchanges::traits::WebSocketApi;

pub struct BybitWebSocketApi {}

impl BybitWebSocketApi {
    pub fn new() -> Self {
        BybitWebSocketApi {}
    }
}

impl WebSocketApi for BybitWebSocketApi {
    async fn subscribe_orderbook(&mut self, symbol: &str) -> Result<(), std::fmt::Error> {
        Ok(())
    }
    async fn run_loop(&mut self) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
