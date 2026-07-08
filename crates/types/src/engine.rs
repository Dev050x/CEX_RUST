use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::user::UserBalance;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "msg")]
pub enum EngineRequest {
    CreateOrder {
        correlation_id: String,
        data: CreateOrderData,
    },
    OnRamp {
        correlation_id: String,
        data: OnRampData,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderData {
    pub market: String,
    pub qty: String,
    pub price: Option<String>,
    pub r#type: TypeOfOrder,
    pub user_id: String,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnRampData {
    pub user_id: String,
}

// Response TYPE -----------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "msg")]
pub enum EngineResponse {
    CreateOrder {
        correlation_id: String,
        data: CreateOrderResponseData,
    },
    OnRamp {
        correlation_id: String,
        data: OnRampResponseData,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderResponseData {
    pub user_id: String,
    pub filled: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnRampResponseData {
    pub user_id: String,
    pub balance: HashMap<String, UserBalance>,
}

// Extra Type-----------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TypeOfOrder {
    LIMIT,
    MARKET,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    BUY,
    SELL,
}
