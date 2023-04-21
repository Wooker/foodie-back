use crate::{
    db::connection,
    errors::CustomError,
    models::{
        menu_item::MenuItem, restaurant_info::RestaurantInfo,
        restaurant_location::RestaurantLocation, restaurant_table::RestaurantTable, types::*,
        user_info::UserReservation,
    },
    schema::{reservations, restaurant_tables, restaurant_tables::dsl::*},
    schema::{reservations::dsl::*, restaurant_info},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(belongs_to(RestaurantTable))]
#[diesel(belongs_to(UserInfo))]
#[diesel(table_name = reservations)]
pub struct Reservation {
    id: Uuid,
    table_id: Uuid,
    user_id: String,
    personas: i16,
    reservation_start: NaiveDateTime,
    reservation_end: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationMask {
    table_id: Uuid,
    user_id: String,
    personas: i16,
    reservation_start: NaiveDateTime,
    reservation_end: NaiveDateTime,
}

impl Reservation {
    pub fn new(mask: ReservationMask) -> Self {
        Self {
            id: Uuid::new_v4(),
            table_id: mask.table_id,
            user_id: mask.user_id,
            personas: mask.personas,
            reservation_start: mask.reservation_start,
            reservation_end: mask.reservation_end,
        }
    }
    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_table_id(&self) -> Uuid {
        self.table_id
    }

    pub fn get_all(
        login: &String,
    ) -> Result<
        Vec<(
            (
                RestaurantInfo,
                Vec<CategoryType>,
                Vec<MenuItem>,
                RestaurantLocation,
            ),
            Self,
        )>,
        CustomError,
    > {
        let mut conn = connection()?;
        let mut restaurant_reservation = vec![];

        let reservations_result: Vec<Reservation> = reservations::table
            .filter(user_id.eq(login))
            .select(Reservation::as_select())
            .load::<Reservation>(&mut conn)?;

        for reservation in reservations_result {
            let table = restaurant_tables::table
                .filter(restaurant_tables::id.eq(reservation.table_id))
                .select(RestaurantTable::as_select())
                .get_result::<RestaurantTable>(&mut conn)?;

            let restaurant_id = restaurant_info::table
                .filter(restaurant_info::id.eq(table.restaurant_info_id))
                .select(restaurant_info::id)
                .get_result::<Uuid>(&mut conn)?;

            let restaurant = RestaurantInfo::get(&restaurant_id)?;

            restaurant_reservation.push((restaurant, reservation));
        }

        Ok(restaurant_reservation)
    }

    pub fn add(reservation: Reservation) -> Result<(), CustomError> {
        let mut conn = connection()?;
        diesel::insert_into(reservations)
            .values(&reservation)
            .execute(&mut conn)
            .expect("Insertion error");

        diesel::update(restaurant_tables)
            .filter(restaurant_tables::id.eq(reservation.table_id))
            .set(status.eq(TableStatus::Occupied))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn delete(user_reservation: UserReservation) -> Result<(), CustomError> {
        let mut conn = connection()?;

        let before = reservations.count().first::<i64>(&mut conn);

        let res: Reservation = reservations::table
            .filter(reservations::id.eq(user_reservation.reservation_id))
            .select(Reservation::as_select())
            .get_result::<Reservation>(&mut conn)?;

        diesel::delete(reservations::table.filter(reservations::id.eq(res.id)))
            .execute(&mut conn)?;

        let after = reservations.count().first::<i64>(&mut conn);

        // dbg!(&reservations
        //     .filter(reservations::user_id.ne("qwe"))
        //     .select(Reservation::as_select())
        //     .load::<Reservation>(&mut conn)?);

        if before == after {
            Err(CustomError::NotFound)
        } else {
            let table = restaurant_tables::table
                .filter(restaurant_tables::id.eq(user_reservation.table_id))
                .select(RestaurantTable::as_select())
                .get_result::<RestaurantTable>(&mut conn)?;

            diesel::update(restaurant_tables::table.filter(restaurant_tables::id.eq(table.id)))
                .set(status.eq(TableStatus::Free))
                .execute(&mut conn)?;

            Ok(())
        }
    }
}
