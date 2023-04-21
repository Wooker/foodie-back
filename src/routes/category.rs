use actix_web::{get, Responder};
use serde_json::{json, Value};

use crate::{models::restaurant_category::RestaurantCategory, routes::restaurants_to_value};

#[get("/categories")]
async fn categories() -> impl Responder {
    let result = RestaurantCategory::get_all().unwrap();

    let mut json: Vec<Value> = Vec::new();
    for (category, restaurants) in result.into_iter() {
        let restaurants_json = restaurants_to_value(restaurants);

        json.push(json!({
            "category": category.0,
            "image_url": category.1,
            "restaurants": restaurants_json
        }));
    }

    actix_web::HttpResponse::Ok().json(json)
}
