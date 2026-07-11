use std::num::ParseIntError;

use types::engine::{CreateOrderData, OrderStatus, Side, TypeOfOrder};
use uuid::Uuid;

#[derive(Debug)]
pub struct Order {
    pub correlation_id: String,
    pub data: CreateOrderData,
}

pub struct OrderData {
    pub market: String,
    pub qty: u64,
    pub price: Option<u64>,
    pub r#type: TypeOfOrder,
    pub user_id: String,
    pub side: Side,
    pub order_id: String,
    pub status: OrderStatus
}

impl TryFrom<CreateOrderData> for OrderData {
    type Error = ParseIntError;

    fn try_from(value: CreateOrderData) -> Result<Self, Self::Error> {
        Ok(OrderData {
            market: value.market,
            qty: value.qty.parse()?,
            price: value.price.map(|p| p.parse()).transpose()?,
            r#type: value.r#type,
            user_id: value.user_id,
            side: value.side,
            order_id: Uuid::new_v4().to_string(),
            status: OrderStatus::OPEN
        })
    }
}


pub struct UpdateBalance {
    pub user_id: String,
    pub asset: String,
    pub available_balance: Option<BalanceOps>,
    pub locked_balance: Option<BalanceOps>,
    pub reserved_balance: Option<BalanceOps>
}

pub enum BalanceOps {
    Increase(u64),
    Decrease(u64)
}