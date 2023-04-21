use std::cmp::Ordering;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::connection,
    errors::CustomError,
    models::{menu_item::MenuItem, restaurant_info::RestaurantInfo, types::CategoryType},
    schema::restaurant_info,
    schema::restaurant_location,
};

#[derive(Debug, Serialize, Identifiable, Associations, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(primary_key(restaurant_info_id))]
#[diesel(table_name = restaurant_location)]
pub struct RestaurantLocation {
    #[serde(skip)]
    restaurant_info_id: Uuid,
    latitude: f32,
    longitude: f32,
}

impl RestaurantLocation {
    pub fn of_restaurant(restaurant: &RestaurantInfo) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let location = Self::belonging_to(restaurant).first::<Self>(&mut conn)?;

        Ok(location)
    }

    pub fn by_nearest_to(
        some_longitude: f32,
        some_latitude: f32,
    ) -> Result<
        Vec<(
            RestaurantInfo,
            Vec<CategoryType>,
            Vec<MenuItem>,
            RestaurantLocation,
        )>,
        CustomError,
    > {
        let mut conn = connection()?;

        let mut all_restaurants = restaurant_location::table
            .inner_join(restaurant_info::table)
            .select((RestaurantLocation::as_select(), restaurant_info::id))
            .load::<(RestaurantLocation, Uuid)>(&mut conn)?;

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
            .map(|r| RestaurantInfo::get(&r.1).unwrap())
            .collect::<Vec<(
                RestaurantInfo,
                Vec<CategoryType>,
                Vec<MenuItem>,
                RestaurantLocation,
            )>>();
        /*
        serde_json::json!({
            "location": {
                "longitude": r.0.longitude,
                "latitude": r.0.latitude
            },
            "restaurant": r.1
        })
        */
        Ok(a)
    }
}
