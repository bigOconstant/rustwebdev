extern crate diesel;

use diesel::{Queryable,Insertable};
use chrono::{NaiveDateTime};
 use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub username:  String,
    pub password: String,
    pub email: String,
    pub published_date:  NaiveDateTime,
    pub last_login: NaiveDateTime,
}

#[derive(Debug, Clone,Insertable)]
#[table_name="users"]
pub struct UserInsertable {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

