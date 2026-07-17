use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
    pub available_balance: Decimal,
    pub locked_balance: Decimal,
    pub reserve_balance: Decimal,
}
