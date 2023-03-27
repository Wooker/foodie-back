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
    types::Location,
    user_info::UserInfo,
    restaurant_info::RestaurantInfo,
    restaurant_info::RestaurantLocation,
    restaurant_category::RestaurantCategory,
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

#[get("/categories")]
async fn categories() -> impl Responder {
    let result = RestaurantCategory::get_all().unwrap();
    let mut json: Vec<serde_json::Value> = Vec::new();
    for (category, ids) in result.iter() {
        json.push(serde_json::json!({
            "category": category,
            "restaurants": ids
        }));
    }

    actix_web::HttpResponse::Ok().json(json)
}

//TODO: allow REST input
#[get("/locations")]
async fn locations(query: web::Query<Location>) -> impl Responder {
    let result = RestaurantLocation::by_nearest_to(query.longitude, query.latitude).unwrap();

    actix_web::HttpResponse::Ok().json(result)
}

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


#[post("/login")]
async fn login_service(user_info: web::Json<UserInfo>) -> impl Responder {
    let user = user_info.into_inner();
    match UserInfo::check(user) {
        Ok(()) =>  actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            let error_message = serde_json::json!({
                "error": e.to_string()
            });
            return actix_web::HttpResponse::InternalServerError().json(error_message)
        }
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();

    db::init()?;

    let a: f32 = 20.2;
    println!("{}", a.powf(2.0));
    
    HttpServer::new(|| {
        App::new()
            .service(cities)
            .service(restaurants)
            //.service(cuisine)
            .service(shirin)
            .service(register)
            .service(login_service)
            .service(categories)
            .service(locations)
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await?;

    Ok(())
}
