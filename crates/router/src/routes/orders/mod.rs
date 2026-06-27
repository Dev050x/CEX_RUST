use actix_web::{HttpResponse, post, web};
use types::engine::{CreateOrderData, EngineRequest, OnRampData};
use uuid::Uuid;

use crate::{
    error::CustomError,
    types::{app::AppState, order::CreateOrderSchema, user::Payload},
    utils::send_to_engine,
};

#[post("/order")]
pub async fn create_order(
    payload: web::ReqData<Payload>,
    data: web::Json<CreateOrderSchema>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let body: CreateOrderSchema = data.into_inner();
    let inner_payload = payload.into_inner();
    let correlation_id = Uuid::new_v4();

    let extra_payload = CreateOrderData {
        market: body.market,
        qty: body.qty,
        price: body.price,
        type_of_order: body.type_of_order,
        user_id: inner_payload.user_id,
    };

    let payload = EngineRequest::CreateOrder {
        correlation_id: correlation_id.to_string(),
        data: extra_payload,
    };

    send_to_engine(correlation_id.to_string(), payload).await
}

#[post("/onramp")]
pub async fn onramp(
    payload: web::ReqData<Payload>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let inner_payload = payload.into_inner();
    let correlation_id = Uuid::new_v4();

    let extra_payload = OnRampData {
        user_id: inner_payload.user_id,
    };

    let payload = EngineRequest::OnRamp {
        correlation_id: correlation_id.to_string(),
        data: extra_payload,
    };

    send_to_engine(correlation_id.to_string(), payload).await
}
