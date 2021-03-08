use actix_web::{Error, HttpResponse, Scope, get, web };
use actix_web::error::ErrorUnauthorized;
use db::establish_connection;
use db::holdings::StockHolding;

use crate::user::UserInfo;

pub fn holdings_service(path: &str) -> Scope {
    web::scope(path)
        .service(get_holdings)
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
