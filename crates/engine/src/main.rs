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
        btc: mpsc::channel::<Order>(1024),
        eth: mpsc::channel::<Order>(1024),
        sol: mpsc::channel::<Order>(1024),
    };

    let tx_channels = TxChannels {
        btc: channels.btc.0,
        sol: channels.sol.0,
        eth: channels.eth.0,
    };

    let rx_channels = RxChannels {
        btc: channels.btc.1,
        sol: channels.sol.1,
        eth: channels.eth.1,
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
        balance(rx_router, tx_channels).await;
    });

    //market
    tokio::spawn(async move {
        run_market(Market::BTC, rx_channels.btc).await;
    });
    tokio::spawn(async move {
        run_market(Market::ETH, rx_channels.eth).await;
    });
    tokio::spawn(async move {
        run_market(Market::SOL, rx_channels.sol).await;
    });

    router.await.unwrap();
}
