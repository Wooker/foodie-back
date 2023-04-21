pub mod category;
pub mod city;
pub mod favorite;
pub mod location;
pub mod reservation;
pub mod restaurant;
pub mod table;
pub mod user;

use actix_web::{get, Responder};
use serde_json::{json, Value};

use crate::models::{
    menu_item::MenuItem, restaurant_info::RestaurantInfo, restaurant_location::RestaurantLocation,
    types::CategoryType,
};

#[get("/shirin")]
async fn shirin() -> impl Responder {
    actix_web::HttpResponse::Ok().body("Zakhar loves Shirin!")
}
pub fn restaurant_to_value(
    (info, categories, menu, location): &(
        RestaurantInfo,
        Vec<CategoryType>,
        Vec<MenuItem>,
        RestaurantLocation,
    ),
) -> Value {
    let mut result = json!({});

    result["id"] = serde_json::to_value(info.id.clone()).unwrap();
    result["address"] = serde_json::to_value(info.address.clone()).unwrap();
    result["contact"] = serde_json::to_value(info.contact.clone()).unwrap();
    result["description"] = serde_json::to_value(info.description.clone()).unwrap();
    result["image_url"] = serde_json::to_value(info.image_url.clone()).unwrap();
    result["name"] = serde_json::to_value(info.name.clone()).unwrap();
    result["opening_hours"] = serde_json::to_value(info.opening_hours.clone()).unwrap();
    result["price_range"] = serde_json::to_value(info.price_range.clone()).unwrap();
    result["rating"] = serde_json::to_value(info.rating.clone()).unwrap();
    result["categories"] = serde_json::to_value(categories).unwrap();
    result["menu"] = serde_json::to_value(menu).unwrap();
    result["location"] = serde_json::to_value(location).unwrap();

    result
}

pub fn restaurants_to_value(
    restaurants: Vec<(
        RestaurantInfo,
        Vec<CategoryType>,
        Vec<MenuItem>,
        RestaurantLocation,
    )>,
) -> Value {
    let mut result = json!([]);
    for (i, restaurant) in restaurants.iter().enumerate() {
        // Add new object to the array
        result.as_array_mut().unwrap().push(serde_json::json!({}));

        result[i] = restaurant_to_value(restaurant)
        /*
        result[i]["info"] = serde_json::to_value(info).unwrap();
        result[i]["categories"] = serde_json::to_value(categories).unwrap();
        result[i]["menu"] = serde_json::to_value(menu).unwrap();
        result[i]["location"] = serde_json::to_value(location).unwrap();
        */
    }

    result
}
