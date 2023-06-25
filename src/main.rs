// server implementation
use axum::routing::get;
use axum::Server;
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
      .with_line_number(true)
      .with_max_level(Level::DEBUG)
      .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let ns = Namespace::builder()
        .add("/", |socket| async move {
            info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.sid);

            socket.on("hello", |socket, data: Value, bin, _| async move {
                info!("Received event: {:?} {:?}", data, bin);
                socket.bin(bin).emit("world", data).ok();
            });
        })
        .build();

    let app = axum::Router::new()
        .route("/", get(|| async { "Up and running" }))
        .layer(SocketIoLayer::new(ns));

    let addr = &"127.0.0.1:3000".parse().unwrap();

    info!("Starting server on {}", addr);

    Server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
