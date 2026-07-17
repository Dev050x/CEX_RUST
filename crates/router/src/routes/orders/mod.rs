use actix_web::{HttpResponse, get, post, web};
use types::engine::{CreateOrderData, EngineRequest, GetBalanceData, GetDepthData, OnRampData};
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
        r#type: body.r#type,
        user_id: inner_payload.user_id,
        side: body.side,
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

#[get("/depth/{market}")]
pub async fn depth(
    data: web::Path<String>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let market: String = data.into_inner();
    let correlation_id = Uuid::new_v4();
    println!("received request: {:?}", market);

    let get_depth_data = GetDepthData { market };

    let payload = EngineRequest::GetDepth {
        correlation_id: correlation_id.to_string(),
        data: get_depth_data,
    };
    send_to_engine(correlation_id.to_string(), payload).await
}

#[get("/balance")]
pub async fn get_user_balance(
    payload: web::ReqData<Payload>,
) -> Result<HttpResponse, CustomError> {
    let inner_payload = payload.into_inner();
    let correlation_id = Uuid::new_v4();
    println!("received get balance request with this userId");

    let get_user_balance_data = GetBalanceData {
        user_id: inner_payload.user_id,
    };

    let payload = EngineRequest::GetBalance {
        correlation_id: correlation_id.to_string(),
        data: get_user_balance_data,
    };
    send_to_engine(correlation_id.to_string(), payload).await
}
