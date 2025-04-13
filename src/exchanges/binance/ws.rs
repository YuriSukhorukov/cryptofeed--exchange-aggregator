use crate::exchanges::traits::WebSocketApi;

pub struct BinanceWebSocketApi {}

impl BinanceWebSocketApi {
    pub fn new() -> Self {
        BinanceWebSocketApi {}
    }
}

// impl WebSocketApi<BinanceWebSocketApi> for BinanceWebSocketApi {
//     fn new() -> Self {
//         BinanceWebSocketApi::new()
//     }
//     async fn subscribe_orderbook(&mut self, symbol: &str) -> Result<(), std::fmt::Error> {
//         Ok(())
//     }
//     async fn run_loop(&mut self) -> Result<(), std::fmt::Error> {
//         Ok(())
//     }
// }
