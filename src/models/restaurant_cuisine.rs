#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use serde_json::Value;

use crate::{
    db::connection,
    errors::CustomError,
    schema::restaurant_cuisine,
    schema::restaurant_cuisine::dsl::*,
    models::types::CuisineType,
};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = restaurant_cuisine)]
pub struct RestaurantCuisine {
    ID: uuid::Uuid,
    Cuisine: CuisineType,
}

impl RestaurantCuisine {
    pub fn get_all() -> Result<Vec<RestaurantCuisine>, CustomError> {
        let mut conn = connection()?;
        let rests = restaurant_cuisine.get_results::<RestaurantCuisine>(&mut conn).unwrap();

        Ok(rests)
    }

    pub fn of_restaurant(id: uuid::Uuid) -> Result<Value, CustomError> {
        let mut conn = connection()?;
        let result = restaurant_cuisine
            .filter(restaurant_cuisine::RestaurantID.eq(id))
            .select(restaurant_cuisine::CuisineType)
            .get_results::<CuisineType>(&mut conn).unwrap();

        Ok(serde_json::to_value(result).unwrap())
    }
}
