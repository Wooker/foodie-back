use actix_web::{get, web, Responder};
use uuid::Uuid;

use crate::models::{restaurant_info::RestaurantInfo, restaurant_table::RestaurantTable};

#[get("{restaurant_id}/tables")]
async fn get_tables(path: web::Path<(Uuid,)>) -> impl Responder {
    let restaurant_id = path.into_inner().0;
    if let Ok((restaurant, _categories, _menu, _location)) = RestaurantInfo::get(&restaurant_id) {
        let tables = RestaurantTable::of_restaurant(&restaurant).unwrap();

        let result = serde_json::to_value(tables).unwrap();
        actix_web::HttpResponse::Ok().json(result)
    } else {
        actix_web::HttpResponse::InternalServerError().finish()
    }
}
