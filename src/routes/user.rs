use actix_web::{post, web, Responder};

use crate::models::user_info::UserInfo;

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
