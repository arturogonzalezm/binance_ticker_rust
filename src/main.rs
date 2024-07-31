use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ticker::Ticker;
use tokio_postgres::{NoTls, Error};

mod observer;
mod ticker;

use observer::{Subject, TickerSubject, PrintObserver};

async fn insert_ticker_data(client: &tokio_postgres::Client, ticker: &Ticker) -> Result<(), Error> {
    client.execute(
        "INSERT INTO ticker_data (
            event_type, event_time, symbol, price_change, price_change_percent, weighted_avg_price,
            first_trade_price, last_price, last_quantity, best_bid_price, best_bid_quantity,
            best_ask_price, best_ask_quantity, open_price, high_price, low_price, volume,
            quote_volume, statistics_open_time, statistics_close_time, first_trade_id,
            last_trade_id, total_trades
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)",
        &[
            &ticker.e, &(ticker.event_time as i64), &ticker.s, &ticker.p, &ticker.price_change_percent,
            &ticker.w, &ticker.x, &ticker.c, &ticker.last_quantity, &ticker.b, &ticker.best_bid_quantity,
            &ticker.a, &ticker.best_ask_quantity, &ticker.o, &ticker.h, &ticker.l, &ticker.v,
            &ticker.q, &(ticker.statistics_open_time as i64), &(ticker.statistics_close_time as i64),
            &(ticker.first_trade_id as i64), &(ticker.last_trade_id as i64), &(ticker.n as i64)
        ]
    ).await?;
    Ok(())
}

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

    // Connect to PostgreSQL
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls)
            .await
            .expect("Failed to connect to PostgreSQL");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

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

                // Insert data into PostgreSQL
                if let Err(e) = insert_ticker_data(&client, &ticker).await {
                    eprintln!("Error inserting data: {}", e);
                }
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
