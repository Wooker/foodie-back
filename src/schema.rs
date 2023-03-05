// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "category"))]
    pub struct Category;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tablestatus"))]
    pub struct Tablestatus;
}

diesel::table! {
    city (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Category;

    restaurant_category (restaurant_info_id, category_type) {
        restaurant_info_id -> Uuid,
        category_type -> Category,
    }
}

diesel::table! {
    restaurant_city (restaurant_info_id, city_id) {
        restaurant_info_id -> Uuid,
        city_id -> Uuid,
    }
}

diesel::table! {
    restaurant_info (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        address -> Varchar,
        openHours -> Varchar,
        averagePrice -> Nullable<Numeric>,
        images -> Nullable<Text>,
    }
}

diesel::table! {
    restaurant_location (restaurant_info_id) {
        restaurant_info_id -> Uuid,
        longitude -> Nullable<Float8>,
        latitude -> Nullable<Float8>,
    }
}

diesel::table! {
    restaurant_menu (restaurant_info_id) {
        restaurant_info_id -> Uuid,
        food_name -> Nullable<Varchar>,
        price -> Nullable<Numeric>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    restaurant_rating (restaurant_info_id, user_info_id) {
        restaurant_info_id -> Uuid,
        user_info_id -> Varchar,
        rating -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tablestatus;

    restaurant_tables (RestaurantID, TableNumber) {
        RestaurantID -> Uuid,
        TableNumber -> Int2,
        Seats -> Nullable<Int2>,
        Status -> Nullable<Tablestatus>,
    }
}

diesel::table! {
    user_comment (restaurant_info_id, user_info_id) {
        restaurant_info_id -> Uuid,
        user_info_id -> Varchar,
        comment -> Text,
    }
}

diesel::table! {
    user_favorites (user_info_id) {
        user_info_id -> Varchar,
        restaurant_info_id -> Uuid,
    }
}

diesel::table! {
    user_info (id) {
        id -> Varchar,
        password -> Nullable<Varchar>,
    }
}

diesel::joinable!(restaurant_category -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_city -> city (city_id));
diesel::joinable!(restaurant_city -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_location -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_menu -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_rating -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_rating -> user_info (user_info_id));
diesel::joinable!(user_comment -> restaurant_info (restaurant_info_id));
diesel::joinable!(user_comment -> user_info (user_info_id));
diesel::joinable!(user_favorites -> restaurant_info (restaurant_info_id));

diesel::allow_tables_to_appear_in_same_query!(
    city,
    restaurant_category,
    restaurant_city,
    restaurant_info,
    restaurant_location,
    restaurant_menu,
    restaurant_rating,
    restaurant_tables,
    user_comment,
    user_favorites,
    user_info,
);
