use axum::Json;


#[derive(Debug, serde::Serialize)]
pub struct Vehicle{
    manufacturer: String,
    model: String,
    year: u32,
    id: String

}

pub async fn vehicle_get() -> Json<Vehicle>{
    println!("Caller retrieved a vehicle from axum");
    Json::from(
        Vehicle{
            manufacturer : "Doge".to_string(),
            model: "RAM 1500".to_string(),
            year: 2021,
            id: uuid::Uuid::new_v4().to_string()
        }
    )
}

pub async fn vehicle_post(){

}