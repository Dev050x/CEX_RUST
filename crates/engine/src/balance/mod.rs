use std::collections::HashMap;
use tokio::sync::mpsc;
use types::{
    engine::{EngineRequest, EngineResponse, OnRampData, OnRampResponseData},
    user::UserBalance,
};

use crate::redis_manager::RedisManager;

const AVAILABLE_BALANCE: [&'static str; 2] = ["SOL", "ETH"];

pub async fn balance(mut rx_router: mpsc::Receiver<EngineRequest>) {
    let mut balances: HashMap<String, HashMap<String, UserBalance>> = HashMap::new(); // userId -> Asset -> UserBalance
    while let Some(msg) = rx_router.recv().await {
        match msg {
            EngineRequest::CreateOrder {
                correlation_id,
                data,
            } => {}
            EngineRequest::OnRamp {
                correlation_id,
                data,
            } => {
                let user_assets = balances
                    .entry(data.user_id.clone())
                    .or_insert_with(HashMap::new);

                for asset in &AVAILABLE_BALANCE {
                    user_assets.entry(asset.to_string()).or_insert(UserBalance {
                        available_balance: 0,
                        locked_balance: 0,
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
        }
    }
}
