use crate::{store::RedisManager, types::DepthStore};
use redis::streams::StreamReadReply;
use tokio::sync::broadcast::Sender;
use types::engine::{Depth, EngineResponse};

pub async fn read_redis_stream_data(_depth_tx: Sender<String>, depth_store: DepthStore) {
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
                    if let Ok(engine_response) = serde_json::from_str::<EngineResponse>(json_str) {
                        println!("received engine response: {:?}", engine_response);
                        process_engine_response(engine_response, &depth_store).await;
                    }
                }
            }
        }
    }
}

async fn process_engine_response(engine_response: EngineResponse, depth_store: &DepthStore) {
    match engine_response {
        EngineResponse::CreateOrder {
            correlation_id: _,
            data,
        } => {
            let mut store = depth_store.write().await;
            if let Some(depth) = data.depth {
                store.insert(
                    data.order.market,
                    Depth {
                        bids: depth.bids,
                        asks: depth.asks,
                    },
                );
            }
        }
        EngineResponse::CancelOrder {
            correlation_id: _,
            data,
        } => {
            let mut store = depth_store.write().await;
            if let Some(depth) = data.depth {
                store.insert(
                    data.market,
                    Depth {
                        bids: depth.bids,
                        asks: depth.asks,
                    },
                );
            }
        }
        _ => {}
    }
}
