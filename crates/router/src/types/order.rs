use serde::{Deserialize, Serialize};
use types::engine::{Side, TypeOfOrder};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderSchema {
    pub market: String,
    pub qty: String,
    pub price: Option<String>,
    pub r#type: TypeOfOrder,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderResponse {
    pub id: String,
    pub quantity: String,
    pub price: String,
    pub side: String,
    pub r#type: String,
    pub status: String,
    pub user_id: String,
    pub market: String
}
