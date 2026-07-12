use redis::{AsyncCommands, Client, streams::StreamReadReply};
use std::time::Duration;
use types::engine::EngineResponse;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("error in loading redis url");
    
    let client = Client::open(redis_url).unwrap();
    let config =
        redis::AsyncConnectionConfig::new().set_response_timeout(Some(Duration::from_secs(10)));
    let mut conn = client
        .get_multiplexed_async_connection_with_config(&config)
        .await
        .unwrap();
    let opts = redis::streams::StreamReadOptions::default()
        .block(0)
        .count(1);
    let mut last_id = String::from("$");

    loop {
        let Ok(value) = conn
            .xread_options(&["to-engine"], &[last_id.clone()], &opts)
            .await
        else {
            println!("no data in redis stream");
            continue;
        };
        // println!("Waiting on stream: to-backend, last_id: {}", last_id);
        let reply: StreamReadReply = redis::from_redis_value(value).unwrap();
        // println!("db poller got your request: {:?} \new", reply);

        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();
                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };

                    if let Ok(engine_response) = serde_json::from_str::<EngineResponse>(json_str) {
                        println!(
                            "engine response that we got from the db poller {:?}",
                            engine_response
                        );
                    }
                }
            }
        }
    }
}
