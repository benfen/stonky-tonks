use actix_web::{ get, HttpResponse, Result };
use db::{ establish_connection, StockPrice };

#[get("/prices")]
async fn index() -> Result<HttpResponse> {
    let connection = establish_connection();

    let prices = StockPrice::retrieve(&connection);

    Ok(HttpResponse::Ok().json(prices))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}