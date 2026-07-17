use tokio::sync::mpsc;

use crate::messages::types::{Request, UpdateBalance};

pub mod types;

pub struct ChannelsBalance {
    pub btc: (mpsc::Sender<Request>, mpsc::Receiver<Request>),
    pub eth: (mpsc::Sender<Request>, mpsc::Receiver<Request>),
    pub sol: (mpsc::Sender<Request>, mpsc::Receiver<Request>),
}

pub struct TxChannelsBalance {
    pub btc: mpsc::Sender<Request>,
    pub sol: mpsc::Sender<Request>,
    pub eth: mpsc::Sender<Request>,
}

pub struct RxChannelsBalance {
    pub btc: mpsc::Receiver<Request>,
    pub sol: mpsc::Receiver<Request>,
    pub eth: mpsc::Receiver<Request>,
}

pub struct ChannelsMarket {
    pub btc: (mpsc::Sender<UpdateBalance>, mpsc::Receiver<UpdateBalance>),
    pub eth: (mpsc::Sender<UpdateBalance>, mpsc::Receiver<UpdateBalance>),
    pub sol: (mpsc::Sender<UpdateBalance>, mpsc::Receiver<UpdateBalance>),
}

pub struct TxChannelsMarket {
    pub btc: mpsc::Sender<UpdateBalance>,
    pub sol: mpsc::Sender<UpdateBalance>,
    pub eth: mpsc::Sender<UpdateBalance>,
}

pub struct RxChannelsMarket {
    pub btc: mpsc::Receiver<UpdateBalance>,
    pub sol: mpsc::Receiver<UpdateBalance>,
    pub eth: mpsc::Receiver<UpdateBalance>,
}
