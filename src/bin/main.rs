use actix_web::{get, HttpResponse, web::ServiceConfig, Responder};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(health_check);
    };

    Ok(config.into())
}
