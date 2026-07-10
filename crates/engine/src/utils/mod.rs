use types::engine::{CreateOrderResponseData, EngineResponse, Trade};

use crate::store::RedisManager;

pub async fn send_create_order_response(
    correlation_id: String,
    user_id: String,
    filled: String,
    msg: String,
    trades: Vec<Trade>,
    order_id: Option<String>
) {
    let _ = RedisManager::get_instance()
        .await
        .publish_message(&EngineResponse::CreateOrder {
            correlation_id,
            data: CreateOrderResponseData {
                user_id: user_id,
                filled,
                msg,
                trades: trades,
                order_id
            },
        })
        .await;
    return;
}
