use actix_web::{Error, HttpResponse, FromRequest, HttpRequest, Scope, error, get, post, web };
use db::{establish_connection, User, NewUser};
use serde::{ Serialize };
use std::pin::Pin;
use std::future::Future;
use actix_web::dev::Payload;
use qstring::QString;

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

pub fn user_service(path: &str) -> Scope {
    web::scope(path)
        .service(get_user)
        .service(get_users)
        .service(post_user)
}

#[get("/user")]
async fn get_user(user: UserInfo) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse, Error> {
    let connection = establish_connection();

    Ok(HttpResponse::Ok().json(
        User::retrieve_all_users(&connection)
    ))
}

#[post("/user")]
async fn post_user(user: UserInfo) -> Result<HttpResponse, Error> {
    if user.username.is_empty() {
        Err(error::ErrorBadRequest("no username provided"))
    } else {
        let connection = establish_connection();
        NewUser::insert_user(&user.username, 10000000, &connection);
        Ok(HttpResponse::Ok().finish())
    }
}