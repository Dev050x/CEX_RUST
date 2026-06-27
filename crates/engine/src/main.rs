mod ingester;
mod redis_manager;
mod router;

use tokio::sync::mpsc;
use types::engine::EngineRequest;

use crate::{ingester::ingester, router::router};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<EngineRequest>(1024);

    // ingester
    tokio::spawn(async move {
        ingester(tx).await;
    });

    //router
    let router = tokio::spawn(async move {
        router(rx).await;
    });

    router.await.unwrap();
}
