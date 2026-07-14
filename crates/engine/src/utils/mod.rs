use types::engine::{CreateOrderData, CreateOrderResponseData, Depth, EngineResponse, OrderStatus, Orderbook, Trade};

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
    depth: Option<Depth>
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
                depth
            }
        })
        .await;
    return;
}

pub fn get_depth(orderbook: &Orderbook) -> Depth {
    let mut depth = Depth {
        bids: Vec::new(),
        asks: Vec::new()
    };
    
    for (price, resting_order) in &orderbook.bids {
        depth.bids.push([price.to_string(), resting_order.available_qty.to_string()]);
    }

    for (price, resting_order) in &orderbook.asks {
        depth.asks.push([price.to_string(), resting_order.available_qty.to_string()]);
    }
    return depth;
}
