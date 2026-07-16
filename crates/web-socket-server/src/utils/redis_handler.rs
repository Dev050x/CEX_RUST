use redis::streams::StreamReadReply;
use tokio::{
    sync::broadcast::{Sender},
};
use types::engine::EngineResponse;
use crate::{store::RedisManager};

pub async fn read_redis_stream_data(depth_tx: Sender<String>) {
    let manager = RedisManager::get_instance().await;
    let mut last_id = manager
        .get_last_stream_id("to-backend")
        .await
        .unwrap_or("0".to_string());

    loop {
        let Ok(result) = manager.read_message(&last_id).await else {
            println!("no data received in websocket server");
            continue;
        };

        let reply: StreamReadReply = redis::from_redis_value(result).unwrap();
        println!("redis server received the data: {:?}", reply);

        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();

                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };
                    let _ = depth_tx.send(json_str.to_string());
                    if let Ok(engine_response) = serde_json::from_str::<EngineResponse>(json_str) {
                        println!("received engine response: {:?}", engine_response);
                    }
                }
            }
        }
    }
}