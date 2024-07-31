use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ticker::Ticker;

mod observer;
mod ticker;

use observer::{Subject, TickerSubject, PrintObserver};

#[tokio::main]
async fn main() {
    let symbol = "btcusdt";
    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@ticker", symbol);
    let url = Url::parse(&ws_url).expect("Can't parse URL");

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Binance WebSocket API");

    let (write, mut read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    // Create subject and register observers
    let ticker_subject = Arc::new(Mutex::new(TickerSubject::new()));
    ticker_subject.lock().await.register_observer(Box::new(PrintObserver));

    // To keep the connection alive
    let write_clone = Arc::clone(&write);
    tokio::spawn(async move {
        let ping_msg = Message::Ping(vec![]);
        loop {
            let mut write_lock = write_clone.lock().await;
            if write_lock.send(ping_msg.clone()).await.is_err() {
                println!("Error sending ping");
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    });

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(txt)) => {
                let ticker: Ticker = serde_json::from_str(&txt).expect("Can't parse ticker");
                ticker_subject.lock().await.notify_observers(&ticker);
            }
            Ok(Message::Ping(_)) => {
                let mut write_lock = write.lock().await;
                if write_lock.send(Message::Pong(vec![])).await.is_err() {
                    println!("Error sending pong");
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
