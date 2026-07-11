use tokio::sync::mpsc;

use crate::messages::types::Order;

pub mod types;

pub struct Channels {
    pub btc: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
    pub eth: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
    pub sol: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
}

pub struct TxChannels {
    pub btc: mpsc::Sender<Order>,
    pub sol: mpsc::Sender<Order>,
    pub eth: mpsc::Sender<Order>
}

pub struct RxChannels {
    pub btc: mpsc::Receiver<Order>,
    pub sol: mpsc::Receiver<Order>,
    pub eth: mpsc::Receiver<Order>
}

