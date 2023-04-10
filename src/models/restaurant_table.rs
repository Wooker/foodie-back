use diesel::{
    prelude::*,
    sql_types::{Integer, SmallInt},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::connection,
    errors::CustomError,
    models::{restaurant_info::RestaurantInfo, types::TableStatus},
    schema::restaurant_tables,
    schema::restaurant_tables::dsl::*,
};

#[derive(Debug, Serialize, Associations, Identifiable, Deserialize, Selectable, Queryable)]
#[diesel(belongs_to(RestaurantInfo))]
#[diesel(primary_key(restaurant_info_id, table_id))]
#[diesel(table_name = restaurant_tables)]
pub struct RestaurantTable {
    restaurant_info_id: Uuid,
    table_id: i16,
    seats: i16,
    status: TableStatus,
}

impl RestaurantTable {
    pub fn of_restaurant(restaurant: &RestaurantInfo) -> Result<Vec<RestaurantTable>, CustomError> {
        let mut conn = connection()?;
        let tables: Vec<RestaurantTable> = RestaurantTable::belonging_to(restaurant)
            .select((restaurant_info_id, table_id, seats, status))
            .load(&mut conn)?;

        Ok(tables)
    }

    /*
    pub fn add(favorite: UserFavorite) -> Result<(), CustomError> {
        let mut conn = connection()?;
        diesel::insert_into(user_favorites)
            .values(&favorite)
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn delete(favorite: UserFavorite) -> Result<(), CustomError> {
        let mut conn = connection()?;
        diesel::delete(
            user_favorites.filter(
                user_info_id
                    .eq(favorite.get_user_id())
                    .and(restaurant_info_id.eq(favorite.get_restaurant_id())),
            ),
        )
        .execute(&mut conn)?;

        Ok(())
    }
    */
}
