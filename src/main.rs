//! I stole most of my code from my previous APIs so this is based mostly around
//! a traditional axum API so that's why it's a weird format I think.

mod constant;
mod handler;
mod protocol;
mod result;
mod routing;
mod server;
mod tool;
mod tools;

use routing::app;
use server::McpServer;

use tokio::net::TcpListener;
use tracing::{Level, info, error};
use tracing_subscriber::FmtSubscriber;

use std::net::SocketAddr;

/// See https://docs.rs/axum/latest/axum/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting {} v{}", constant::SERVICE_NAME, constant::VERSION);

    let server: McpServer = McpServer::builder(constant::SERVICE_NAME, constant::VERSION)
        .tool(tools::Echo)
        .tool(tools::Add)
        .build();

    let app: axum::Router = app(server);

    let addr: SocketAddr = "[::]:3000".parse().expect("Failed to parse socket address");

    let listener: TcpListener = TcpListener::bind(addr).await.unwrap_or_else(|e: std::io::Error| {
        error!("Failed to bind to address {}: {}", addr, e);
        panic!("Failed to bind to address {}: {}", addr, e);
    });

    info!("MCP server listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap_or_else(|e: std::io::Error| {
        error!("Server error: {}", e);
        panic!("Server error: {}", e);
    });

    Ok(())
}
