use serde::{Deserialize, Serialize};
use types::engine::TypeOfOrder;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderSchema {
    pub market: String,
    pub qty: String,
    pub price: Option<String>,
    pub type_of_order: TypeOfOrder,
}
