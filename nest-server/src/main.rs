use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    routing::get,
    Router,
};
use ngrok::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(
            |ConnectInfo(remote_addr): ConnectInfo<SocketAddr>| async move {
                format!("Hello, {remote_addr:?}!\r\n")
            },
        ),
    );

    let tun = ngrok::Session::builder()
        // Read the token from the NGROK_AUTHTOKEN environment variable
        .authtoken("2oBb9ZjvpHjEjzZoEfqI7hVzDJJ_4WvqwQNNGsNB2xFUt857M")
        // Connect the ngrok session
        .connect()
        .await?
        .http_endpoint()
        // Start a tunnel with an HTTP edge
        .listen()
        .await?;

    println!("Tunnel started on URL: {:?}", tun.url());

    // Instead of binding a local port like so:
    // axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    // Run it with an ngrok tunnel instead!
    axum::Server::builder(tun)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}