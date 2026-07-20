pub mod auth;
pub mod health;
pub mod orders;

use actix_web::{middleware::from_fn, web::{self}};

use crate::{middleware::my_custom_middleware};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check);

    cfg.service(
        web::scope("/api/v1")
            .service(auth::sign_up)
            .service(auth::sign_in)
            .service(orders::depth)
            .service(
                web::scope("")
                    .wrap(from_fn(my_custom_middleware))
                    .service(orders::onramp)
                    .service(orders::create_order)
                    .service(orders::get_user_balance)
                    .service(orders::get_order)
                    .service(orders::delete_order)
            ),
    );
}
