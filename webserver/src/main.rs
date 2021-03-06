mod holdings;
mod prices;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(prices::price_service("/prices"))
            .service(user::user_service("/user"))
            .service(holdings::holdings_service("/holdings"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
