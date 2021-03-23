use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{error, get, post, web, Error, FromRequest, HttpRequest, HttpResponse, Scope};
use db::establish_connection;
use db::user::{NewUser, User};
use qstring::QString;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    user: User,
}

impl UserInfo {
    pub fn get_user(&self) -> &User {
        &self.user
    }
}

impl FromRequest for UserInfo {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<UserInfo, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string();
        let parsed_query_string = QString::from(query_string);
        let connection = establish_connection();
        let username = parsed_query_string.get("username").unwrap_or("");
        let user_option = User::retrieve_user(&connection, username);

        if let Some(user) = user_option {
            Box::pin(async move { Ok(UserInfo { user }) })
        } else {
            Box::pin(async move { Err(ErrorUnauthorized("User required for request")) })
        }
    }
}

pub fn user_service(path: &str) -> Scope {
    web::scope(path)
        .service(get_user)
        .service(get_users)
        .service(post_user)
}

#[get("/get/{username}")]
async fn get_user(username: web::Path<String>) -> Result<HttpResponse, Error> {
    let connection = establish_connection();

    Ok(HttpResponse::Ok().json(User::retrieve_user(&connection, &username.into_inner())))
}

#[get("/list")]
async fn get_users() -> Result<HttpResponse, Error> {
    let connection = establish_connection();

    Ok(HttpResponse::Ok().json(User::retrieve_all_users(&connection)))
}

#[post("/create/{username}")]
async fn post_user(username: web::Path<String>) -> Result<HttpResponse, Error> {
    let name = username.into_inner();
    if name.is_empty() {
        return Err(error::ErrorBadRequest("No username provided"));
    }

    let connection = establish_connection();

    let user = User::retrieve_user(&connection, &name);

    if user.is_some() {
        Err(error::ErrorBadRequest("Username already exists"))
    } else {
        let connection = establish_connection();
        NewUser::insert_user(&name, 10000000, &connection);
        Ok(HttpResponse::Ok().finish())
    }
}
