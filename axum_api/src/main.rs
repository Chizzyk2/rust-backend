#![allow(unused_imports)]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router
};
use std::env;
use ::postgres::row;
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{postgres::{self, PgPoolOptions}, PgPool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    // Expose the environment variable

    dotenvy::dotenv().expect("Unable to access the .env file");
    
    // Set variables from the environment variable

    let server_address = std::env::var("SERVER_ADDRESS")
    .unwrap_or("127.0.0.1:4000".to_owned());
    let database_url = std::env::var("DATABASE_UR")
    .expect("Database Url not found in the .env file");

    // Create the database pool

    let _db_pool = PgPoolOptions::new()
    .max_connections(16)
    .connect(&database_url)
    .await
    .expect("Can't connect to the database");

    // Create our TCP listner

    let listen = TcpListener::bind(server_address)
    .await
    .expect("Can't connect to TCP listener");

    println!("Listening on {}", listen.local_addr().unwrap());

    // Compose the routes

    let app = Router::new()
    .route("/", get(|| async {"Hello world"}))
    .route("/tasks/", get(get_tasks).post(create_tasks))
    .route("/tasks/:task_id", patch(update_task).delete(delete_task))
    .with_state(_db_pool);

    // Serve the application

        axum::serve(listen, app)
        .await
        .expect("Error serving application");

}
#[derive(Debug, serde::Serialize)]
struct TaskRow{
    task_id: i32,
    name: String,
    priority: Option<i32>
}

async fn get_tasks(
    State(_pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)>{

    let row = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
    .fetch_all(&_pg_pool)
    .await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"Success" : false, "Message" : e.to_string()}).to_string()
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({"Success" : true, "data": row}).to_string()
    ))
}

async fn create_tasks(
    State(_pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)>
{

    todo!()
}

async fn update_task(
    State(_pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)>
{}

async fn delete_task(
    State(_pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)>
{
    todo!()
}