use actix_web::{get, web, Responder};

use crate::{
    models::{restaurant_location::RestaurantLocation, types::Location},
    routes::restaurants_to_value,
};

#[get("/locations")]
async fn locations(query: web::Query<Location>) -> impl Responder {
    let restaurants = RestaurantLocation::by_nearest_to(query.longitude, query.latitude).unwrap();

    let result = restaurants_to_value(restaurants);

    actix_web::HttpResponse::Ok().json(result)
}
