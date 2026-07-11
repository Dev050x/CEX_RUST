use tokio::sync::mpsc;

use crate::messages::types::{Order, UpdateBalance};

pub mod types;

pub struct ChannelsBalance {
    pub btc: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
    pub eth: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
    pub sol: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
}

pub struct TxChannelsBalance {
    pub btc: mpsc::Sender<Order>,
    pub sol: mpsc::Sender<Order>,
    pub eth: mpsc::Sender<Order>
}

pub struct RxChannelsBalance {
    pub btc: mpsc::Receiver<Order>,
    pub sol: mpsc::Receiver<Order>,
    pub eth: mpsc::Receiver<Order>
}


pub struct ChannelsMarket {
    pub btc: (mpsc::Sender<UpdateBalance>, mpsc::Receiver<UpdateBalance>),
    pub eth: (mpsc::Sender<UpdateBalance>, mpsc::Receiver<UpdateBalance>),
    pub sol: (mpsc::Sender<UpdateBalance>, mpsc::Receiver<UpdateBalance>),
}


pub struct TxChannelsMarket {
    pub btc: mpsc::Sender<UpdateBalance>,
    pub sol: mpsc::Sender<UpdateBalance>,
    pub eth: mpsc::Sender<UpdateBalance>
}

pub struct RxChannelsMarket {
    pub btc: mpsc::Receiver<UpdateBalance>,
    pub sol: mpsc::Receiver<UpdateBalance>,
    pub eth: mpsc::Receiver<UpdateBalance>
}

