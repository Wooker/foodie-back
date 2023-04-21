#![allow(non_snake_case)]

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::connection,
    errors::CustomError,
    models::{
        menu_item::MenuItem, restaurant_category::RestaurantCategory,
        restaurant_location::RestaurantLocation, types::CategoryType,
    },
    schema::restaurant_info,
    schema::restaurant_info::dsl::*,
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

    pub fn get_all_ids() -> Result<Vec<Uuid>, CustomError> {
        let mut conn = connection()?;
        let ids = restaurant_info
            .select(restaurant_info::id)
            .get_results::<Uuid>(&mut conn)
            .unwrap();

        Ok(ids)
    }

    pub fn get(
        search_id: &Uuid,
    ) -> Result<
        (
            RestaurantInfo,
            Vec<CategoryType>,
            Vec<MenuItem>,
            RestaurantLocation,
        ),
        CustomError,
    > {
        let mut conn = connection()?;
        let restaurants = restaurant_info
            .get_results::<RestaurantInfo>(&mut conn)
            .unwrap();

        let restaurant = restaurants
            .into_iter()
            .find(|i| i.get_id().eq(&search_id))
            .unwrap();

        let categories = RestaurantCategory::of_restaurant(&restaurant)?;
        let menu: Vec<MenuItem> = MenuItem::of_restaurant(&restaurant)?;
        let location: RestaurantLocation = RestaurantLocation::of_restaurant(&restaurant)?;

        Ok((restaurant, categories, menu, location))
    }
}
