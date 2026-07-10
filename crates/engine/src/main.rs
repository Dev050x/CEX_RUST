mod balance;
mod ingester;
mod market;
mod matching;
mod messages;
mod router;
mod services;
mod store;
mod utils;

use tokio::sync::mpsc;
use types::engine::{EngineRequest, Market};

use crate::{
    balance::balance,
    ingester::ingester,
    market::run_market,
    messages::{Channels, RxChannels, TxChannels, types::Order},
    router::router,
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

    //market
    tokio::spawn(async move {
        run_market(Market::BTC, rxChannels.rx_btc_channel).await;
    });
    tokio::spawn(async move {
        run_market(Market::ETH, rxChannels.rx_eth_channel).await;
    });
    tokio::spawn(async move {
        run_market(Market::SOL, rxChannels.rx_sol_channel).await;
    });

    router.await.unwrap();
}
