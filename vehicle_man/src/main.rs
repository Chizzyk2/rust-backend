#[allow(unused_imports)]
mod vehicle;

use axum::{routing::{get, post}, Router};
use vehicle::{vehicle_get, vehicle_post};

#[tokio::main]
async fn main(){
    // Create the axum route

    let route01 = Router::new()
    .route("/vehicle", get(vehicle_get))
    .route("/vehicle", post(vehicle_post));

    // create the address and a TCPlistener
    let address = "127.0.0.1:4000";
    let listen = tokio::net::TcpListener::bind(address)
    .await
    .unwrap();

    // Create the axum server
    axum::serve(listen, route01)
    .await
    .unwrap();
}
