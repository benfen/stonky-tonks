use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };
use ::uuid::Uuid;

use super::schema::user::dsl::*;
use super::schema::user;

#[derive(Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name="user"]
#[primary_key(uuid)]
pub struct User {
    pub name: String,
    pub capital: i32,
    pub uuid: String,
}

#[derive(Debug, Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub capital: i32,
    pub uuid: &'a str,
}

impl User {
    pub fn retrieve_user(connection: &SqliteConnection, search_name: &str) -> Option<User> {
        user::table.select((name, capital, uuid))
            .filter(name.eq(search_name))
            .first(connection)
            .optional()
            .expect("Error loading user from table")
    }

    pub fn retrieve_all_users(connection: &SqliteConnection) -> Vec<User> {
        user::table.select((name, capital, uuid))
            .load(connection)
            .expect("Error loading users from table")
    }
}

impl <'a> NewUser<'a> {

    pub fn insert_user(new_name: &str, new_capital: i32, connection: &SqliteConnection) -> String {
        let new_uuid = Uuid::new_v4().to_hyphenated().to_string();
        let new_user = NewUser {
            name: new_name,
            capital: new_capital,
            uuid: &new_uuid
        };

        diesel::insert_into(user::table)
            .values(new_user)
            .execute(connection)
            .expect("Error creating new user");

        new_uuid
    }

    pub fn update_capital(&self, connection: &SqliteConnection) {
        diesel::update(user::table.filter(uuid.eq(self.uuid)))
            .set(capital.eq(self.capital))
            .execute(connection)
            .expect("Error updating captial for user");
    }
}

