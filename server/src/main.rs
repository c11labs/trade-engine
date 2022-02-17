use axum::{
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;
use tokio;
use tracing;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    app(5000).await
}

async fn app(port: u16) {
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/ping", get(ping))
        .route("/buy", post(buy))
        .route("/sell", post(sell));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument]
async fn root() -> &'static str {
    tracing::info!("root");

    "hello, world"
}

#[tracing::instrument]
async fn ping() -> &'static str {
    tracing::info!("ping");

    "Ok"
}

async fn buy() {}
async fn sell() {}
