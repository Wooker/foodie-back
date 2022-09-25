#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use uuid::Uuid;
use dotenv::dotenv;
use actix_web::{get, web, App, HttpServer, Responder};

mod db;
mod errors;
mod models;
mod schema;

use models::{
    city::City,
    restaurant_info::RestaurantInfo,
    restaurant_cuisine::RestaurantCuisine
};

#[get("/cities")]
async fn cities() -> impl Responder {
    let cities = City::find_all().unwrap();
    serde_json::to_string(&cities)
}

#[get("/restaurants")]
async fn restaurants() -> impl Responder {
    let result = RestaurantInfo::get_all().unwrap();
    serde_json::to_string(&result)
}

#[get("/cuisine/{restaurant_id}")]
async fn cuisine(restaurant_id: web::Path<String>) -> impl Responder {
    let result = RestaurantCuisine::of_restaurant(Uuid::parse_str(restaurant_id.as_str()).unwrap()).unwrap();
    serde_json::to_string(&result)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    db::init();

    HttpServer::new(|| {
        App::new()
            .service(cities)
            .service(restaurants)
            .service(cuisine)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
