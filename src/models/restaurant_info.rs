#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use serde_json::Value;
use bigdecimal::{BigDecimal, num_traits::Pow};
use std::cmp::Ordering;

use crate::{
    db::connection,
    errors::CustomError,
    schema::restaurant_info,
    schema::restaurant_info::dsl::*,
    schema::restaurant_location,
    schema::restaurant_location::dsl::*,
};


#[derive(Debug, PartialEq, Identifiable, Selectable, Serialize, Deserialize, Queryable)]
#[diesel(table_name = restaurant_info)]
pub struct RestaurantInfo {
    id: uuid::Uuid,
    name: String,
    description: String,
    address: String,
    openHours: String,
    averagePrice: Option<BigDecimal>,
    images: Option<String>,
}

impl RestaurantInfo {
    pub fn get_all() -> Result<Value, CustomError> {
        let mut conn = connection()?;
        let rests = restaurant_info.get_results::<RestaurantInfo>(&mut conn).unwrap();

        Ok(serde_json::to_value(rests).unwrap())
    }
}


#[derive(Debug, Serialize, Associations, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(table_name = restaurant_location)]
pub struct RestaurantLocation {
    restaurant_info_id: uuid::Uuid,
    latitude: f32,
    longitude: f32
}

impl RestaurantLocation {
    pub fn get_all() -> Result<Value, CustomError> {
        todo!();
    }

    pub fn by_nearest_to(some_longitude: f32, some_latitude: f32) -> Result<Value, CustomError> {
        let mut conn = connection()?;

        let mut all_restaurants = restaurant_location::table
            .inner_join(restaurant_info::table)
            .select((RestaurantLocation::as_select(), RestaurantInfo::as_select()))
            .load::<(RestaurantLocation, RestaurantInfo)>(&mut conn)?;
       
        all_restaurants
            .sort_by(|a, b| {
                let distance_a = f32::sqrt(
                    (some_longitude - a.0.longitude).powf(2.0) +
                    (some_latitude - a.0.latitude).powf(2.0)
                );
                let distance_b = f32::sqrt(
                    (some_longitude - b.0.longitude).powf(2.0) +
                    (some_latitude - b.0.latitude).powf(2.0)
                );

                if distance_a < distance_b {
                    Ordering::Less
                } else if distance_a == distance_b {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            });
        let a = all_restaurants
            .iter()
            .map(|r| serde_json::json!({
                "location": {
                    "longitude": r.0.longitude,
                    "latitude": r.0.latitude
                },
                "restaurant": r.1
            }))
            .collect::<serde_json::Value>();


        Ok(serde_json::to_value(a).unwrap())
    }
}
