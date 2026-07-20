use std::num::ParseIntError;

use rust_decimal::Decimal;
use types::engine::{CreateOrderData, OrderStatus, Side, TypeOfOrder};
use uuid::Uuid;

use crate::utils::convert_to_decimal;

#[derive(Debug)]
pub struct Order {
    pub correlation_id: String,
    pub data: CreateOrderData,
}

#[derive(Clone)]
pub struct OrderData {
    pub market: String,
    pub qty: Decimal,
    pub price: Option<Decimal>,
    pub r#type: TypeOfOrder,
    pub user_id: String,
    pub side: Side,
    pub order_id: String,
    pub status: OrderStatus,
}

impl TryFrom<CreateOrderData> for OrderData {
    type Error = ParseIntError;

    fn try_from(value: CreateOrderData) -> Result<Self, Self::Error> {
        Ok(OrderData {
            market: value.market,
            qty: convert_to_decimal(value.qty),
            price: value.price.map(|p| convert_to_decimal(p)),
            r#type: value.r#type,
            user_id: value.user_id,
            side: value.side,
            order_id: Uuid::new_v4().to_string(),
            status: OrderStatus::OPEN,
        })
    }
}

pub struct UpdateBalance {
    pub user_id: String,
    pub asset: String,
    pub available_balance: Option<BalanceOps>,
    pub locked_balance: Option<BalanceOps>,
    pub reserved_balance: Option<BalanceOps>,
}

pub enum BalanceOps {
    Increase(Decimal),
    Decrease(Decimal),
}

#[derive(Debug)]
pub struct Depth {
    pub correlation_id: String,
}

#[derive(Debug)]
pub struct DeleteOrder {
    pub correlation_id: String,
    pub order_id: String,
    pub market: String
}

pub enum Request {
    OrderData(Order),
    DepthData(Depth),
    DeleteOrderData(DeleteOrder)
}
