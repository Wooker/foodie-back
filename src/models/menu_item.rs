use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::connection,
    errors::CustomError,
    models::{restaurant_info::RestaurantInfo, types::CategoryType},
    schema::restaurant_menu,
};

#[derive(
    Debug, PartialEq, Associations, Identifiable, Selectable, Serialize, Deserialize, Queryable,
)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(primary_key(restaurant_info_id, id))]
#[diesel(table_name = restaurant_menu)]
pub struct MenuItem {
    id: Uuid,
    #[serde(skip)]
    restaurant_info_id: Uuid,
    name: String,
    menu_category: CategoryType,
    price: i32,
    ingredients: String,
    image_url: String,
}

impl MenuItem {
    pub fn of_restaurant(restaurant: &RestaurantInfo) -> Result<Vec<MenuItem>, CustomError> {
        let mut conn = connection()?;

        let menu: Vec<MenuItem> = MenuItem::belonging_to(&restaurant)
            .select(MenuItem::as_select())
            .load(&mut conn)
            .unwrap();

        Ok(menu)
    }
}
