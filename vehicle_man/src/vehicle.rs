use axum::{extract::Query, Json};
use uuid::uuid;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle{
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>

}

pub async fn vehicle_get() -> Json<Vehicle>{
    println!("Caller retrieved a vehicle from axum");
    Json::from(
        Vehicle{
            manufacturer : "Doge".to_string(),
            model: "RAM 1500".to_string(),
            year: 2021,
            id: Some(uuid::Uuid::new_v4().to_string())
        }
    )
}

// pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle>{
//     println!("manufacturer: {0}, model:{1}, year{2}", v.manufacturer, v.model, v.year);
//     v.id = Some(uuid::Uuid::new_v4().to_string());
//     Json::from(v)
// }

#[derive(Debug, serde::Deserialize)]

pub struct Customer{
    first_name: String,
    last_name: String
}

pub async fn vehicle_post(
    Query(mut v): Query<Vehicle>,
    Query(c): Query<Customer>
) -> Json<Vehicle>{
    println!("manufacturer: {0}, model: {1}, year: {2}", v.manufacturer, v.model, v.year);
    v.id = Some(uuid::Uuid::new_v4().to_string());
    println!("First name is {0}\nLast name is {1}", c.first_name, c.last_name);
    Json(v)
}