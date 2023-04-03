#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    RestaurantInfo,
    db::connection,
    errors::CustomError,
    schema::restaurant_category,
    schema::restaurant_info,
    models::types::CategoryType,
};

#[derive(Debug, Serialize, Associations, Identifiable, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(primary_key(restaurant_info_id))]
#[diesel(table_name = restaurant_category)]
pub struct RestaurantCategory {
    restaurant_info_id: Uuid,
    category_type: CategoryType,
}

impl RestaurantCategory {
    pub fn get_all() -> Result<HashMap<CategoryType, Vec<RestaurantInfo>>, CustomError> {
        let mut conn = connection()?;
        let mut categories: HashMap<CategoryType, Vec<RestaurantInfo>> = HashMap::new();

        for category in CategoryType::iter() {
            let ids = restaurant_category::table
                .inner_join(restaurant_info::table)
                .filter(restaurant_category::category_type.eq(category)) //.get_results::<RestaurantCategory>(&mut conn)?;
                .select((RestaurantCategory::as_select(), RestaurantInfo::as_select()))
                .load::<(RestaurantCategory, RestaurantInfo)>(&mut conn)?;

            categories.insert(
                category,
                ids
                    .into_iter()
                    .map(|result_join| { result_join.1 })
                    .collect()
            );
        }

        Ok(categories)
    }

    pub fn of_restaurant(other_id: &Uuid) -> Result<Vec<CategoryType>, CustomError> {
        let mut conn = connection()?;
        let categories: Vec<CategoryType> = restaurant_category::table
            .filter(restaurant_category::restaurant_info_id.eq(other_id))
            .select(restaurant_category::category_type)
            .load::<CategoryType>(&mut conn).unwrap();

        Ok(categories)
    }
}
