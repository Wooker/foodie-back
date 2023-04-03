#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::{
    db::connection,
    errors::CustomError,
    schema::user_info,
    schema::user_info::dsl::*,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = user_info)]
pub struct UserInfo {
    #[serde(rename = "login")]
    id: String,
    #[serde(rename = "password")]
    password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    login: String
}

impl UserLogin {
    pub fn get_login(&self) -> &String {
        &self.login
    }
}


impl UserInfo {
    pub fn get_login(&self) -> &String {
        &self.id
    }

    pub fn password_empty(&self) -> bool {
        self.password.is_none()
    }

    pub fn register(user: UserInfo) -> Result<String, CustomError> {
        let mut conn = connection()?;
        diesel::insert_into(user_info)
            .values(&user)
            .execute(&mut conn)?;

        Ok(user.id)
    }

    pub fn check(user: UserInfo) -> Result<(), CustomError> {
        let mut conn = connection()?;
        let user_db = user_info.filter(id.eq(user.id)).first::<UserInfo>(&mut conn)?;

        if user_db.password.eq(&user.password) {
            Ok(())
        } else {
            Err(CustomError::PasswordMismatch)
        }
    }
}
