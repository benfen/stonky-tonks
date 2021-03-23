use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::holdings_models::StockHolding;
use crate::stockprice_models::StockPrice;
use crate::user_models::User;

use super::schema::balancehistory;

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name = "balancehistory"]
#[belongs_to(User, foreign_key = "userid")]
pub struct BalanceHistory {
    pub id: String,
    pub userid: String,
    pub capital: i64,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name = "balancehistory"]
pub struct ModBalanceHistory<'a> {
    id: String,
    userid: &'a str,
    capital: i64,
}

impl BalanceHistory {
    pub fn retrieve_history(id: &str, connection: &SqliteConnection) -> Option<Self> {
        use super::schema::balancehistory::dsl::balancehistory;

        balancehistory
            .find(id)
            .first(connection)
            .optional()
            .expect("Error retrieving balance history")
    }
}

impl<'a> ModBalanceHistory<'a> {
    pub fn create_history(user: &'a User, connection: &SqliteConnection) -> BalanceHistory {
        let new_history = Self {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            userid: &user.id,
            capital: calculate_user_capital(user, connection),
        };

        connection
            .transaction::<BalanceHistory, Error, _>(move || {
                diesel::insert_into(balancehistory::table)
                    .values(&new_history)
                    .execute(connection)
                    .expect("Error creating new stock buy offer");

                // We just inserted this record, so there's no reason for it not to exist
                Ok(BalanceHistory::retrieve_history(&new_history.id, connection).unwrap())
            })
            .unwrap()
    }
}

fn calculate_user_capital(user: &User, connection: &SqliteConnection) -> i64 {
    let mut capital = user.capital;

    for priced_holding in StockHolding::retrieve_all_holdings_with_price(connection, user).iter() {
        let quantity = priced_holding.0.get_quantity();
        let price = priced_holding.1.price;

        capital += (quantity as i64) * (price as i64);
    }

    capital
}
