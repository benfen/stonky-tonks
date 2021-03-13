use actix_web::{ Error, HttpResponse, Scope, get, post, web };
use actix_web::error::{ ErrorBadRequest };
use actix_web::http::StatusCode;
use db::{ establish_connection, perform_transation };
use db::holdings::{ ModStockHoldings, StockHolding };
use db::transactions::{ ModTransaction, TransactionKind };
use db::price::{ StockPrice };
use serde::{ Deserialize, Serialize };

use crate::user::UserInfo;

#[derive(Debug, Deserialize, Serialize)]
struct Holding {
    ticker: String,
    quantity: u32,
}

pub fn holdings_service(path: &str) -> Scope {
    web::scope(path)
        .service(get_holdings)
        .service(create_holding)
        .service(sell_holding)
}

#[get("")]
async fn get_holdings(user_info: UserInfo) -> Result<HttpResponse, Error> {
    let user = user_info.get_user();
    let connection = establish_connection();

    let holdings = StockHolding::retrieve_all_holdings(&connection, user);

    Ok(HttpResponse::Ok().json(holdings))
}

#[post("/buy")]
async fn create_holding(user_info: UserInfo, new_holding: web::Json<Holding>) -> Result<HttpResponse, Error> {
    let user = user_info.get_user();
    let connection = establish_connection();

    let price_option = StockPrice::retrieve_price(&connection, &new_holding.ticker);

    if let Some(price) = price_option {
        let cost = (price.price as i64) * (new_holding.quantity as i64);

        if cost > user.capital {
            Err(ErrorBadRequest("Purchase request exceeds available capital"))
        } else {
            let existing_holding_opt = StockHolding::retrieve_holding(&connection, &user, &new_holding.ticker);

            if let Some(existing_holding) = existing_holding_opt {
                perform_transation::<(), _>(&connection, || {
                    user.update_capital(user.capital - cost, &connection);

                    ModStockHoldings::update_quantity(&existing_holding, existing_holding.get_quantity() + new_holding.quantity, &connection);

                    ModTransaction::record_transaction(&user, &new_holding.ticker, new_holding.quantity, TransactionKind::BuyStock, &connection);
                    Ok(())
                }).unwrap();
            } else {
                perform_transation::<(), _>(&connection, || {
                    user.update_capital(user.capital - cost, &connection);
    
                    ModStockHoldings::create_new_holding(user, &new_holding.ticker, new_holding.quantity, &connection);

                    ModTransaction::record_transaction(&user, &new_holding.ticker, new_holding.quantity, TransactionKind::BuyStock, &connection);
                    Ok(())
                }).unwrap();
            }

            Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
        }
    } else {
        Err(ErrorBadRequest(format!("Stock ticker does not exist: {}", &new_holding.ticker)))
    }
}

#[post("/sell")]
async fn sell_holding(user_info: UserInfo, holding: web::Json<Holding>) -> Result<HttpResponse, Error> {
    let user = user_info.get_user();
    let connection = establish_connection();

    let price_option = StockPrice::retrieve_price(&connection, &holding.ticker);

    if let Some(price) = price_option {
        let cost = (price.price as i64) * (holding.quantity as i64);

        let existing_holding_opt = StockHolding::retrieve_holding(&connection, &user, &holding.ticker);

        if let Some(existing_holding) = existing_holding_opt {
            if existing_holding.get_quantity() >= holding.quantity {
                perform_transation::<(), _>(&connection, || {
                    user.update_capital(user.capital + cost, &connection);

                    ModStockHoldings::update_quantity(&existing_holding, existing_holding.get_quantity() - holding.quantity, &connection);

                    ModTransaction::record_transaction(&user, &holding.ticker, holding.quantity, TransactionKind::SellStock, &connection);
                    Ok(())
                }).unwrap();

                Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
            } else {
                Err(ErrorBadRequest("User does not own enough stock to make this transaction"))
            }
        } else {
            Err(ErrorBadRequest("User does not own enough stock to make this transaction"))
        }
    } else {
        Err(ErrorBadRequest(format!("Stock ticker does not exist: {}", &holding.ticker)))
    }
}
