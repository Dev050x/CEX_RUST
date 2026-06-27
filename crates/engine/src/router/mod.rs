use tokio::sync::mpsc::Receiver;
use types::engine::EngineRequest;

pub async fn router(mut rx: Receiver<EngineRequest>) {
    while let Some(msg) = rx.recv().await {
        // we need to route msg
        println!("msg: {:?} ", msg);
    }
}
