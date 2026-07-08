use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
    pub available_balance: u64,
    pub locked_balance: u64,
    pub reserve_balance: u64,
}
