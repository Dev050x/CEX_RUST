mod balance;
mod ingester;
mod internal_types;
mod ops;
mod redis_manager;
mod router;

use tokio::sync::mpsc;
use types::engine::EngineRequest;

use crate::{
    balance::balance, ingester::ingester, internal_types::{Channels, Order, RxChannels, TxChannels}, router::router,
};

#[tokio::main]
async fn main() {
    let (tx_ingest, rx_ingest) = mpsc::channel::<EngineRequest>(1024);
    let (tx_router, rx_router) = mpsc::channel::<EngineRequest>(1024);

    let channels = Channels {
        eth_channel: mpsc::channel::<Order>(1024),
        btc_channel: mpsc::channel::<Order>(1024),
        sol_channel: mpsc::channel::<Order>(1024),
    };

    let txChannels = TxChannels {
        tx_btc_channel: channels.btc_channel.0,
        tx_sol_channel: channels.sol_channel.0,
        tx_eth_channel: channels.eth_channel.0,
    };

    let rxChannels = RxChannels {
        rx_btc_channel: channels.btc_channel.1,
        rx_sol_channel: channels.sol_channel.1,
        rx_eth_channel: channels.eth_channel.1,
    };

    // ingester
    tokio::spawn(async move {
        ingester(tx_ingest).await;
    });

    //router
    let router = tokio::spawn(async move {
        router(rx_ingest, tx_router).await;
    });

    //balance
    tokio::spawn(async move {
        balance(rx_router, txChannels).await;
    });

    router.await.unwrap();
}
