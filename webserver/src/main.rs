mod user;

use actix_web::{ get, HttpResponse, Result };

use db::{ establish_connection, StockPrice };

#[get("/prices")]
async fn get_prices() -> Result<HttpResponse> {
    let connection = establish_connection();

    let prices = StockPrice::retrieve(&connection);

    Ok(HttpResponse::Ok().json(prices))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(get_prices)
            .service(user::user_service("/user"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}