use crate::{
    db::connection,
    errors::CustomError,
    schema::reservations::dsl::*,
    schema::{reservations, restaurant_tables, restaurant_tables::dsl::*},
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::TableStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = reservations)]
pub struct Reservation {
    restaurant_info_id: Uuid,
    table_id: i16,
    user_id: String,
}

impl Reservation {
    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_restaurant_info_id(&self) -> Uuid {
        self.restaurant_info_id
    }

    pub fn get_table_id(&self) -> i16 {
        self.table_id
    }

    pub fn get_all(login: &String) -> Result<Vec<(Uuid, i16)>, CustomError> {
        let mut conn = connection()?;
        let result = reservations
            .filter(user_id.eq(login))
            .select((reservations::restaurant_info_id, reservations::table_id))
            .load::<(Uuid, i16)>(&mut conn)
            .unwrap();

        Ok(result)
    }

    pub fn add(reservation: Reservation) -> Result<(), CustomError> {
        let mut conn = connection()?;
        diesel::insert_into(reservations)
            .values(&reservation)
            .execute(&mut conn)?;

        diesel::update(restaurant_tables)
            .filter(restaurant_tables::restaurant_info_id.eq(reservation.restaurant_info_id))
            .filter(restaurant_tables::table_id.eq(reservation.table_id))
            .set(status.eq(TableStatus::Occupied))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn delete(reservation: Reservation) -> Result<(), CustomError> {
        let mut conn = connection()?;
        diesel::delete(
            reservations.filter(
                user_id
                    .eq(reservation.get_user_id())
                    .and(reservations::restaurant_info_id.eq(reservation.get_restaurant_info_id()))
                    .and(reservations::table_id.eq(reservation.get_table_id())),
            ),
        )
        .execute(&mut conn)?;

        diesel::update(restaurant_tables)
            .filter(restaurant_tables::restaurant_info_id.eq(reservation.restaurant_info_id))
            .filter(restaurant_tables::table_id.eq(reservation.table_id))
            .set(status.eq(TableStatus::Free))
            .execute(&mut conn)?;

        Ok(())
    }
}
