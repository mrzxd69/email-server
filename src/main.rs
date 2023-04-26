use actix_web::{HttpServer, App};

mod controllers;
mod structures;
mod database;
mod models;
mod cfg;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::request_email::request_email)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
