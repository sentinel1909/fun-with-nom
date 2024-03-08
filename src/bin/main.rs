// src/bin/main.rs
// binary and entry point for the project

// dependencies
use actix_web::web::ServiceConfig;
use fun_with_nom_lib::routes::{count, health_check};
use shuttle_actix_web::ShuttleActixWeb;

// main function, annotated with the shuttle_runtime macro
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(health_check).service(count);
    };

    Ok(config.into())
}
