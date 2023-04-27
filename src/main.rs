use actix_web::{HttpServer, App, web};
use sea_orm::DatabaseConnection;
use structures::database::PoolDatabaseConnection;

mod controllers;
mod structures;
mod database;
mod models;
mod cfg;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let read_database_connection: DatabaseConnection = sea_orm::Database::connect(cfg::config::DATABASE_CONNECTION)
        .await
        .expect("Ошибка при подключении к базе данных");
   
    let database_manager: PoolDatabaseConnection = structures::database::PoolDatabaseConnection {
        connection: read_database_connection
    };


    HttpServer::new(move || {
        App::new()
            .service(controllers::request_email::request_email)
            .app_data(web::Data::new(database_manager.clone()))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
