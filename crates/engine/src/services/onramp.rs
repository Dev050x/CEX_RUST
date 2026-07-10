use crate::store::RedisManager;
use std::collections::HashMap;
use types::{
    engine::{EngineResponse, OnRampData, OnRampResponseData},
    user::UserBalance,
};

const AVAILABLE_BALANCE: [&'static str; 2] = ["SOL", "ETH"];

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
