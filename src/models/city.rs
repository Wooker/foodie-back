use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{db::connection, errors::CustomError, schema::city::dsl::*};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = city)]
pub struct City {
    id: uuid::Uuid,
    name: String,
}

impl City {
    pub fn find_all() -> Result<Value, CustomError> {
        let mut conn = connection()?;
        let cities = city.get_results::<City>(&mut conn).unwrap();

        let value = serde_json::to_value(cities).unwrap();

        Ok(value)
    }
}
