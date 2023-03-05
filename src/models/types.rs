use self::CategoryType::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Category"]
#[DbValueStyle = "PascalCase"]
pub enum CategoryType {
  FastFood,
  Ramen,
  Italian,
  Japanese
}

impl CategoryType {
    pub fn iter() -> impl Iterator<Item = CategoryType> {
        [FastFood, Ramen, Italian, Japanese].iter().copied()
    }
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Tablestatus"]
#[DbValueStyle = "PascalCase"]
pub enum TableStatus {
    Free,
    Occupied
}
