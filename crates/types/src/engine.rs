use serde::{Deserialize, Serialize};

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
    pub type_of_order: TypeOfOrder,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TypeOfOrder {
    LIMIT,
    MARKET,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnRampData {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineResponse {
    pub correlation_id: String,
    pub data: CreateOrderResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderResponseData {
    pub user_id: String,
    pub filled: String,
}
