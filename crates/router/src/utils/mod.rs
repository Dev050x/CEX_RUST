use actix_web::HttpResponse;
use dashmap::DashMap;
use redis::streams::StreamReadReply;
use std::sync::OnceLock;
use tokio::sync::oneshot;
use types::engine::{EngineRequest, EngineResponse};

use crate::{error::CustomError, redis::RedisManager};

type PendingMap = DashMap<String, oneshot::Sender<EngineResponse>>;

static PENDING: OnceLock<PendingMap> = OnceLock::new();

pub fn get_pending() -> &'static PendingMap {
    PENDING.get_or_init(|| DashMap::new())
}

pub async fn send_to_engine(
    correlation_id: String,
    payload: EngineRequest,
) -> Result<HttpResponse, CustomError> {
    let (tx, rx) = oneshot::channel();
    get_pending().insert(correlation_id, tx);

    RedisManager::get_instance()
        .await
        .publish_message(&payload)
        .await
        .expect("msg");

    let response = rx.await.map_err(|_| CustomError::InternalError)?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn listening_for_engine_response() {
    let mut last_id = "0".to_string();

    loop {
        let Ok(reply) = RedisManager::get_instance()
            .await
            .read_message(&last_id)
            .await
        else {
            continue;
        };

        let reply: StreamReadReply = match redis::from_redis_value(reply) {
            Ok(r) => r,
            Err(e) => {
                println!("parse reply error: {:?}", e);
                continue;
            }
        };

        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();

                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };

                    match serde_json::from_str::<EngineResponse>(json_str) {
                        Ok(response) => {
                            println!("parsed response: {:?}", response);
                            if let Some(tx) = get_pending().remove(&response.correlation_id) {
                                tx.1.send(response).ok();
                            }
                        }
                        Err(e) => println!("json parse error: {:?}", e), 
                    }
                }
            }
        }
    }
}
