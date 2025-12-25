use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}