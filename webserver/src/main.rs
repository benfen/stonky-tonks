use actix_web::dev::Payload;
use actix_web::{HttpRequest, error};
use std::future::Future;
use std::pin::Pin;
use serde::{ Serialize };
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::{ get, HttpResponse, Result, post };
use qstring::QString;

use db::{ establish_connection, NewUser, StockPrice, User };

#[derive(Debug, Serialize)]
struct UserInfo {
    user: Option<User>,
    username: String,
}

impl FromRequest for UserInfo {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<UserInfo, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string();
        let parsed_query_string = QString::from(query_string);
        let connection = establish_connection();
        let username = parsed_query_string.get("username").unwrap_or("").to_string();
        let user = User::retrieve_user(&connection, &username);

        Box::pin(async move {
            Ok(UserInfo {
                user,
                username
            })
        })
    }
}

#[get("/prices")]
async fn get_prices() -> Result<HttpResponse> {
    let connection = establish_connection();

    let prices = StockPrice::retrieve(&connection);

    Ok(HttpResponse::Ok().json(prices))
}

#[get("/user")]
async fn get_user(user: UserInfo) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse> {
    let connection = establish_connection();

    Ok(HttpResponse::Ok().json(
        User::retrieve_all_users(&connection)
    ))
}

#[post("/user")]
async fn post_user(user: UserInfo) -> Result<HttpResponse> {
    if user.username.is_empty() {
        Err(error::ErrorBadRequest("no username provided"))
    } else {
        let connection = establish_connection();
        NewUser::insert_user(&user.username, 10000000, &connection);
        Ok(HttpResponse::Ok().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(get_prices)
            .service(get_user)
            .service(get_users)
            .service(post_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}