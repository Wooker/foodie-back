use actix_web::{delete, post, web, Responder};
use serde_json::{json, to_value};

use crate::{
    models::{
        reservations::{Reservation, ReservationMask},
        user_info::{UserLogin, UserReservation},
    },
    routes::restaurant_to_value,
};

#[post("/get_reservations")]
async fn get_reservations(login: web::Json<UserLogin>) -> impl Responder {
    if let Ok(restaurant_reservation) = Reservation::get_all(login.into_inner().get_login()) {
        let mut result = json!([]);

        for (i, (restaurant, reservation)) in restaurant_reservation.iter().enumerate() {
            result.as_array_mut().unwrap().push(serde_json::json!({}));
            result[i]["reservation"] = to_value(reservation).unwrap();
            result[i]["restaurant"] = restaurant_to_value(restaurant);
        }
        actix_web::HttpResponse::Ok().json(result)
    } else {
        return actix_web::HttpResponse::InternalServerError().finish();
    }
}

#[post("/reserve")]
async fn add_reservation(reservation: web::Json<ReservationMask>) -> impl Responder {
    let reservation = reservation.into_inner();
    if let Ok(_login) = Reservation::add(Reservation::new(reservation)) {
        actix_web::HttpResponse::Ok().finish()
    } else {
        let json = serde_json::json!({
            "message": "Could not create the reservation"
        });
        return actix_web::HttpResponse::InternalServerError().json(json);
    }
}

#[delete("/delete_reservation")]
async fn delete_reservation(reservation: web::Json<UserReservation>) -> impl Responder {
    match Reservation::delete(reservation.into_inner()) {
        Ok(_login) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            let json = serde_json::json!({
                "message": e.to_string()
            });
            actix_web::HttpResponse::InternalServerError().json(json)
        }
    }
}
