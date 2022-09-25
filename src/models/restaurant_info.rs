#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use serde_json::Value;
use bigdecimal::BigDecimal;

use crate::{
    db::connection,
    errors::CustomError,
    schema::restaurant_info::dsl::*,
};


#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = restaurant_info)]
pub struct RestaurantInfo {
    ID: uuid::Uuid,
    Name: String,
    Description: String,
    Address: String,
    OpenHours: String,
    AveragePrice: Option<BigDecimal>,
    Images: Option<String>,
}

impl RestaurantInfo {
    pub fn get_all() -> Result<Value, CustomError> {
        let mut conn = connection()?;
        let rests = restaurant_info.get_results::<RestaurantInfo>(&mut conn).unwrap();

        Ok(serde_json::to_value(rests).unwrap())
    }
}
