mod core;
mod config;
mod wshandler;

use crate::config::Config;
use crate::wshandler::WSHandler;
use tokio::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("Loading server...");
    let config = Config::load().unwrap();
    let listener = TcpListener::bind(&config.address).await.expect("Can't bind address");
    println!("Server ready");
    
    let handler = Arc::new(WSHandler::new());

    while let Ok((stream, _)) = listener.accept().await {
        let handler = Arc::clone(&handler);
        tokio::spawn(async move {
            handler.handler(stream).await;
        });
    }

    Ok(())
}
