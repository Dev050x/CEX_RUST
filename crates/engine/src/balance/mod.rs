use std::collections::HashMap;
use tokio::sync::mpsc;
use types::{engine::EngineRequest, user::UserBalance};

use crate::{
    internal_types::{TxChannels}, ops::{create_order::handle_create_order, onramp::handle_onramp},
};

pub async fn balance(mut rx_router: mpsc::Receiver<EngineRequest>, mut channels: TxChannels) {
    let mut balances: HashMap<String, HashMap<String, UserBalance>> = HashMap::new(); // userId -> Asset -> UserBalance
    while let Some(msg) = rx_router.recv().await {
        match msg {
            EngineRequest::CreateOrder {
                correlation_id,
                data,
            } => {
                handle_create_order(correlation_id, data, &mut balances, &mut channels).await;
            }
            EngineRequest::OnRamp {
                correlation_id,
                data,
            } => {
                handle_onramp(correlation_id, data, &mut balances).await;
            }
        }
    }
}
