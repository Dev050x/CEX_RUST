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
