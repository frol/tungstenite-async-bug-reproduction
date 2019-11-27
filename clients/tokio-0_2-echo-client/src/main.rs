use std::env;

use futures::{SinkExt, StreamExt};
use tungstenite::protocol::Message;

use tokio::spawn;
use tokio_tungstenite::connect_async;

const ITERATIONS: usize = 100_000_000;

async fn run() {
    let connect_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut ws_tx, mut ws_rx) = ws_stream.split();
    let (quit_tx, quit_rx) = futures::channel::oneshot::channel();

    spawn(async move {
        let mut received = 0usize;
        loop {
            if let Some(_) = ws_rx.next().await {
                received += 1;
                if received % 100_000 == 0 {
                    println!("Received: {}", received);
                    if received >= ITERATIONS {
                        break;
                    }
                }
            } else {
                println!("Received: no new messages (ws_rx.next() is None)");
                break;
            }
        }
        println!("Received: ended with {} messages", received);
        quit_tx.send(()).unwrap();
    });

    for sent in 1usize..=ITERATIONS {
        if let Err(err) = ws_tx.send(Message::text("ping".to_owned())).await {
            println!("ERR: {:?}", err);
            continue;
        }
        if sent % 100_000 == 0 {
            println!("Sent: {}", sent);
        }
    }
    println!("Sent: DONE");
    quit_rx.await.unwrap();
}

#[tokio::main]
async fn main() {
    run().await;
}
