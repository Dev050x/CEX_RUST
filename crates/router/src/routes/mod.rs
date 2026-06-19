pub mod auth;
pub mod health;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check);

    cfg.service(
        web::scope("/api/v1")
            .service(auth::sign_up)
            .service(auth::sign_in)
    );
}