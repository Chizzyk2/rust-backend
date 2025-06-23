#![allow(unused_imports)]

use axum::{debug_handler, routing::{get, post}, serve::{self, Listener}, Json, Router};


#[tokio::main]

async fn main(){

    // 1. Create an axum router

    let route01 = Router::new()
    .route("/vehicle", get(vehicle_get))
    .route("/vehicle", post(vehicle_post));

    // 2. Create the address and listener

    let address = "127.0.0.1:4000";
    let listen = tokio::net::TcpListener::bind(address)
    .await
    .unwrap();

    println!("Server running at {address:?}");

    // 3. Create the axum server

    axum::serve(listen, route01)
    .await
    .unwrap();
}

#[derive(Debug, serde::Serialize)]
struct Vehicle{
    manufacturer: String,
    model: String,
    year: u32,
    id: String
}

#[debug_handler]
async fn vehicle_get() -> Json<Vehicle>{
    Json::from(
        Vehicle{
            manufacturer: "Doge".to_string(),
            model: "RAM 1500".to_string(),
            year: 2021,
            id: uuid::Uuid::new_v4().to_string()
        }
    )
}

async fn vehicle_post(){

}
