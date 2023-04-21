use actix_web::{get, Responder};

use crate::models::city::City;

#[get("/cities")]
async fn cities() -> impl Responder {
    let cities = City::find_all().unwrap();
    serde_json::to_string(&cities)
}
