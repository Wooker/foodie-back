#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use serde_json::Value;

use crate::{
    db::connection,
    errors::CustomError,
    schema::restaurant_category,
    schema::restaurant_category::dsl::*,
    models::types::CategoryType,
};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = restaurant_category)]
pub struct RestaurantCategory {
    ID: uuid::Uuid,
    Category: CategoryType,
}

impl RestaurantCategory {
    pub fn get_all() -> Result<Vec<RestaurantCategory>, CustomError> {
        let mut conn = connection()?;
        let rests = restaurant_category.get_results::<RestaurantCategory>(&mut conn).unwrap();

        Ok(rests)
    }

    pub fn of_restaurant(id: uuid::Uuid) -> Result<Value, CustomError> {
        let mut conn = connection()?;
        let result = restaurant_category
            .filter(restaurant_category::RestaurantID.eq(id))
            .select(restaurant_category::CategoryType)
            .get_results::<CategoryType>(&mut conn).unwrap();

        Ok(serde_json::to_value(result).unwrap())
    }
}
