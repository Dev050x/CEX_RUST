use std::collections::HashMap;

use types::{
    engine::{CreateOrderData, Side, Trade}, user::UserBalance,
};

use crate::{
    messages::{TxChannels, types::Order}, utils::send_create_order_response,
};

pub async fn handle_create_order(
    correlation_id: String,
    data: CreateOrderData,
    balances: &mut HashMap<String, HashMap<String, UserBalance>>,
    channels: &TxChannels,
) {
    let Some(user_balances) = balances.get_mut(&data.user_id) else {
        send_create_order_response(
            correlation_id,
            data.user_id,
            0.to_string(),
            "Please buy asset first".to_string(),
            Vec::new(),
            None,
        )
        .await;
        return;
    };

    let user_usdt_balance = user_balances
        .get(&String::from("USDT"))
        .unwrap()
        .available_balance;
    let user_asset_balance = user_balances
        .get(&String::from(&data.market))
        .unwrap()
        .available_balance;

    // check user have enough balance
    // TODO: get avg price for market price(still remain)
    match &data.side {
        Side::BUY => {
            let qty = data.qty.parse::<u64>().unwrap()
                * data.price.as_ref().unwrap().parse::<u64>().unwrap();
            if qty < user_usdt_balance {
                send_create_order_response(
                    correlation_id,
                    data.user_id,
                    0.to_string(),
                    "user does not have enough usdt".to_string(),
                    Vec::new(),
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
                    channels,
                )
                .await;
            }
        }
        Side::SELL => {
            let qty = data.qty.parse::<u64>().unwrap();
            if qty < user_asset_balance {
                send_create_order_response(
                    correlation_id,
                    data.user_id,
                    0.to_string(),
                    "user does not have enough asset".to_string(),
                    Vec::new(),
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
                    channels,
                )
                .await;
            }
        }
    }
}


async fn send_to_orderbook(order: Order, channels: &TxChannels) {
    let tx = match order.data.market.as_str() {
        "BTC" => &channels.tx_btc_channel,
        "SOL" => &channels.tx_sol_channel,
        "ETH" => &channels.tx_eth_channel,
        _ => {
            println!("no matching asset");
            return;
        }
    };
    if let Err(e) = tx.send(order).await {
        println!("there is some error in sending to orderbook via channel {:?}", e);
    }
}
