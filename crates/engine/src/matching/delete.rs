use std::collections::BTreeMap;

use rust_decimal::Decimal;
use tokio::sync::mpsc;
use types::engine::{OrderStatus, Orderbook, Orders, RestingOrder, Side};

use crate::messages::types::{BalanceOps, DeleteOrder, UpdateBalance};
use crate::utils::{get_depth, send_cancel_order_response};

pub async fn delete_order(
    orderbook: &mut Orderbook,
    order: DeleteOrder,
    tx_channel_market: &mut mpsc::Sender<UpdateBalance>,
) {
    if let Some((removed, price)) =
        remove_from_side(&mut orderbook.bids, &order.order_id, &order.market)
    {
        unlock_balance(Side::BUY, &removed, price, &order.market, tx_channel_market).await;

        let depth = get_depth(orderbook);
        send_cancel_order_response(
            order.correlation_id,
            removed.user_id,
            removed.order_id,
            "order cancelled".to_string(),
            order.market,
            OrderStatus::CANCEL,
            Some(depth),
        )
        .await;
        return;
    }

    if let Some((removed, price)) =
        remove_from_side(&mut orderbook.asks, &order.order_id, &order.market)
    {
        unlock_balance(
            Side::SELL,
            &removed,
            price,
            &order.market,
            tx_channel_market,
        )
        .await;

        let depth = get_depth(orderbook);
        send_cancel_order_response(
            order.correlation_id,
            removed.user_id,
            removed.order_id,
            "order cancelled".to_string(),
            order.market,
            OrderStatus::CANCEL,
            Some(depth),
        )
        .await;
        return;
    }

    send_cancel_order_response(
        order.correlation_id,
        String::new(),
        order.order_id,
        "order not found".to_string(),
        order.market,
        OrderStatus::CANCEL,
        None,
    )
    .await;
}

fn remove_from_side(
    side_map: &mut BTreeMap<Decimal, RestingOrder>,
    order_id: &str,
    market: &str,
) -> Option<(Orders, Decimal)> {
    let price = side_map.iter().find_map(|(&price, resting_orders)| {
        resting_orders
            .orders
            .iter()
            .any(|o| o.order_id == order_id && o.market == market)
            .then_some(price)
    })?;

    let resting_orders = side_map.get_mut(&price)?;
    let pos = resting_orders
        .orders
        .iter()
        .position(|o| o.order_id == order_id)?;
    let removed = resting_orders.orders.remove(pos)?;
    resting_orders.available_qty -= removed.qty;

    if resting_orders.available_qty == Decimal::from(0) {
        side_map.remove(&price);
    }

    Some((removed, price))
}

async fn unlock_balance(
    side: Side,
    removed: &Orders,
    price: Decimal,
    market: &str,
    tx_channel_market: &mut mpsc::Sender<UpdateBalance>,
) {
    let _ = match side {
        Side::BUY => {
            tx_channel_market
                .send(UpdateBalance {
                    user_id: removed.user_id.clone(),
                    asset: "USDT".to_string(),
                    available_balance: Some(BalanceOps::Increase(removed.qty * price)),
                    locked_balance: Some(BalanceOps::Decrease(removed.qty * price)),
                    reserved_balance: None,
                })
                .await
        }
        Side::SELL => {
            tx_channel_market
                .send(UpdateBalance {
                    user_id: removed.user_id.clone(),
                    asset: market.to_string(),
                    available_balance: Some(BalanceOps::Increase(removed.qty)),
                    locked_balance: Some(BalanceOps::Decrease(removed.qty)),
                    reserved_balance: None,
                })
                .await
        }
    };
}
