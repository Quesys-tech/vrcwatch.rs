use futures_util::{future, pin_mut, SinkExt, StreamExt};
use serde::Serialize;
use serde_json::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Serialize)]
struct XsApiCommand {
    sender: String,
    target: String,
    command: String,
    jsonData: String,
    rawData: String,
}

pub async fn battery_check() {
    let url = "ws://localhost:42070/?client=vrcwatch-rs";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to {}", url);
    let (mut write, mut read) = ws_stream.split();

    let command = XsApiCommand {
        sender: "vrcwatch-rs".to_string(),
        target: "xsoverlay".to_string(),
        command: "RequestDeviceInformation".to_string(),
        jsonData: "".to_string(),
        rawData: "".to_string(),
    };
    let command_str = serde_json::to_string(&command).expect("Failed to serialize command");
    println!("Sent message: {}", command_str);
    
    let message_tx = Message::Text(command_str.into());
    write
        .send(message_tx)
        .await
        .expect("Failed to send message");
    let message_rx = read
        .next()
        .await
        .expect("Failed to receive message")
        .expect("Connection closed");
    let message = message_rx
        .to_text()
        .expect("Failed to convert message to text")
        .to_string();
    println!("Received message: {}", message);
}
