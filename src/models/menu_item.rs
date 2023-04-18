use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::types::CategoryType, schema::restaurant_menu, RestaurantInfo};

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

impl MenuItem {}
