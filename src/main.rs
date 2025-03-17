use actix_web::{App, HttpServer};
use std::io::Error;

mod data;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new()
            .service(routes::get_all_translations)
            .service(routes::get_translation)
            .service(routes::get_random_translation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
