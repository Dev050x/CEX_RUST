use crate::store::RedisManager;
use std::collections::HashMap;
use types::{
    engine::{EngineResponse, OnRampData, OnRampResponseData},
    user::{self, UserBalance},
};

const AVAILABLE_BALANCE: [&'static str; 4] = ["BTC", "SOL", "ETH", "USDT"];
const HARD_CODED_USER_1: &'static str = "582c5432-d357-469d-8a18-9081f9d0762c";
const HARD_CODED_USER_2: &'static str = "db47313c-b40f-4c68-8ccd-c50b592b1241";

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
            available_balance: 0,
            locked_balance: 0,
            reserve_balance: 0,
        });
    }

    if data.user_id == HARD_CODED_USER_1.to_string() {
        user_assets.insert(
            "USDT".to_string(),
            UserBalance {
                available_balance: 10000,
                locked_balance: 0,
                reserve_balance: 0,
            },
        );
    }

    if data.user_id == HARD_CODED_USER_2.to_string() {
        user_assets.insert(
            "ETH".to_string(),
            UserBalance {
                available_balance: 10,
                locked_balance: 0,
                reserve_balance: 0,
            },
        );
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
