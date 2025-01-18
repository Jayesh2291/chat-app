use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};

pub async fn connect_to_server() {
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:8080/ws").await.unwrap();

    // Send a message to the server
    ws_stream.send(tokio_tungstenite::tungstenite::protocol::Message::Text("Hello, Server!".to_string())).await.unwrap();

    // Receive messages from the server
    while let Some(Ok(msg)) = ws_stream.next().await {
        if let tokio_tungstenite::tungstenite::protocol::Message::Text(text) = msg {
            println!("Received: {}", text);
        }
    }
}
