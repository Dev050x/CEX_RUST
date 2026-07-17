use crate::store::RedisManager;
use rust_decimal::Decimal;
use serde_json::to_string;
use std::collections::HashMap;
use types::{
    engine::{EngineResponse, OnRampData, OnRampResponseData},
    user::UserBalance,
};

const AVAILABLE_BALANCE: [&'static str; 4] = ["BTC", "SOL", "ETH", "USDT"];
const HARD_CODED_USER_1: &'static str = "7c33c301-ddb1-4239-8ffe-79a707110b77";
const HARD_CODED_USER_2: &'static str = "df79bdf8-bbec-4800-9e85-c7202c0016f0";

pub async fn handle_onramp(
    correlation_id: String,
    data: OnRampData,
    balances: &mut HashMap<String, HashMap<String, UserBalance>>,
) {
    let user_assets = balances
        .entry(data.user_id.clone())
        .or_insert_with(HashMap::new);

    for asset in &AVAILABLE_BALANCE {
        user_assets.entry(asset.to_string()).or_insert(UserBalance {
            available_balance: Decimal::from(0),
            locked_balance: Decimal::from(0),
            reserve_balance: Decimal::from(0),
        });
    }

    if data.user_id == HARD_CODED_USER_1.to_string() {
        for asset in &AVAILABLE_BALANCE {
            user_assets.insert(
                asset.to_string(),
                UserBalance {
                    available_balance: Decimal::from(10000),
                    locked_balance: Decimal::from(0),
                    reserve_balance: Decimal::from(0),
                },
            );
        }
    }

    if data.user_id == HARD_CODED_USER_2.to_string() {
        for asset in &AVAILABLE_BALANCE {
            user_assets.insert(
                asset.to_string(),
                UserBalance {
                    available_balance: Decimal::from(10000),
                    locked_balance: Decimal::from(0),
                    reserve_balance: Decimal::from(0),
                },
            );
        }
    }
    println!("Publishing msg to backend");
    let _ = RedisManager::get_instance()
        .await
        .publish_message(&EngineResponse::OnRamp {
            correlation_id,
            data: OnRampResponseData {
                user_id: data.user_id,
                balance: user_assets.clone(),
            },
        })
        .await;
}
