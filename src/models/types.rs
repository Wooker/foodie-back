use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Cuisine"]
#[DbValueStyle = "PascalCase"]
pub enum CuisineType {
    European,
    Japanese,
    Chinese,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Foodtype"]
#[DbValueStyle = "PascalCase"]
pub enum FoodType {
    Fastfood,
    Ramen
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Tablestatus"]
#[DbValueStyle = "PascalCase"]
pub enum TableStatus {
    Free,
    Occupied
}
