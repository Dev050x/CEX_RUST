use std::{collections::VecDeque};

use types::engine::{OrderStatus, Orderbook, Orders, RestingOrder, Side, Trade, TypeOfOrder};

use crate::messages::types::OrderData;

pub fn match_order(orderbook: &mut Orderbook, mut order: OrderData) -> Vec<Trade> {
    let trades = match order.side {
        Side::BUY => match_against_asks(orderbook, &mut order),
        Side::SELL => match_against_bids(orderbook, &mut order),
    };

    if order.qty > 0 && !matches!(order.r#type, TypeOfOrder::MARKET) {
        order.status = if trades.is_empty() {
            OrderStatus::OPEN
        } else {
            OrderStatus::PartialyFilled
        };

        let orderbook_side = match order.side {
            Side::BUY => &mut orderbook.bids,
            Side::SELL => &mut orderbook.asks
        };

        let resting_order = orderbook_side.entry(order.price.unwrap()).or_insert_with(|| RestingOrder {
            available_qty: 0,
            orders: VecDeque::new()
        });

        resting_order.available_qty += order.qty;
        resting_order.orders.push_back(Orders{
            order_id: order.order_id,
            user_id: order.user_id,
            market: order.market,
            side: order.side,
            qty: order.qty,
            r#type: order.r#type,
            price: order.price.unwrap(),
            status: order.status
        });

    }
    trades
}

fn match_against_asks(orderbook: &mut Orderbook, order: &mut OrderData) -> Vec<Trade> {
    let mut trades: Vec<Trade> = Vec::new();

    let asks = &mut orderbook.asks;
    while order.qty != 0 {
        let Some((&best_price, _)) = asks.iter().next() else { break; };

        let crosses = matches!(order.r#type, TypeOfOrder::MARKET)
            || order.price.unwrap() >= best_price;
        if !crosses { break; }

        let resting_orders = asks.get_mut(&best_price).unwrap();

        while let Some(resting_order) = resting_orders.orders.front_mut() {

            if order.qty == 0 { break; }
            let fill_qty = order.qty.min(resting_order.qty);

            trades.push(Trade {
                maker_order_id: resting_order.order_id.clone(),
                taker_order_id: order.order_id.clone(),
                maker_user_id: resting_order.user_id.clone(),
                taker_user_id: order.user_id.clone(),
                fill_qty: fill_qty,
                price: resting_order.price,
            });

            order.qty -= fill_qty;
            resting_order.qty -= fill_qty;
            resting_orders.available_qty -= fill_qty;

            if resting_order.qty == 0 {
                resting_orders.orders.pop_front();
            }
        }

        if resting_orders.available_qty == 0 {
            asks.remove(&best_price);
        }
    }

    return trades;
}

fn match_against_bids(orderbook: &mut Orderbook, order: &mut OrderData) -> Vec<Trade> {
    let mut trades: Vec<Trade> = Vec::new();

    let bids = &mut orderbook.bids;
    while order.qty != 0 {
        let Some((&best_price, _)) = bids.iter().next() else { break; };

        let crosses = matches!(order.r#type, TypeOfOrder::MARKET)
            || order.price.unwrap() <= best_price;
        if !crosses { break; }
        let resting_orders = bids.get_mut(&best_price).unwrap();

        while let Some(resting_order) = resting_orders.orders.front_mut() {

            if order.qty == 0 { break; }
            let fill_qty = order.qty.min(resting_order.qty);

            trades.push(Trade {
                maker_order_id: resting_order.order_id.clone(),
                taker_order_id: order.order_id.clone(),
                maker_user_id: resting_order.user_id.clone(),
                taker_user_id: order.user_id.clone(),
                fill_qty: fill_qty,
                price: resting_order.price,
            });

            order.qty -= fill_qty;
            resting_order.qty -= fill_qty;
            resting_orders.available_qty -= fill_qty;

            if resting_order.qty == 0 {
                resting_orders.orders.pop_front();
            }
        }

        if resting_orders.available_qty == 0 {
            bids.remove(&best_price);
        }
    }

    return trades;

}
