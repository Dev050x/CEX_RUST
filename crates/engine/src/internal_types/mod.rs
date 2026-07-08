use tokio::sync::mpsc;

use types::engine::CreateOrderData;

pub struct Channels {
    pub btc_channel: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
    pub eth_channel: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
    pub sol_channel: (mpsc::Sender<Order>, mpsc::Receiver<Order>),
}

pub struct TxChannels {
    pub tx_btc_channel: mpsc::Sender<Order>,
    pub tx_sol_channel: mpsc::Sender<Order>,
    pub tx_eth_channel: mpsc::Sender<Order>
}

pub struct RxChannels {
    pub rx_btc_channel: mpsc::Receiver<Order>,
    pub rx_sol_channel: mpsc::Receiver<Order>,
    pub rx_eth_channel: mpsc::Receiver<Order>
}

pub struct Order {
    pub correlation_id: String,
    pub data: CreateOrderData,
}
