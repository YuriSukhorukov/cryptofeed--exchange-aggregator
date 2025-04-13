// use std::sync::Arc;

use std::time::Instant;

use futures_util::{
    SinkExt,
    StreamExt,
    // stream::{SplitSink, SplitStream},
};

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::{Utf8Bytes, client::IntoClientRequest};

use crate::exchanges::{error::Error, traits::WebSocketApi};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;
// type Stream = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
// type Write = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
// type Read = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
pub struct BybitWebSocketApi {
    stream: Option<Stream>,
    // write: Option<Write>,
    // read: Option<Read>,
}

impl BybitWebSocketApi {
    pub fn new() -> Self {
        BybitWebSocketApi { stream: None }
    }
}

impl WebSocketApi<BybitWebSocketApi> for BybitWebSocketApi {
    fn new() -> Self {
        BybitWebSocketApi::new()
    }
    async fn subscribe_orderbook(&mut self, args: Vec<&str>) -> Result<(), Error> {
        // levels: 1, 50, 200, 500
        let subscribe_msg = serde_json::json!({
            "op": "subscribe",
            "args": args
        });
        let msg_text = subscribe_msg.to_string();
        let message =
            tokio_tungstenite::tungstenite::Message::Text(Utf8Bytes::try_from(msg_text).unwrap());
        let (mut write, mut read) = self.stream.as_mut().unwrap().split();
        if let Err(msg) = write.send(message).await {
            return Err(Error::new(msg.to_string()));
        };
        Ok(())
    }
    async fn subscribe_trades(&mut self, args: Vec<&str>) -> Result<(), Error> {
        // let subscribe_msg = br#"{"op": "subscribe", "args": ["publicTrade.BTCUSDT", "publicTrade.ETHUSDT", "publicTrade.SOLUSDT", "publicTrade.SUIUSDT"]}"#.to_vec();
        let subscribe_msg = serde_json::json!({
            "op": "subscribe",
            "args": args
        });

        let msg_text = subscribe_msg.to_string();
        let message =
            tokio_tungstenite::tungstenite::Message::Text(Utf8Bytes::try_from(msg_text).unwrap());

        let (mut write, mut read) = self.stream.as_mut().unwrap().split();
        if let Err(msg) = write.send(message).await {
            return Err(Error::new(msg.to_string()));
        };
        Ok(())
    }
    async fn run_loop(&mut self) -> Result<(), Error> {
        let (mut write, mut read) = self.stream.as_mut().unwrap().split();

        loop {
            let Some(Ok(mut msg)) = read.next().await else {
                break;
            };
            let start = Instant::now();
            println!("{:?}", msg);
            let duration = start.elapsed();
            println!("{duration:?}");
        }

        Ok(())
    }
    async fn connect(&mut self, host: &str, target: &str) -> Result<(), Error> {
        let mut request: hyper::Request<()> = "wss://stream.bybit.com/v5/public/linear"
            .into_client_request()
            .unwrap();
        // request
        //     .headers_mut()
        //     .insert("api-key", "42".parse().unwrap());

        let (ws_stream, response) = match tokio_tungstenite::connect_async(request).await {
            Ok(res) => {
                println!("Connected");
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
}
