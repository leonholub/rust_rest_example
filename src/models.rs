use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use diesel;
use diesel::prelude::*;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::DbConn;

use super::schema::users;
use super::schema::vehicles;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub name: String,
    pub coolness: i32,
    pub wattage: i32,
    pub description: Option<String>,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn find(username_: String, connection: &PgConnection) -> Vec<User> {
        let res = users::table
            .filter(users::username.eq(username_))
            .load::<User>(connection)
            .expect("Error loading user");
        res
    }

    pub fn create(user: CreateUser, connection: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)?;

        users::table.order(users::id.desc()).first(connection)
    }
}