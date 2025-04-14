// use std::sync::Arc;

use std::time::Instant;

use futures_util::{
    SinkExt,
    StreamExt,
    // stream::{SplitSink, SplitStream},
};
use serde_json::Value;
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
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SubscribeMsg {
    pub op: String,
    pub args: Vec<String>,
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
    async fn connect(&mut self, host: &str, target: &str) -> Result<(), Error> {
        let address = vec![host, target].join("/");
        println!("Connecting to: {}", &address);
        let request: hyper::Request<()> = "wss://stream.bybit.com/v5/public/linear"
            .into_client_request()
            .unwrap();
        // request
        //     .headers_mut()
        //     .insert("api-key", "42".parse().unwrap());

        let (ws_stream, response) = match tokio_tungstenite::connect_async(request).await {
            Ok(res) => {
                println!("Connected to Bybit");
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
        let msg_text = msg.to_string();
        let message =
            tokio_tungstenite::tungstenite::Message::Text(Utf8Bytes::try_from(msg_text).unwrap());
        let (mut write, mut read) = self.stream.as_mut().unwrap().split();
        if let Err(msg) = write.send(message).await {
            return Err(Error::new(msg.to_string()));
        };
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
            tx.send(Some(msg.to_string())).await?;
        }

        Ok(())
    }
}
