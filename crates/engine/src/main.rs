mod redis_manager;

use redis::streams::StreamReadReply;
use types::engine::{CreateOrderResponseData, EngineRequest, EngineResponse};

use crate::redis_manager::RedisManager;

#[tokio::main]
async fn main() {
    let mut last_id = "0".to_string();
    loop {
        let Ok(result) = RedisManager::get_instance()
            .await
            .read_message(&last_id)
            .await
        else {
            continue;
        };

        let reply: StreamReadReply = redis::from_redis_value(result).unwrap();
    
        println!("data from the backend: {:?} ", reply);
        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();

                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };

                    if let Ok(engine_request) = serde_json::from_str::<EngineRequest>(json_str) {
                        match engine_request {
                            EngineRequest::CreateOrder {
                                correlation_id,
                                data,
                            } => {
                                let engine_response = EngineResponse {
                                    correlation_id,
                                    data: CreateOrderResponseData {
                                        user_id: data.user_id,
                                        filled: "100".to_string(),
                                    },
                                };

                                RedisManager::get_instance()
                                    .await
                                    .publish_message(&engine_response)
                                    .await
                                    .expect("publishing error form engine");
                            }
                            EngineRequest::OnRamp {
                                correlation_id,
                                data,
                            } => {
                                let engine_response = EngineResponse {
                                    correlation_id,
                                    data: CreateOrderResponseData {
                                        user_id: data.user_id,
                                        filled: "100".to_string(),
                                    },
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}
