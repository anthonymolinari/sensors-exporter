mod data;
mod http;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add logger
    HttpServer::new(|| {
        App::new()
            .service(http::router::health)
            .service(http::router::metrics)
    })
    .bind(("0.0.0.0", 8282))?
    .run()
    .await
}
