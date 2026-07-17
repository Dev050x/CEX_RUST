use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

use crate::types::{ClientRegistry, DepthStore};

pub async fn depth_push_interval(depth_store: DepthStore, client_registery: ClientRegistry) {
    let mut ticker = tokio::time::interval(std::time::Duration::from_millis(300));
    loop {
        ticker.tick().await;
        let store = depth_store.read().await;
        let clients = client_registery.lock().await;

        for client in clients.iter() {
            if let Some(market_depth) = store.get(&client.market) {
                let payload = serde_json::json!({
                    "depth": market_depth
                });
                let text = payload.to_string();
                let mut w = client.write.lock().await;
                let _ = w.send(Message::Text(text.into())).await;
            }
        }
    }
}
