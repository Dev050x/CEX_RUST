use std::collections::VecDeque;

use rust_decimal::Decimal;
use tokio::sync::mpsc;
use types::engine::{OrderStatus, Orderbook, Orders, RestingOrder, Side, Trade, TypeOfOrder};

use crate::messages::types::{BalanceOps, OrderData, UpdateBalance};

pub async fn match_order(
    orderbook: &mut Orderbook,
    mut order: OrderData,
    tx_channel_market: &mut mpsc::Sender<UpdateBalance>,
) -> Vec<Trade> {
    let trades = match order.side {
        Side::BUY => match_against_asks(orderbook, &mut order, tx_channel_market).await,
        Side::SELL => match_against_bids(orderbook, &mut order, tx_channel_market).await,
    };

    if order.qty > Decimal::from(0) && !matches!(order.r#type, TypeOfOrder::MARKET) {
        order.status = if trades.is_empty() {
            OrderStatus::OPEN
        } else {
            OrderStatus::PartialyFilled
        };

        let orderbook_side = match order.side {
            Side::BUY => &mut orderbook.bids,
            Side::SELL => &mut orderbook.asks,
        };

        let resting_order = orderbook_side
            .entry(order.price.unwrap())
            .or_insert_with(|| RestingOrder {
                available_qty: Decimal::from(0),
                orders: VecDeque::new(),
            });

        //locking user blaance
        let _ = match &order.side {
            Side::BUY => {
                tx_channel_market
                    .send(UpdateBalance {
                        user_id: order.user_id.clone(),
                        asset: "USDT".to_string(),
                        available_balance: None,
                        locked_balance: Some(BalanceOps::Increase(
                            order.qty * order.price.unwrap(),
                        )),
                        reserved_balance: None,
                    })
                    .await
            }
            Side::SELL => {
                tx_channel_market
                    .send(UpdateBalance {
                        user_id: order.user_id.clone(),
                        asset: order.market.clone(),
                        available_balance: None,
                        locked_balance: Some(BalanceOps::Increase(order.qty)),
                        reserved_balance: None,
                    })
                    .await
            }
        };
        println!("user balance locked");

        resting_order.available_qty += order.qty;
        resting_order.orders.push_back(Orders {
            order_id: order.order_id,
            user_id: order.user_id,
            market: order.market,
            side: order.side,
            qty: order.qty,
            r#type: order.r#type,
            price: order.price.unwrap(),
            status: order.status,
        });
    }
    trades
}

async fn match_against_asks(
    orderbook: &mut Orderbook,
    order: &mut OrderData,
    tx_channel_market: &mut mpsc::Sender<UpdateBalance>,
) -> Vec<Trade> {
    let mut trades: Vec<Trade> = Vec::new();

    let asks = &mut orderbook.asks;
    while order.qty != Decimal::from(0) {
        let Some((&best_price, _)) = asks.iter().next() else {
            break;
        };

        let crosses =
            matches!(order.r#type, TypeOfOrder::MARKET) || order.price.unwrap() >= best_price;
        if !crosses {
            break;
        }

        let resting_orders = asks.get_mut(&best_price).unwrap();

        while let Some(resting_order) = resting_orders.orders.front_mut() {
            if order.qty == Decimal::from(0) {
                break;
            }
            let fill_qty = order.qty.min(resting_order.qty);

            trades.push(Trade {
                maker_order_id: resting_order.order_id.clone(),
                taker_order_id: order.order_id.clone(),
                maker_user_id: resting_order.user_id.clone(),
                taker_user_id: order.user_id.clone(),
                fill_qty: fill_qty,
                price: resting_order.price,
            });

            //updating the orders
            order.qty -= fill_qty;
            resting_order.qty -= fill_qty;
            resting_orders.available_qty -= fill_qty;

            //updating the user balance
            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: order.user_id.clone(),
                    asset: "USDT".to_string(),
                    available_balance: Some(BalanceOps::Increase(
                        (fill_qty * order.price.unwrap()) - (fill_qty * best_price),
                    )),
                    locked_balance: None,
                    reserved_balance: Some(BalanceOps::Decrease(fill_qty * best_price)),
                })
                .await;
            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: order.user_id.clone(),
                    asset: order.market.clone(),
                    available_balance: Some(BalanceOps::Increase(fill_qty)),
                    locked_balance: None,
                    reserved_balance: None,
                })
                .await;
            //updating the resting order user balance
            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: resting_order.user_id.clone(),
                    asset: "USDT".to_string(),
                    available_balance: Some(BalanceOps::Increase(fill_qty * best_price)),
                    locked_balance: None,
                    reserved_balance: None,
                })
                .await;

            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: resting_order.user_id.clone(),
                    asset: order.market.clone(),
                    available_balance: None,
                    locked_balance: Some(BalanceOps::Decrease(fill_qty)),
                    reserved_balance: None,
                })
                .await;

            if resting_order.qty == Decimal::from(0) {
                resting_orders.orders.pop_front();
            }
        }

        if resting_orders.available_qty == Decimal::from(0) {
            asks.remove(&best_price);
        }
    }

    return trades;
}

