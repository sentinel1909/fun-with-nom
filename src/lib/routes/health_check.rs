// src/lib/routes/health_check.rs

// dependencies
use actix_web::{get, HttpResponse, Responder};

// health_check endpoint; responds with 200 OK and an empty body
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
