use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };
use ::uuid::Uuid;

use super::schema::user::dsl::*;
use super::schema::user;

#[derive(Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name="user"]
pub struct User {
    pub name: String,
    pub capital: i64,
    pub id: String,
}

#[derive(Debug, Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub capital: i64,
    pub id: &'a str,
}

impl User {
    pub fn retrieve_user(connection: &SqliteConnection, search_name: &str) -> Option<User> {
        user::table.select((name, capital, id))
            .filter(name.eq(search_name))
            .first(connection)
            .optional()
            .expect("Error loading user from table")
    }

    pub fn retrieve_all_users(connection: &SqliteConnection) -> Vec<User> {
        user::table.select((name, capital, id))
            .load(connection)
            .expect("Error loading users from table")
    }

    pub fn update_capital(&self, new_capital: i64,connection: &SqliteConnection) {
        diesel::update(self)
            .set(capital.eq(new_capital))
            .execute(connection)
            .expect("Error updating captial for user");
    }
}

impl <'a> NewUser<'a> {

    pub fn insert_user(new_name: &str, new_capital: i64, connection: &SqliteConnection) -> String {
        let new_uuid = Uuid::new_v4().to_hyphenated().to_string();
        let new_user = NewUser {
            name: new_name,
            capital: new_capital,
            id: &new_uuid
        };

        diesel::insert_into(user::table)
            .values(new_user)
            .execute(connection)
            .expect("Error creating new user");

        new_uuid
    }
}