async fn match_against_bids(
    orderbook: &mut Orderbook,
    order: &mut OrderData,
    tx_channel_market: &mut mpsc::Sender<UpdateBalance>,
) -> Vec<Trade> {
    let mut trades: Vec<Trade> = Vec::new();

    let bids = &mut orderbook.bids;
    while order.qty != Decimal::from(0) {
        let Some((&best_price, _)) = bids.iter().next() else {
            break;
        };

        let crosses =
            matches!(order.r#type, TypeOfOrder::MARKET) || order.price.unwrap() <= best_price;
        if !crosses {
            break;
        }
        let resting_orders = bids.get_mut(&best_price).unwrap();

        while let Some(resting_order) = resting_orders.orders.front_mut() {
            if order.qty == Decimal::from(0) {
                break;
            }
            let fill_qty = order.qty.min(resting_order.qty);

            trades.push(Trade {
                maker_order_id: resting_order.order_id.clone(),
                taker_order_id: order.order_id.clone(),
                maker_user_id: resting_order.user_id.clone(),
                taker_user_id: order.user_id.clone(),
                fill_qty: fill_qty,
                price: resting_order.price,
            });

            //updating the orders
            order.qty -= fill_qty;
            resting_order.qty -= fill_qty;
            resting_orders.available_qty -= fill_qty;

            //updating the user balance
            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: order.user_id.clone(),
                    asset: order.market.to_string(),
                    available_balance: None,
                    locked_balance: None,
                    reserved_balance: Some(BalanceOps::Decrease(fill_qty)),
                })
                .await;
            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: order.user_id.clone(),
                    asset: "USDT".to_string(),
                    available_balance: Some(BalanceOps::Increase(fill_qty * resting_order.price)),
                    locked_balance: None,
                    reserved_balance: None,
                })
                .await;
            //updating the resting order user balance
            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: resting_order.user_id.clone(),
                    asset: order.market.clone(),
                    available_balance: Some(BalanceOps::Increase(fill_qty)),
                    locked_balance: None,
                    reserved_balance: None,
                })
                .await;

            let _ = tx_channel_market
                .send(UpdateBalance {
                    user_id: resting_order.user_id.clone(),
                    asset: "USDT".to_string(),
                    available_balance: None,
                    locked_balance: Some(BalanceOps::Decrease(fill_qty * resting_order.price)),
                    reserved_balance: None,
                })
                .await;

            if resting_order.qty == Decimal::from(0) {
                resting_orders.orders.pop_front();
            }
        }

        if resting_orders.available_qty == Decimal::from(0) {
            bids.remove(&best_price);
        }
    }

    return trades;
}
