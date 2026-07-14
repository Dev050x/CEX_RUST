use std::collections::{BTreeMap, HashMap, VecDeque};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub order_id: Option<String>,
    pub filled: String,
    pub status: OrderStatus,
    pub msg: String,
    pub order: CreateOrderData,
    pub trades: Vec<Trade>,
    pub depth: Option<Depth>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnRampResponseData {
    pub user_id: String,
    pub balance: HashMap<String, UserBalance>,
}

// Extra Type-----------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TypeOfOrder {
    LIMIT,
    MARKET,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Side {
    BUY,
    SELL,
}

#[derive(Debug)]
pub enum Market {
    BTC,
    ETH,
    SOL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Depth {
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}

// Orderbook -----------------------------------------------------------------------------------
pub struct Orderbook {
    pub bids: BTreeMap<u64, RestingOrder>,
    pub asks: BTreeMap<u64, RestingOrder>,
    pub last_traded_price: u64,
}

pub struct RestingOrder {
    pub available_qty: u64,
    pub orders: VecDeque<Orders>,
}

pub struct Orders {
    pub order_id: String,
    pub user_id: String,
    pub market: String,
    pub side: Side,
    pub qty: u64,
    pub r#type: TypeOfOrder,
    pub price: u64,
    pub status: OrderStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderStatus {
    OPEN,
    PartialyFilled,
    FILLED,
    CANCEL,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub maker_user_id: String,
    pub taker_user_id: String,
    pub fill_qty: u64,
    pub price: u64,
}
