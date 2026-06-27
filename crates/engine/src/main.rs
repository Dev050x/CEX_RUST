mod router;
mod ingester;
mod balance;
mod redis_manager;

use tokio::sync::mpsc;
use types::engine::EngineRequest;

use crate::{balance::balance, ingester::ingester, router::router};

#[tokio::main]
async fn main() {
    let (tx_ingest, rx_ingest) = mpsc::channel::<EngineRequest>(1024);
    let (tx_router, rx_router) = mpsc::channel::<EngineRequest>(1024);

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
        balance(rx_router).await;
    });

    router.await.unwrap();
}
