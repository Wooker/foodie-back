use actix_web::{delete, post, web, Responder};
use serde_json::json;

use crate::{
    models::{favorites::UserFavorite, user_info::UserLogin},
    routes::restaurants_to_value,
};

#[post("/get_favorites")]
async fn get_favorites(login: web::Json<UserLogin>) -> impl Responder {
    if let Ok(restaurants) = UserFavorite::get_all(login.into_inner().get_login()) {
        let result = restaurants_to_value(restaurants);
        actix_web::HttpResponse::Ok().json(result)
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[post("/favorite")]
async fn add_favorite(favorite: web::Json<UserFavorite>) -> impl Responder {
    if let Ok(_login) = UserFavorite::add(favorite.into_inner()) {
        actix_web::HttpResponse::Ok().json(json!({"message": "ok"}))
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[delete("/delete_favorite")]
async fn delete_favorite(favorite: web::Json<UserFavorite>) -> impl Responder {
    if let Ok(_login) = UserFavorite::delete(favorite.into_inner()) {
        actix_web::HttpResponse::Ok().json(json!({"message": "ok"}))
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}
