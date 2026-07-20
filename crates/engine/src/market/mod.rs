use rust_decimal::Decimal;
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use types::engine::{Market, OrderStatus, Orderbook, Trade};

use crate::{
    matching::{delete::delete_order, match_order},
    messages::types::{OrderData, Request, UpdateBalance},
    utils::{convert_to_decimal, get_depth, send_create_order_response, send_get_depth_response},
};

pub async fn run_market(
    market: Market,
    mut rx_channel: mpsc::Receiver<Request>,
    mut tx_channel_market: mpsc::Sender<UpdateBalance>,
) {
    let mut orderbook = Orderbook {
        bids: BTreeMap::new(),
        asks: BTreeMap::new(),
        last_traded_price: Decimal::from(0),
    };

    while let Some(order) = rx_channel.recv().await {
        match order {
            Request::OrderData(order) => {
                println!("market:{:?} got your request: {:?} \new_v4", order, market);
                let user_id = order.data.user_id.clone();
                let order_data = order.data.clone();
                let incoming_order: OrderData = order.data.try_into().unwrap();
                let order_id = incoming_order.order_id.clone();
                let trades =
                    match_order(&mut orderbook, incoming_order, &mut tx_channel_market).await;
                println!("trades that happens: {:?} ", trades);
                let fill_qty = calculate_filled_qty(&trades);
                let order_status =
                    calculate_status(fill_qty, convert_to_decimal(order_data.qty.clone()));
                let depth = get_depth(&orderbook);
                let _ = send_create_order_response(
                    order.correlation_id,
                    user_id,
                    Some(order_id),
                    fill_qty.to_string(),
                    String::from("order Placed successfully"),
                    trades,
                    order_status,
                    order_data,
                    Some(depth),
                )
                .await;
            }
            Request::DepthData(data) => {
                let depth = get_depth(&orderbook);
                send_get_depth_response(data.correlation_id, depth).await;
            }
            Request::DeleteOrderData(delete_order_data) => {
                delete_order(&mut orderbook, delete_order_data, &mut tx_channel_market).await;
            }
        }
    }
}

fn calculate_filled_qty(trades: &Vec<Trade>) -> Decimal {
    let mut fill_qty = Decimal::from(0);

    if trades.is_empty() {
        return fill_qty;
    };

    for trade in trades {
        fill_qty += trade.fill_qty;
    }

    return fill_qty;
}

fn calculate_status(fill_qty: Decimal, qty: Decimal) -> OrderStatus {
    if fill_qty == qty {
        OrderStatus::FILLED
    } else if fill_qty == Decimal::from(0) {
        OrderStatus::OPEN
    } else {
        OrderStatus::PartialyFilled
    }
}
