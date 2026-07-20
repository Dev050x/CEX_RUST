use actix_web::{
    HttpResponse, delete, get, post,
    web::{self},
};
use types::engine::{
    CreateOrderData, DeleteOrderData, EngineRequest, GetBalanceData, GetDepthData, OnRampData,
};
use uuid::Uuid;

use crate::{
    error::CustomError,
    types::{
        app::AppState,
        order::{CreateOrderSchema, GetOrderResponse},
        user::Payload,
    },
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
pub async fn get_user_balance(payload: web::ReqData<Payload>) -> Result<HttpResponse, CustomError> {
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

#[get("/order/{order_id}")]
pub async fn get_order(
    data: web::Path<String>,
    _payload: web::ReqData<Payload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let order_id =
        Uuid::parse_str(&data.into_inner()).map_err(|_| CustomError::TypeConversionError)?;

    let order = sqlx::query!(
        "SELECT id,
        price,
        quantity,
        side::text as side,
        type::text as type,
        user_id,
        market,
        status::text as status FROM orders WHERE id = $1",
        &order_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| CustomError::DBError)?;

    let order = match order {
        Some(order) => order,
        None => return Err(CustomError::OrderNotFound),
    };

    let respone = GetOrderResponse {
        id: order.id.to_string(),
        quantity: order.quantity,
        price: order.price,
        side: order.side.unwrap(),
        r#type: order.r#type.unwrap(),
        status: order.status.unwrap(),
        user_id: order.user_id.to_string(),
        market: order.market,
    };

    Ok(HttpResponse::Ok().json(respone))
}

#[delete("/order/{order_id}")]
pub async fn delete_order(
    data: web::Path<String>,
    payload: web::ReqData<Payload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let order_id = data.into_inner();
    let inner_payload = payload.into_inner();
    let correlation_id = Uuid::new_v4();

    let order = sqlx::query!(
        "SELECT id,
        price,
        quantity,
        side::text as side,
        type::text as type,
        user_id,
        market,
        status::text as status FROM orders WHERE id = $1",
        &Uuid::parse_str(&order_id).unwrap()
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| CustomError::DBError)?;

    let order = match order {
        Some(order) => order,
        None => return Err(CustomError::OrderNotFound),
    };

    if matches!(order.status.as_deref(), Some("filled" | "cancelled")) {
        return Err(CustomError::OrderAlreadyCancelledOrFilled);
    }

    let delete_order_data = DeleteOrderData {
        user_id: inner_payload.user_id,
        order_id: order_id,
        market: order.market,
    };

    let payload = EngineRequest::DeleteOrder {
        correlation_id: correlation_id.to_string(),
        data: delete_order_data,
    };

    send_to_engine(correlation_id.to_string(), payload).await
}
