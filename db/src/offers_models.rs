use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user_models::User;

use super::schema::stockbuyoffers;
use super::schema::stockselloffers;

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name = "stockbuyoffers"]
#[belongs_to(User, foreign_key = "userid")]
pub struct StockBuyOffer {
    pub id: String,
    pub userid: String,
    pub stockid: String,
    pub quantity: i32,
    pub price: i32,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name = "stockbuyoffers"]
pub struct ModStockBuyOffer<'a> {
    id: String,
    userid: &'a str,
    stockid: &'a str,
    quantity: i32,
    price: i32,
}

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name = "stockselloffers"]
#[belongs_to(User, foreign_key = "userid")]
pub struct StockSellOffer {
    pub id: String,
    pub userid: String,
    pub stockid: String,
    pub quantity: i32,
    pub price: i32,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name = "stockselloffers"]
pub struct ModStockSellOffer<'a> {
    id: String,
    userid: &'a str,
    stockid: &'a str,
    quantity: i32,
    price: i32,
}

impl StockBuyOffer {
    pub fn delete_offer(&self, connection: &SqliteConnection) {
        diesel::delete(self)
            .execute(connection)
            .expect("Error deleting stock buy offer");
    }

    pub fn retrieve_all_offers(user: &User, connection: &SqliteConnection) -> Vec<Self> {
        Self::belonging_to(user)
            .load::<Self>(connection)
            .expect("Error loading buy offers from table")
    }

    pub fn retrieve_stock_offers(
        user: &User,
        ticker: &str,
        connection: &SqliteConnection,
    ) -> Vec<Self> {
        use super::schema::stockbuyoffers::dsl::*;

        stockbuyoffers
            .filter(userid.eq(&user.id))
            .filter(stockid.eq(ticker))
            .load::<Self>(connection)
            .expect("Error retrieving buy offers for user and stock")
    }

    pub fn retrieve_offer(id: &str, connection: &SqliteConnection) -> Option<Self> {
        use super::schema::stockbuyoffers::dsl::stockbuyoffers;

        stockbuyoffers
            .find(id)
            .first(connection)
            .optional()
            .expect("Error retrieving offer")
    }

    pub fn update_offer_quantity(&self, new_quantity: u32, connection: &SqliteConnection) {
        use super::schema::stockbuyoffers::dsl::*;

        diesel::update(self)
            .set(quantity.eq(new_quantity as i32))
            .execute(connection)
            .expect("Error updating stock buy offer");
    }
}

impl<'a> ModStockBuyOffer<'a> {
    pub fn create_new_offer(
        user: &'a User,
        ticker: &'a str,
        quantity: u32,
        price: u32,
        connection: &SqliteConnection,
    ) -> StockBuyOffer {
        let new_offer = Self {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            userid: &user.id,
            stockid: ticker,
            quantity: quantity as i32,
            price: price as i32,
        };

        connection
            .transaction::<StockBuyOffer, Error, _>(move || {
                diesel::insert_into(stockbuyoffers::table)
                    .values(&new_offer)
                    .execute(connection)
                    .expect("Error creating new stock buy offer");

                // We just inserted this record, so there's no reason for it not to exist
                Ok(StockBuyOffer::retrieve_offer(&new_offer.id, connection).unwrap())
            })
            .unwrap()
    }
}

impl StockSellOffer {
    pub fn delete_offer(&self, connection: &SqliteConnection) {
        diesel::delete(self)
            .execute(connection)
            .expect("Error deleting stock buy offer");
    }

    pub fn retrieve_all_offers(user: &User, connection: &SqliteConnection) -> Vec<Self> {
        Self::belonging_to(user)
            .load::<Self>(connection)
            .expect("Error loading sell offers from table")
    }

    pub fn retrieve_stock_offers(
        user: &User,
        ticker: &str,
        connection: &SqliteConnection,
    ) -> Vec<Self> {
        use super::schema::stockselloffers::dsl::*;

        stockselloffers
            .filter(userid.eq(&user.id))
            .filter(stockid.eq(ticker))
            .load::<Self>(connection)
            .expect("Error retrieving sell offers for user and stock")
    }

    pub fn retrieve_offer(id: &str, connection: &SqliteConnection) -> Option<Self> {
        use super::schema::stockselloffers::dsl::stockselloffers;

        stockselloffers
            .find(id)
            .first(connection)
            .optional()
            .expect("Error retrieving offer")
    }

    pub fn update_offer_quantity(&self, new_quantity: u32, connection: &SqliteConnection) {
        use super::schema::stockselloffers::dsl::*;

        diesel::update(self)
            .set(quantity.eq(new_quantity as i32))
            .execute(connection)
            .expect("Error updating stock buy offer");
    }
}

impl<'a> ModStockSellOffer<'a> {
    pub fn create_new_offer(
        user: &'a User,
        ticker: &'a str,
        quantity: u32,
        price: u32,
        connection: &SqliteConnection,
    ) -> StockSellOffer {
        let new_offer = Self {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            userid: &user.id,
            stockid: ticker,
            quantity: quantity as i32,
            price: price as i32,
        };

        connection
            .transaction::<StockSellOffer, Error, _>(move || {
                diesel::insert_into(stockselloffers::table)
                    .values(&new_offer)
                    .execute(connection)
                    .expect("Error creating new stock buy offer");

                // We just inserted this record, so there's no reason for it not to exist
                Ok(StockSellOffer::retrieve_offer(&new_offer.id, connection).unwrap())
            })
            .unwrap()
    }
}
