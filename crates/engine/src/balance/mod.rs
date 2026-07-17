use std::{collections::HashMap};
use tokio::sync::mpsc;
use types::{engine::EngineRequest, user::UserBalance};

use crate::{
    messages::{RxChannelsMarket, TxChannelsBalance, types::{BalanceOps, UpdateBalance}}, services::{create_order::handle_create_order, onramp::handle_onramp},
};

pub async fn balance(
    mut rx_router: mpsc::Receiver<EngineRequest>,
    mut tx_balance: TxChannelsBalance,
    mut rx_market: RxChannelsMarket,
) {
    let mut balances: HashMap<String, HashMap<String, UserBalance>> = HashMap::new(); // userId -> Asset -> UserBalance

    loop {
        tokio::select! {
            Some(msg) = rx_router.recv() => {
                println!("balance task got your request: {:?} \new", msg);
                match msg {
                    EngineRequest::CreateOrder {
                        correlation_id,
                        data,
                    } => {
                        handle_create_order(correlation_id, data, &mut balances, &mut tx_balance).await;
                    }
                    EngineRequest::OnRamp {
                        correlation_id,
                        data,
                    } => {
                        handle_onramp(correlation_id, data, &mut balances).await;
                    }
                }
            }

            Some(update_balance_req) = rx_market.btc.recv() => {
                update_user_balance(&mut balances, update_balance_req);
            }

            Some(update_balance_req) = rx_market.eth.recv() => {
                update_user_balance(&mut balances, update_balance_req);
            }

            Some(update_balance_req) = rx_market.sol.recv() => {
                update_user_balance(&mut balances, update_balance_req);
            }

        }
    }
}

fn update_user_balance(
    balance: &mut HashMap<String, HashMap<String, UserBalance>>,
    req: UpdateBalance,
) {
    let user_asset_balance = balance.get_mut(&req.user_id).unwrap().get_mut(&req.asset).unwrap();
    
    if let Some(available_balance) = req.available_balance {
        match available_balance {
            BalanceOps::Increase(decimal) => {
                user_asset_balance.available_balance += decimal;
            },
            BalanceOps::Decrease(decimal) => {
                user_asset_balance.available_balance -= decimal;
            }
        }
    }

    if let Some(locked_balance) = req.locked_balance {
        match locked_balance {
            BalanceOps::Increase(decimal) => {
                user_asset_balance.locked_balance += decimal;
            },
            BalanceOps::Decrease(decimal) => {
                user_asset_balance.locked_balance  -= decimal;
            }
        }
    }

    if let Some(reserve_balance) = req.reserved_balance {
        match reserve_balance {
            BalanceOps::Increase(decimal) => {
                user_asset_balance.reserve_balance += decimal;
            },
            BalanceOps::Decrease(decimal) => {
                user_asset_balance.reserve_balance -= decimal;
            }
        }
    }
}
