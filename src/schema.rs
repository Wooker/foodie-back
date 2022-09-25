// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cuisine"))]
    pub struct Cuisine;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "foodtype"))]
    pub struct Foodtype;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tablestatus"))]
    pub struct Tablestatus;
}

diesel::table! {
    city (ID) {
        ID -> Uuid,
        Name -> Varchar,
    }
}

diesel::table! {
    restaurant_city (RestaurantID, CityID) {
        RestaurantID -> Uuid,
        CityID -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Cuisine;

    restaurant_cuisine (RestaurantID, CuisineType) {
        RestaurantID -> Uuid,
        CuisineType -> Cuisine,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Foodtype;

    restaurant_foodtype (RestaurantID, FoodType) {
        RestaurantID -> Uuid,
        FoodType -> Foodtype,
    }
}

diesel::table! {
    restaurant_info (ID) {
        ID -> Uuid,
        Name -> Varchar,
        Description -> Text,
        Address -> Varchar,
        OpenHours -> Varchar,
        AveragePrice -> Nullable<Numeric>,
        Images -> Nullable<Text>,
    }
}

diesel::table! {
    restaurant_location (RestaurantID) {
        RestaurantID -> Uuid,
        longitude -> Nullable<Float8>,
        latitude -> Nullable<Float8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Foodtype;

    restaurant_menu (RestaurantID) {
        RestaurantID -> Uuid,
        FoodName -> Nullable<Varchar>,
        FoodType -> Nullable<Foodtype>,
        Price -> Nullable<Numeric>,
        Description -> Nullable<Text>,
    }
}

diesel::table! {
    restaurant_rating (RestaurantID, User) {
        RestaurantID -> Uuid,
        User -> Varchar,
        Rating -> Int2,
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
    user_comment (RestaurantID, User) {
        RestaurantID -> Uuid,
        User -> Varchar,
        Comment -> Text,
    }
}

diesel::table! {
    user_info (Login) {
        Login -> Varchar,
        Password -> Nullable<Varchar>,
    }
}

diesel::joinable!(restaurant_city -> city (CityID));
diesel::joinable!(restaurant_city -> restaurant_info (RestaurantID));
diesel::joinable!(restaurant_cuisine -> restaurant_info (RestaurantID));
diesel::joinable!(restaurant_foodtype -> restaurant_info (RestaurantID));
diesel::joinable!(restaurant_location -> restaurant_info (RestaurantID));
diesel::joinable!(restaurant_rating -> restaurant_info (RestaurantID));
diesel::joinable!(restaurant_rating -> user_info (User));
diesel::joinable!(user_comment -> restaurant_info (RestaurantID));
diesel::joinable!(user_comment -> user_info (User));

diesel::allow_tables_to_appear_in_same_query!(
    city,
    restaurant_city,
    restaurant_cuisine,
    restaurant_foodtype,
    restaurant_info,
    restaurant_location,
    restaurant_menu,
    restaurant_rating,
    restaurant_tables,
    user_comment,
    user_info,
);
