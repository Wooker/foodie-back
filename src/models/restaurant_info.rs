#![allow(non_snake_case)]

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use uuid::Uuid;

use crate::{
    db::connection, errors::CustomError, schema::restaurant_info, schema::restaurant_info::dsl::*,
    schema::restaurant_location,
};

#[derive(Debug, PartialEq, Identifiable, Selectable, Serialize, Deserialize, Queryable)]
#[diesel(table_name = restaurant_info)]
pub struct RestaurantInfo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: String,
    pub opening_hours: String,
    pub price_range: i16,
    pub rating: f32,
    pub image_url: String,
    pub contact: String,
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

        Ok(restaurants)
    }

    pub fn get(search_id: Uuid) -> Result<RestaurantInfo, CustomError> {
        let mut conn = connection()?;
        let restaurants = restaurant_info
            .get_results::<RestaurantInfo>(&mut conn)
            .unwrap();

        let restaurant = restaurants
            .into_iter()
            .find(|i| i.get_id().eq(&search_id))
            .unwrap();

        Ok(restaurant)
    }
}

#[derive(Debug, Serialize, Associations, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(primary_key(restaurant_info_id, latitude, longitude))]
#[diesel(table_name = restaurant_location)]
pub struct RestaurantLocation {
    #[serde(skip)]
    restaurant_info_id: Uuid,
    latitude: f32,
    longitude: f32,
}

impl RestaurantLocation {
    pub fn of_restaurant(other_id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let location: RestaurantLocation = restaurant_location::table
            .filter(restaurant_location::restaurant_info_id.eq(other_id))
            .select((
                restaurant_location::restaurant_info_id,
                restaurant_location::longitude,
                restaurant_location::latitude,
            ))
            .first(&mut conn)?;

        Ok(location)
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
