use std::collections::HashMap;
use types::{
    engine::{EngineResponse, GetBalanceData, GetBalanceResponseData},
    user::UserBalance,
};

use crate::store::RedisManager;

pub async fn handle_get_balance(
    correlation_id: String,
    data: GetBalanceData,
    balances: &HashMap<String, HashMap<String, UserBalance>>,
) {
    let user_id = data.user_id;
    let user_balance = balances.get(&user_id);
    let manager = RedisManager::get_instance().await;
    match user_balance {
        Some(user_balance) => {
            let _ = manager
                .publish_message(&EngineResponse::GetBalance {
                    correlation_id,
                    data: GetBalanceResponseData {
                        balance: Some(user_balance.clone()),
                    },
                })
                .await;
        }
        None => {
            let _ = manager
                .publish_message(&EngineResponse::GetBalance {
                    correlation_id,
                    data: GetBalanceResponseData { balance: None },
                })
                .await;
        }
    }
}
