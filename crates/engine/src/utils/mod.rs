use rust_decimal::Decimal;
use types::engine::{
    CancelOrderResponseData, CreateOrderData, CreateOrderResponseData, Depth, EngineResponse, GetDepthResponseData, OrderStatus, Orderbook, Trade,
};

use crate::store::RedisManager;

pub async fn send_create_order_response(
    correlation_id: String,
    user_id: String,
    order_id: Option<String>,
    filled: String,
    msg: String,
    trades: Vec<Trade>,
    status: OrderStatus,
    order: CreateOrderData,
    depth: Option<Depth>,
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
                order,
                depth,
            },
        })
        .await;
    return;
}

pub async fn send_get_depth_response(correlation_id: String, depth: Depth) {
    println!(
        "sending the depth to this correlation_id {:?} \new",
        correlation_id
    );
    let _ = RedisManager::get_instance()
        .await
        .publish_message(&EngineResponse::GetDepth {
            correlation_id,
            data: GetDepthResponseData { depth },
        })
        .await;
    return;
}

pub fn get_depth(orderbook: &Orderbook) -> Depth {
    let mut depth = Depth {
        bids: Vec::new(),
        asks: Vec::new(),
    };

    for (price, resting_order) in &orderbook.bids {
        depth
            .bids
            .push([price.to_string(), resting_order.available_qty.to_string()]);
    }

    for (price, resting_order) in &orderbook.asks {
        depth
            .asks
            .push([price.to_string(), resting_order.available_qty.to_string()]);
    }
    return depth;
}

pub async fn send_cancel_order_response(
    correlation_id: String,
    user_id: String,
    order_id: String,
    msg: String,
    market: String,
    status: OrderStatus,
    depth: Option<Depth>,
) {
    println!("sending the cancel order response {:?} \n", msg);
    let _ = RedisManager::get_instance()
        .await
        .publish_message(&EngineResponse::CancelOrder {
            correlation_id,
            data: CancelOrderResponseData {
                user_id,
                order_id,
                msg,
                status,
                depth,
                market
            },
        })
        .await;
    return;
}

pub fn convert_to_decimal(data: String) -> Decimal {
    data.parse().unwrap()
}
