use tokio::sync::mpsc;
use types::engine::EngineRequest;

pub async fn router(
    mut rx_ingest: mpsc::Receiver<EngineRequest>,
    tx_router: mpsc::Sender<EngineRequest>,
) {
    while let Some(msg) = rx_ingest.recv().await {
        println!("router got your request: {:?} \new_v4", msg);
        let _ = tx_router.send(msg).await;
    }
}
