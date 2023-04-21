use actix_web::{get, Responder};

use crate::{models::restaurant_info::RestaurantInfo, routes::restaurants_to_value};

#[get("/restaurants")]
async fn restaurants() -> impl Responder {
    let ids = RestaurantInfo::get_all_ids().unwrap();
    let mut result = serde_json::json!([]);

    let mut restaurants = vec![];

    for (i, id) in ids.iter().enumerate() {
        if let Ok(restaurant) = RestaurantInfo::get(id) {
            restaurants.push(restaurant);
        } else {
            result[i]["message"] =
                serde_json::to_value(format!("Couldn't load the restaurant with id {}", id))
                    .unwrap();
        }
    }

    let result = restaurants_to_value(restaurants);

    actix_web::HttpResponse::Ok().json(result)
}
