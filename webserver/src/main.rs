use actix_web::dev::Payload;
use actix_web::HttpRequest;
use std::future::Future;
use std::pin::Pin;
use serde::{ Serialize };
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::{ get, HttpResponse, Result };
use qstring::QString;

use db::{ establish_connection, StockPrice };

#[derive(Debug, Serialize)]
struct User {
    name: Option<String>,
}

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string();
        let parsed_query_string = QString::from(query_string);
        let user = User {
            name: parsed_query_string.get("username").map(|x| x.to_string())
        };

        Box::pin(async move {
            Ok(user)
        })
    }
}

#[get("/prices")]
async fn index() -> Result<HttpResponse> {
    let connection = establish_connection();

    let prices = StockPrice::retrieve(&connection);

    Ok(HttpResponse::Ok().json(prices))
}

#[get("/whoami")]
async fn whoami(user: User) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(whoami)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}