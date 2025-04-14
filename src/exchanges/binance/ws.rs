use crate::exchanges::error::Error;
use crate::exchanges::traits::WebSocketApi;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::client::IntoClientRequest;

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub struct BinanceWebSocketApi {
    stream: Option<Stream>,
}

impl BinanceWebSocketApi {
    pub fn new() -> Self {
        BinanceWebSocketApi { stream: None }
    }
}

impl WebSocketApi<BinanceWebSocketApi> for BinanceWebSocketApi {
    fn new() -> Self {
        BinanceWebSocketApi::new()
    }
    async fn connect(&mut self, host: &str, target: &str) -> Result<(), Error> {
        let address = vec![host, target].join("/");
        println!("Connecting to: {}", &address);
        let request: hyper::Request<()> = address.into_client_request().unwrap();
        let (ws_stream, response) = match tokio_tungstenite::connect_async(request).await {
            Ok(res) => {
                println!("Connected to Binance");
                res
            }
            Err(err) => {
                println!("Error: {err}");
                return Err(Error::new(err.to_string()));
            }
        };
        self.stream = Some(ws_stream);
        Ok(())
    }
    async fn subscribe(&mut self, msg: Value) -> Result<(), Error> {
        Ok(())
    }
    async fn run_loop(
        &mut self,
        tx: tokio::sync::mpsc::Sender<Option<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (mut write, mut read) = self.stream.as_mut().unwrap().split();

        loop {
            let Some(Ok(mut msg)) = read.next().await else {
                break;
            };
            // println!("{:?}", msg);
            tx.send(Some(msg.to_string())).await?;
        }

        Ok(())
    }
}
