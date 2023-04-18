use actix_files::Files;
use actix_web::{delete, get, middleware::Logger, post, web, App, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info};
use uuid::Uuid;

mod db;
mod errors;
mod models;
mod schema;

use crate::{
    errors::CustomError,
    models::{menu_item::MenuItem, restaurant_table::RestaurantTable},
    schema::restaurant_category,
};

use models::{
    city::City,
    favorites::UserFavorite,
    reservations::Reservation,
    restaurant_category::RestaurantCategory,
    restaurant_info::RestaurantInfo,
    restaurant_info::RestaurantLocation,
    types::{CategoryType, Location},
    user_info::{UserInfo, UserLogin},
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
    let restaurants = RestaurantInfo::get_all().unwrap();
    let mut result = serde_json::json!([]);

    for (i, restaurant) in restaurants.iter().enumerate() {
        // Add new object to the array
        result.as_array_mut().unwrap().push(serde_json::json!({}));

        // Get categories of the restaurant
        let mut conn = db::connection().unwrap();
        let cats: Vec<CategoryType> = RestaurantCategory::belonging_to(&restaurant)
            .select(restaurant_category::category_type)
            .load(&mut conn)
            .unwrap();

        let menu: Vec<MenuItem> = MenuItem::belonging_to(&restaurant)
            .select(MenuItem::as_select())
            .load(&mut conn)
            .unwrap();

        let location = RestaurantLocation::of_restaurant(restaurant.get_id()).unwrap();

        // Add all the fields
        result[i]["id"] = serde_json::to_value(restaurant.id).unwrap();
        result[i]["name"] = serde_json::to_value(restaurant.name.clone()).unwrap();
        result[i]["description"] = serde_json::to_value(restaurant.description.clone()).unwrap();
        result[i]["address"] = serde_json::to_value(restaurant.address.clone()).unwrap();
        result[i]["opening_hours"] =
            serde_json::to_value(restaurant.opening_hours.clone()).unwrap();
        result[i]["price_range"] = serde_json::to_value(restaurant.price_range.clone()).unwrap();
        result[i]["image_url"] = serde_json::to_value(restaurant.image_url.clone()).unwrap();
        result[i]["categories"] = serde_json::to_value(cats).unwrap();
        result[i]["coordinate"] = serde_json::to_value(location).unwrap();
        result[i]["menu_items"] = serde_json::to_value(menu).unwrap();
    }

    actix_web::HttpResponse::Ok().json(result)
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
        return actix_web::HttpResponse::InternalServerError().json(error_message);
    }

    if let Ok(login) = UserInfo::register(user_info.into_inner()) {
        let result = &serde_json::json!({
            "login": login,
        });
        actix_web::HttpResponse::Ok().json(result)
    } else {
        let error_message = serde_json::json!({
            "error": "User already exists"
        });
        return actix_web::HttpResponse::InternalServerError().json(error_message);
    }
}

#[post("/login")]
async fn login_service(user_info: web::Json<UserInfo>) -> impl Responder {
    let user = user_info.into_inner();
    match UserInfo::check(user.clone()) {
        Ok(()) => {
            let result = serde_json::json!({ "login": user.get_login() });
            actix_web::HttpResponse::Ok().json(result)
        }
        Err(e) => {
            let error_message = serde_json::json!({
                "error": e.to_string()
            });
            return actix_web::HttpResponse::InternalServerError().json(error_message);
        }
    }
}

#[post("/get_favorites")]
async fn get_favorites(login: web::Json<UserLogin>) -> impl Responder {
    if let Ok(data) = UserFavorite::get_all(login.into_inner().get_login()) {
        let result = serde_json::to_value(data).unwrap();
        actix_web::HttpResponse::Ok().json(result)
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[post("/favorite")]
async fn add_favorite(favorite: web::Json<UserFavorite>) -> impl Responder {
    if let Ok(_login) = UserFavorite::add(favorite.into_inner()) {
        actix_web::HttpResponse::Ok().finish()
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[delete("/delete_favorite")]
async fn delete_favorite(favorite: web::Json<UserFavorite>) -> impl Responder {
    if let Ok(_login) = UserFavorite::delete(favorite.into_inner()) {
        actix_web::HttpResponse::Ok().finish()
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[get("{restaurant_id}/tables")]
async fn get_tables(path: web::Path<(Uuid,)>) -> impl Responder {
    let restaurant_id = path.into_inner().0;
    if let Ok(restaurant) = RestaurantInfo::get(restaurant_id) {
        let tables = RestaurantTable::of_restaurant(&restaurant).unwrap();
        let result = serde_json::to_value(tables).unwrap();
        actix_web::HttpResponse::Ok().json(result)
    } else {
        actix_web::HttpResponse::InternalServerError().finish()
    }
}

#[post("/get_reservations")]
async fn get_reservations(login: web::Json<UserLogin>) -> impl Responder {
    if let Ok(data) = Reservation::get_all(login.into_inner().get_login()) {
        let mut result = serde_json::json!([]); //serde_json::to_value(data).unwrap();

        for (i, reservation) in data.iter().enumerate() {
            result.as_array_mut().unwrap().push(serde_json::json!({}));

            result[i]["restaurant_id"] = serde_json::to_value(reservation.0).unwrap();
            result[i]["table_id"] = serde_json::to_value(reservation.1).unwrap();
        }
        actix_web::HttpResponse::Ok().json(result)
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[post("/reserve")]
async fn add_reservation(reservation: web::Json<Reservation>) -> impl Responder {
    if let Ok(_login) = Reservation::add(reservation.into_inner()) {
        actix_web::HttpResponse::Ok().finish()
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[delete("/delete_reservation")]
async fn delete_reservation(reservation: web::Json<Reservation>) -> impl Responder {
    if let Ok(_login) = Reservation::delete(reservation.into_inner()) {
        actix_web::HttpResponse::Ok().finish()
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();

    db::init()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/images", "./images/").show_files_listing())
            .service(cities)
            .service(restaurants)
            .service(shirin)
            .service(register)
            .service(login_service)
            .service(categories)
            .service(locations)
            .service(get_favorites)
            .service(add_favorite)
            .service(delete_favorite)
            .service(get_tables)
            .service(get_reservations)
            .service(add_reservation)
            .service(delete_reservation)
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await?;

    Ok(())
}
