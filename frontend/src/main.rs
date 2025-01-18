use tui::{backend::CrosstermBackend, widgets::{Block, Borders}, Terminal};
use tokio_tungstenite::connect_async;
use tokio::net::TcpStream;
use std::io;
use futures_util::{SinkExt, StreamExt};

mod chat_ui;
mod websocket_client;
mod auth;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:8080/ws").await.unwrap();

    // Setup the terminal UI
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Chat").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    // Send and receive WebSocket messages
    let message = "Hello, WebSocket!";
    ws_stream.send(tokio_tungstenite::tungstenite::protocol::Message::Text(message.to_string())).await.unwrap();

    while let Some(Ok(msg)) = ws_stream.next().await {
        if let tokio_tungstenite::tungstenite::protocol::Message::Text(text) = msg {
            println!("Received: {}", text);
        }
    }

    Ok(())
}
