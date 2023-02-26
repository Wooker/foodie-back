use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Category"]
#[DbValueStyle = "PascalCase"]
pub enum CategoryType {
    European,
    Japanese,
    Chinese,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Tablestatus"]
#[DbValueStyle = "PascalCase"]
pub enum TableStatus {
    Free,
    Occupied
}
