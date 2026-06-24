use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}
