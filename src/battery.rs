

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub async fn battery_check() {
    let url = "ws://localhost:42070/?client=vrcwatch-rs";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to {}", url);
    let (write, read) = ws_stream.split();
    

    let ws_stdout ={
        read.for_each(|message| async {
            match message {
                Ok(msg) => {
                    println!("Received: {}", msg.to_text().unwrap());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        })
    };
}