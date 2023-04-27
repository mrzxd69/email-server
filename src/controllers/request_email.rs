use crate::structures::{request::Body, database::PoolDatabaseConnection};
use crate::database::users::register_user;

use regex::Regex;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/email")]
pub async fn request_email(req_body: web::Json<Body>, database_manager: web::Data<PoolDatabaseConnection>) -> impl Responder {
    let regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

    if req_body.email.is_empty() || !regex.is_match(&req_body.email) {
        return HttpResponse::BadRequest().body("Указан некоррентный email");
    }
    
    match register_user(
        req_body.email.to_string(), 
        database_manager.connection.clone()
    ).await {
        Ok(()) => HttpResponse::Ok().body("Поздравляем! На вашу почту поступит одним из первых письмо с ссылкой на приложение!"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}