use redis::streams::StreamReadReply;
use tokio::sync::mpsc;
use types::engine::EngineRequest;

use crate::store::RedisManager;

pub async fn ingester(tx_ingest: mpsc::Sender<EngineRequest>) {
    let manager = RedisManager::get_instance().await;
    let mut last_id = manager
        .get_last_stream_id("to-engine")
        .await
        .unwrap_or_else(|_| "0".to_string());
    loop {
        let Ok(result) = manager.read_message(&last_id).await else {
            println!("no data received in engine");
            continue;
        };

        let reply: StreamReadReply = redis::from_redis_value(result).unwrap();
        println!("ingester got your request: {:?} \new", reply);

        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();

                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };

                    if let Ok(engine_request) = serde_json::from_str::<EngineRequest>(json_str) {
                        let _ = tx_ingest.send(engine_request).await;
                    }
                }
            }
        }
    }
}
