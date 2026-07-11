use std::collections::BTreeMap;
use tokio::sync::mpsc;
use types::engine::{Market, Orderbook, Trade};

use crate::{
    matching::match_order,
    messages::types::{Order, OrderData, UpdateBalance},
    utils::send_create_order_response,
};

pub async fn run_market(
    market: Market,
    mut rx_channel: mpsc::Receiver<Order>,
    mut tx_channel_market: mpsc::Sender<UpdateBalance>,
) {
    let mut orderbook = Orderbook {
        bids: BTreeMap::new(),
        asks: BTreeMap::new(),
        last_traded_price: 0,
    };

    while let Some(order) = rx_channel.recv().await {
        println!("market:{:?} got your request: {:?} \new_v4", order, market);
        let user_id = order.data.user_id.clone();
        let incoming_order: OrderData = order.data.try_into().unwrap();
        let order_id = incoming_order.order_id.clone();
        let trades = match_order(&mut orderbook, incoming_order, &mut tx_channel_market).await;
        println!("trades that happens: {:?} ", trades);
        let fill_qty = calculate_filled_qty(&trades);
        let _ = send_create_order_response(
            order.correlation_id,
            user_id,
            fill_qty.to_string(),
            "Order Succefully Placed".to_string(),
            trades,
            Some(order_id),
        ).await;
    }
}

fn calculate_filled_qty(trades: &Vec<Trade>) -> u64 {
    let mut fill_qty = 0;

    if trades.is_empty() {
        return fill_qty;
    };

    for trade in trades {
        fill_qty += trade.fill_qty;
    }

    return fill_qty;
}
