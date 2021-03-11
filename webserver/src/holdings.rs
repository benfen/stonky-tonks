use actix_web::{Error, HttpResponse, Scope, get, post, web };
use actix_web::error::{ ErrorBadRequest, ErrorUnauthorized };
use db::{ establish_connection, perform_transation };
use db::holdings::{ ModStockHoldings, StockHolding};
use db::price::{ StockPrice };
use serde::{ Deserialize, Serialize };

use crate::user::UserInfo;

#[derive(Debug, Deserialize, Serialize)]
struct Holding {
    ticker: String,
    quantity: i32,
}

pub fn holdings_service(path: &str) -> Scope {
    web::scope(path)
        .service(get_holdings)
        .service(create_holding)
}

#[get("")]
async fn get_holdings(user_info: UserInfo) -> Result<HttpResponse, Error> {
    if let Some(user) = user_info.get_user() {
        let connection = establish_connection();

        let holdings = StockHolding::retrieve_holdings(&connection, user);

        Ok(HttpResponse::Ok().json(holdings))
    } else {
        Err(ErrorUnauthorized("User required for request"))
    }
}

#[post("/create")]
async fn create_holding(user_info: UserInfo, new_holding: web::Json<Holding>) -> Result<HttpResponse, Error> {
    if let Some(user) = user_info.get_user() {
        let connection = establish_connection();

        let price_option = StockPrice::retrieve_price(&connection, &new_holding.ticker);

        if let Some(price) = price_option {
            let cost = (price.price as i64) * (new_holding.quantity as i64);

            if cost > user.capital {
                Err(ErrorBadRequest("Purchase request exceeds available capital"))
            } else {
                let holding = perform_transation::<String, _>(&connection, || {
                    user.update_capital(user.capital - cost, &connection);

                    Ok(ModStockHoldings::create_new_holding(user, &new_holding.ticker, new_holding.quantity, &connection))
                }).unwrap();
    
                Ok(HttpResponse::Ok().json(holding))
            }
        } else {
            Err(ErrorBadRequest(format!("Stock ticker does not exist: {}", &new_holding.ticker)))
        }

        // let holding = ModStockHoldings::create_new_holding(user, &holding.ticker, holding.quantity, &connection);

        // Ok(HttpResponse::Ok().json(holding))
    } else {
        Err(ErrorUnauthorized("User required for request"))
    }
}
