use actix_web::{web, HttpResponse};

pub async fn api_docs() -> HttpResponse {
    HttpResponse::Ok().body(\