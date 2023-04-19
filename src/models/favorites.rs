#![allow(non_snake_case)]

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::connection, errors::CustomError, schema::user_favorites, schema::user_favorites::dsl::*,
    RestaurantInfo,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = user_favorites)]
pub struct UserFavorite {
    #[serde(rename = "login")]
    user_info_id: String,
    #[serde(rename = "restaurant_id")]
    restaurant_info_id: Uuid,
}

impl UserFavorite {
    pub fn get_user_id(&self) -> &String {
        &self.user_info_id
    }

    pub fn get_restaurant_id(&self) -> Uuid {
        self.restaurant_info_id
    }

    pub fn get_all(login: &String) -> Result<Vec<RestaurantInfo>, CustomError> {
        let mut conn = connection()?;
        let ids = user_favorites
            .filter(user_info_id.eq(login))
            .select(restaurant_info_id)
            .load::<Uuid>(&mut conn)
            .unwrap();

        let mut result = vec![];
        for id in ids.iter() {
            result.push(RestaurantInfo::get(id)?);
        }

        Ok(result)
    }

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
}
