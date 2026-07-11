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
    balance::balance, ingester::ingester, market::run_market, messages::{ChannelsBalance, ChannelsMarket, RxChannelsBalance, RxChannelsMarket, TxChannelsBalance, TxChannelsMarket, types::{Order, UpdateBalance}}, router::router,
};

#[tokio::main]
async fn main() {
    let (tx_ingest, rx_ingest) = mpsc::channel::<EngineRequest>(1024);
    let (tx_router, rx_router) = mpsc::channel::<EngineRequest>(1024);

    let channels_balance = ChannelsBalance {
        btc: mpsc::channel::<Order>(1024),
        eth: mpsc::channel::<Order>(1024),
        sol: mpsc::channel::<Order>(1024),
    };

    let channels_market = ChannelsMarket {
        btc: mpsc::channel::<UpdateBalance>(1024),
        eth: mpsc::channel::<UpdateBalance>(1024),
        sol: mpsc::channel::<UpdateBalance>(1024),
    };

    let tx_balance = TxChannelsBalance {
        btc: channels_balance.btc.0,
        sol: channels_balance.sol.0,
        eth: channels_balance.eth.0,
    };

    let rx_balance = RxChannelsBalance {
        btc: channels_balance.btc.1,
        sol: channels_balance.sol.1,
        eth: channels_balance.eth.1,
    };

    let tx_market = TxChannelsMarket {
        btc: channels_market.btc.0,
        sol: channels_market.sol.0,
        eth: channels_market.eth.0,
    };

    let rx_market = RxChannelsMarket {
        btc: channels_market.btc.1,
        sol: channels_market.sol.1,
        eth: channels_market.eth.1,
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
        balance(rx_router, tx_balance, rx_market).await;
    });

    //market
    tokio::spawn(async move {
        run_market(Market::BTC, rx_balance.btc, tx_market.btc).await;
    });
    tokio::spawn(async move {
        run_market(Market::ETH, rx_balance.eth, tx_market.eth).await;
    });
    tokio::spawn(async move {
        run_market(Market::SOL, rx_balance.sol, tx_market.sol).await;
    });

    router.await.unwrap();
}
