use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    db::connection,
    errors::CustomError,
    models::{
        menu_item::MenuItem, restaurant_info::RestaurantInfo,
        restaurant_location::RestaurantLocation, types::CategoryType,
    },
    schema::restaurant_category,
    schema::restaurant_info,
};

#[derive(Debug, Serialize, Associations, Identifiable, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(primary_key(restaurant_info_id))]
#[diesel(table_name = restaurant_category)]
pub struct RestaurantCategory {
    restaurant_info_id: Uuid,
    category_type: CategoryType,
    image_url: String,
}

impl RestaurantCategory {
    pub fn get_all() -> Result<
        HashMap<
            (CategoryType, String),
            Vec<(
                RestaurantInfo,
                Vec<CategoryType>,
                Vec<MenuItem>,
                RestaurantLocation,
            )>,
        >,
        CustomError,
    > {
        let mut conn = connection()?;
        let mut categories: HashMap<
            (CategoryType, String),
            Vec<(
                RestaurantInfo,
                Vec<CategoryType>,
                Vec<MenuItem>,
                RestaurantLocation,
            )>,
        > = HashMap::new();

        for category in CategoryType::iter() {
            let cats_ids: Vec<(RestaurantCategory, Uuid)> = restaurant_category::table
                .inner_join(restaurant_info::table)
                .filter(restaurant_category::category_type.eq(category))
                .select((RestaurantCategory::as_select(), restaurant_info::id))
                .load::<(RestaurantCategory, Uuid)>(&mut conn)?;

            let mut restaurants = vec![];
            for cat_id in cats_ids.iter() {
                restaurants.push(RestaurantInfo::get(&cat_id.1).unwrap())
            }

            categories.insert(
                (category, cats_ids[0].0.image_url.clone()),
                restaurants, //ids.into_iter().map(|result_join| result_join.1).collect(),
            );
        }

        Ok(categories)
    }

    pub fn of_restaurant(restaurant: &RestaurantInfo) -> Result<Vec<CategoryType>, CustomError> {
        let mut conn = connection()?;

        let categories: Vec<CategoryType> = RestaurantCategory::belonging_to(restaurant)
            .select(restaurant_category::category_type)
            .load(&mut conn)
            .unwrap();

        Ok(categories)
    }
}
