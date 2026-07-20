use redis::{AsyncCommands, Client, streams::StreamReadReply};
use sqlx::{Pool, Postgres};
use sqlx_postgres::PostgresDb;
use std::time::Duration;
use types::engine::EngineResponse;

use crate::controllers::{handle_create_order, handle_delete_order};

mod controllers;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("error in loading redis url");

    let client = Client::open(redis_url).unwrap();
    println!("connected to redis");

    let config =
        redis::AsyncConnectionConfig::new().set_response_timeout(Some(Duration::from_secs(10)));
    let mut conn = client
        .get_multiplexed_async_connection_with_config(&config)
        .await
        .unwrap();
    let opts = redis::streams::StreamReadOptions::default()
        .block(5_000)
        .count(1);
    let mut last_id: String = conn
        .xinfo_stream("to-backend")
        .await
        .unwrap_or_else(|_| "$".to_string());

    let db_connection = PostgresDb::new()
        .await
        .unwrap()
        .get_pg_connection()
        .expect("Some Error in DB Connection");

    loop {
        let Ok(value) = conn
            .xread_options(&["to-backend"], &[last_id.clone()], &opts)
            .await
        else {
            println!("no data in redis stream");
            continue;
        };
        // println!("Waiting on stream: to-backend, last_id: {}", last_id);
        let reply: StreamReadReply = redis::from_redis_value(value).unwrap();
        println!("db poller got your request: {:?} \new", reply);

        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();
                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };

                    if let Ok(engine_response) = serde_json::from_str::<EngineResponse>(json_str) {
                        println!("engine_response: {:?}", engine_response);
                        handle_response(engine_response, &db_connection).await
                    }
                }
            }
        }
    }
}

async fn handle_response(data: EngineResponse, conn: &Pool<Postgres>) {
    match data {
        EngineResponse::CreateOrder {
            correlation_id: _,
            data,
        } => handle_create_order(data, conn).await.unwrap(),
        EngineResponse::OnRamp {
            correlation_id: _,
            data: _,
        } => {}
        EngineResponse::CancelOrder {
            correlation_id: _,
            data,
        } => handle_delete_order(data, conn).await.unwrap(),
        _ => {}
    }
}
