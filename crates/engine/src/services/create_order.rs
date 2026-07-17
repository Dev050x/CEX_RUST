use std::collections::HashMap;

use types::{
    engine::{CreateOrderData, Side},
    user::UserBalance,
};

use crate::{
    messages::{TxChannelsBalance, types::Order}, utils::{convert_to_decimal, send_create_order_response},
};

pub async fn handle_create_order(
    correlation_id: String,
    data: CreateOrderData,
    balances: &mut HashMap<String, HashMap<String, UserBalance>>,
    channels_balance: &TxChannelsBalance,
) {
    let Some(user_balances) = balances.get_mut(&data.user_id) else {
        send_create_order_response(
            correlation_id,
            data.user_id.clone(),
            None,
            0.to_string(),
            String::from("Please Buy Asset First"),
            Vec::new(),
            types::engine::OrderStatus::CANCEL,
            data,
            None
        )
        .await;
        return;
    };

    let user_usdt_balance = user_balances
        .get(&String::from("USDT"))
        .unwrap()
        .available_balance;
    println!("user usdt balance {:?} ", user_usdt_balance);
    let user_asset_balance = user_balances
        .get(&String::from(&data.market))
        .unwrap()
        .available_balance;
    println!("user asset balance {:?} ", user_asset_balance);

    // check user have enough balance
    // TODO: get avg price for market price(still remain)
    match &data.side {
        Side::BUY => {
            let qty = convert_to_decimal(data.qty.clone())
                * convert_to_decimal(data.price.as_ref().unwrap().clone());
            println!("required usdt {:?} ", qty);
            if user_usdt_balance < qty {
                send_create_order_response(
                    correlation_id,
                    data.user_id.clone(),
                    None,
                    0.to_string(),
                    String::from("User Does not have enough USDT"),
                    Vec::new(),
                    types::engine::OrderStatus::CANCEL,
                    data,
                    None
                )
                .await;
                return;
            } else {
                //reserve balance
                if let Some(user_usdt_balance) = user_balances.get_mut(&String::from("USDT")) {
                    user_usdt_balance.available_balance -= qty;
                    user_usdt_balance.reserve_balance += qty;
                }
                send_to_orderbook(
                    Order {
                        correlation_id,
                        data,
                    },
                    channels_balance,
                )
                .await;
            }
        }
        Side::SELL => {
            let qty = convert_to_decimal(data.qty.clone());
            if user_asset_balance < qty {
                send_create_order_response(
                    correlation_id,
                    data.user_id.clone(),
                    None,
                    0.to_string(),
                    String::from("User Does not have enough Asset"),
                    Vec::new(),
                    types::engine::OrderStatus::CANCEL,
                    data,
                    None
                )
                .await;
                return;
            } else {
                //reserve balance
                if let Some(user_asset_balance) = user_balances.get_mut(&data.market) {
                    user_asset_balance.available_balance -= qty;
                    user_asset_balance.reserve_balance += qty;
                }
                send_to_orderbook(
                    Order {
                        correlation_id,
                        data,
                    },
                    channels_balance,
                )
                .await;
            }
        }
    }
}

async fn send_to_orderbook(order: Order, channels: &TxChannelsBalance) {
    let tx = match order.data.market.as_str() {
        "BTC" => &channels.btc,
        "SOL" => &channels.sol,
        "ETH" => &channels.eth,
        _ => {
            println!("no matching asset");
            return;
        }
    };
    if let Err(e) = tx.send(order).await {
        println!(
            "there is some error in sending to orderbook via channel {:?}",
            e
        );
    }
}
