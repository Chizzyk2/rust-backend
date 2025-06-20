#[allow(unused_imports)]
use axum::{routing::get, serve::Listener, Router};

#[tokio::main]

async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
    .await
    .unwrap();

    println!("Server running at {addr:?}");

    axum::serve(listener, router())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new().route("/", get(hello_world))

}

async fn hello_world() -> &'static str {
    "Hello, world! from axum"
}