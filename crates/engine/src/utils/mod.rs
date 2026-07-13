use types::engine::{CreateOrderData, CreateOrderResponseData, EngineResponse, OrderStatus, Trade};

use crate::store::RedisManager;

pub async fn send_create_order_response(
    correlation_id: String,
    user_id: String,
    order_id: Option<String>,
    filled: String,
    msg: String,
    trades: Vec<Trade>,
    status: OrderStatus,
    order: CreateOrderData
) {
    println!("sending the response {:?} \new", msg);
    let _ = RedisManager::get_instance()
        .await
        .publish_message(&EngineResponse::CreateOrder {
            correlation_id,
            data: CreateOrderResponseData {
                user_id: user_id,
                order_id,
                trades: trades,
                filled,
                msg,
                status,
                order
            }
        })
        .await;
    return;
}
