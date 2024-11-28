// websocket_server.rs
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub async fn start_server(addr: SocketAddr, sender: broadcast::Sender<String>) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {}", addr);

    // Accept incoming client connections
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_client(stream, sender.clone()));
    }
}

async fn handle_client(stream: tokio::net::TcpStream, sender: broadcast::Sender<String>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to broadcast channel
    let mut rx = sender.subscribe();

    // Listen for updates and send to client
    while let Ok(message) = rx.recv().await {
        let response = Message::Text(message);
        if write.send(response).await.is_err() {
            eprintln!("Client disconnected.");

            break;
        }
    }
}
