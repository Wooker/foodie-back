#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::{
    db::connection,
    errors::CustomError,
    schema::user_info,
    schema::user_info::dsl::*,
};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = user_info)]
pub struct UserInfo {
    #[serde(rename = "login")]
    Login: String,
    #[serde(rename = "password")]
    Password: Option<String>,
}

impl UserInfo {
    pub fn password_empty(&self) -> bool {
        self.Password.is_none()
    }

    pub fn register(user: UserInfo) -> Result<String, CustomError> {
        let mut conn = connection()?;
        diesel::insert_into(user_info)
            .values(&user)
            .execute(&mut conn)?;

        Ok(user.Login)
    }
}
