use super::schema::users;
use diesel::{Queryable, Insertable};
use serde::Deserialize;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginUser {
    pub username: String,
    pub password: String,
}