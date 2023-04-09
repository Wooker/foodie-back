#![allow(non_snake_case)]

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use uuid::Uuid;

use crate::{
    db::connection,
    errors::CustomError,
    models::{restaurant_category::RestaurantCategory, types::CategoryType},
    schema::restaurant_category::dsl::*,
    schema::restaurant_info,
    schema::restaurant_info::dsl::*,
    schema::restaurant_location,
};

#[derive(Debug, PartialEq, Identifiable, Selectable, Serialize, Deserialize, Queryable)]
#[diesel(table_name = restaurant_info)]
pub struct RestaurantInfo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: String,
    pub openHours: String,
    pub averagePrice: Option<BigDecimal>,
    pub image_url: Option<String>,
}

impl RestaurantInfo {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_all() -> Result<Vec<RestaurantInfo>, CustomError> {
        let mut conn = connection()?;
        let restaurants = restaurant_info
            .get_results::<RestaurantInfo>(&mut conn)
            .unwrap();

        for restaurant in restaurants.iter() {
            let categories: Vec<CategoryType> = RestaurantCategory::belonging_to(&restaurant)
                .select(category_type)
                .load(&mut conn)?;
        }

        Ok(restaurants)
    }
}

#[derive(Debug, Serialize, Associations, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(table_name = restaurant_location)]
pub struct RestaurantLocation {
    restaurant_info_id: Uuid,
    latitude: f32,
    longitude: f32,
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

        all_restaurants.sort_by(|a, b| {
            let distance_a = f32::sqrt(
                (some_longitude - a.0.longitude).powf(2.0)
                    + (some_latitude - a.0.latitude).powf(2.0),
            );
            let distance_b = f32::sqrt(
                (some_longitude - b.0.longitude).powf(2.0)
                    + (some_latitude - b.0.latitude).powf(2.0),
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
            .map(|r| {
                serde_json::json!({
                    "location": {
                        "longitude": r.0.longitude,
                        "latitude": r.0.latitude
                    },
                    "restaurant": r.1
                })
            })
            .collect::<serde_json::Value>();

        Ok(serde_json::to_value(a).unwrap())
    }
}
