#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

//use uuid::Uuid;
use dotenv::dotenv;
use actix_web::{get, post, web, App, HttpServer, Responder};

mod db;
mod errors;
mod models;
mod schema;

use crate::errors::CustomError;

use models::{
    city::City,
    user_info::UserInfo,
    restaurant_info::RestaurantInfo,
    //restaurant_category::RestaurantCategory
};

#[get("/shirin")]
async fn shirin() -> impl Responder {
    actix_web::HttpResponse::Ok().body("Zakhar loves Shirin!")
}

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

/*
#[get("/cuisine/{restaurant_id}")]
async fn cuisine(restaurant_id: web::Path<String>) -> impl Responder {
    let result = RestaurantCuisine::of_restaurant(Uuid::parse_str(restaurant_id.as_str()).unwrap()).unwrap();
    serde_json::to_string(&result)
}
*/

#[post("/register")]
async fn register(user_info: web::Json<UserInfo>) -> impl Responder {
    if user_info.password_empty() {
        let error_message = serde_json::json!({
            "error": "Password is empty"
        });
        return actix_web::HttpResponse::InternalServerError().json(error_message)
    }

    if let Ok(login) = UserInfo::register(user_info.into_inner()) {
        let result = serde_json::to_string(&serde_json::json!({
            "login": login,
        })).unwrap();
        actix_web::HttpResponse::Ok().body(result)
    } else {
        let error_message = serde_json::json!({
            "error": "User already exists"
        });
        return actix_web::HttpResponse::InternalServerError().json(error_message)
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();

    db::init()?;

    HttpServer::new(|| {
        App::new()
            .service(cities)
            .service(restaurants)
            //.service(cuisine)
            .service(shirin)
            .service(register)
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await?;

    Ok(())
}
