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
    reservations (id) {
        id -> Uuid,
        table_id -> Uuid,
        user_id -> Varchar,
        reservation_start -> Timestamp,
        reservation_end -> Timestamp,
        personas -> Int2,
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
        opening_hours -> Varchar,
        price_range -> Int2,
        rating -> Float4,
        image_url -> Text,
        contact -> Text,
    }
}

diesel::table! {
    restaurant_location (restaurant_info_id, longitude, latitude) {
        restaurant_info_id -> Uuid,
        longitude -> Float4,
        latitude -> Float4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Category;

    restaurant_menu (restaurant_info_id, id) {
        id -> Uuid,
        restaurant_info_id -> Uuid,
        name -> Varchar,
        menu_category -> Category,
        price -> Int4,
        ingredients -> Text,
        image_url -> Text,
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

    restaurant_tables (id) {
        id -> Uuid,
        restaurant_info_id -> Uuid,
        seats -> Int2,
        status -> Tablestatus,
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
    user_favorites (user_info_id, restaurant_info_id) {
        user_info_id -> Varchar,
        restaurant_info_id -> Uuid,
    }
}

diesel::table! {
    user_info (id) {
        id -> Varchar,
        password -> Nullable<Varchar>,
        full_name -> Varchar,
        phone_number -> Varchar,
        image_url -> Text,
    }
}

diesel::joinable!(reservations -> restaurant_tables (table_id));
diesel::joinable!(reservations -> user_info (user_id));
diesel::joinable!(restaurant_category -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_city -> city (city_id));
diesel::joinable!(restaurant_city -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_location -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_menu -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_rating -> restaurant_info (restaurant_info_id));
diesel::joinable!(restaurant_rating -> user_info (user_info_id));
diesel::joinable!(restaurant_tables -> restaurant_info (restaurant_info_id));
diesel::joinable!(user_comment -> restaurant_info (restaurant_info_id));
diesel::joinable!(user_comment -> user_info (user_info_id));
diesel::joinable!(user_favorites -> restaurant_info (restaurant_info_id));

diesel::allow_tables_to_appear_in_same_query!(
    city,
    reservations,
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
