use self::CategoryType::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, diesel_derive_enum::DbEnum,
)]
#[ExistingTypePath = "crate::schema::sql_types::Category"]
#[DbValueStyle = "PascalCase"]
pub enum CategoryType {
    FastFood,
    Ramen,
    Italian,
    Japanese,
}

impl CategoryType {
    pub fn iter() -> impl Iterator<Item = CategoryType> {
        [FastFood, Ramen, Italian, Japanese].iter().copied()
    }
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Tablestatus"]
#[DbValueStyle = "PascalCase"]
pub enum TableStatus {
    Free,
    Occupied,
}

#[derive(Deserialize)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}
