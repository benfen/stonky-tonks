use chrono::NaiveDateTime;
use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

use crate::user_models::User;

use super::schema::transactions;

pub enum TransactionKind {
    BuyStock,
    SellStock
}

impl TransactionKind {
    fn get_name(&self) -> &'static str {
        match *self {
            TransactionKind::BuyStock => "BuyStock",
            TransactionKind::SellStock => "SellStock"
        } 
    }
}

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name="transactions"]
#[belongs_to(User, foreign_key="userid")]
pub struct Transaction {
    pub id: String,
    pub userid: String,
    pub stockid: String,
    pub quantity: i32,
    pub kind: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name="transactions"]
pub struct ModTransaction<'a> {
    pub id: String,
    pub userid: &'a str,
    pub stockid: &'a str,
    pub quantity: i32,
    pub kind: &'a str,
}

impl Transaction {

    pub fn retrieve_all(user: &User, connection: &SqliteConnection) -> Vec<Self> {
        Transaction::belonging_to(user)
            .load::<Transaction>(connection)
            .expect("Error loading holdings from table")
    }
}

impl<'a> ModTransaction<'a>  {

    pub fn record_transaction(user: &'a User, ticker: &'a str, quantity: u32, kind: TransactionKind, connection: &SqliteConnection) -> ModTransaction<'a> {

        let new_transaction = ModTransaction {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            userid: &user.id,
            stockid: ticker,
            quantity: quantity as i32,
            kind: kind.get_name()
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(connection)
            .expect("Error creating new transaction");

        new_transaction
    }
}