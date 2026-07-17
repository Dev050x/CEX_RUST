use std::time::Duration;

use dotenvy;
use redis::{AsyncCommands, Client, Value};
use tokio::sync::OnceCell;

pub struct RedisManager {
    subscriber: redis::aio::MultiplexedConnection,
}

static REDIS_INSTANCE: OnceCell<RedisManager> = OnceCell::const_new();

impl RedisManager {
    async fn new() -> Self {
        dotenvy::dotenv().ok();
        let redis_url = std::env::var("REDIS_URL").expect("redis url is missing");
        let config =
            redis::AsyncConnectionConfig::new().set_response_timeout(Some(Duration::from_secs(10)));

        let subscriber = Client::open(redis_url)
            .unwrap()
            .get_multiplexed_async_connection_with_config(&config)
            .await
            .unwrap();

        let mut warmup: redis::aio::MultiplexedConnection = subscriber.clone();
        match warmup.ping::<String>().await {
            Ok(pong) => println!("redis ping ok: {pong}"),
            Err(e) => println!("redis ping FAILED: {e:?}"),
        }

        println!("connected with Redis");
        Self { subscriber }
    }

    pub async fn get_instance() -> &'static RedisManager {
        REDIS_INSTANCE
            .get_or_init(|| async { RedisManager::new().await })
            .await
    }

    pub async fn read_message(&self, last_id: &String) -> redis::RedisResult<Value> {
        let opts = redis::streams::StreamReadOptions::default()
            .block(5_000)
            .count(1);
        let mut conn = self.subscriber.clone();
        conn.xread_options(&["to-backend"], &[last_id], &opts).await
    }

    pub async fn get_last_stream_id(&self, stream: &str) -> redis::RedisResult<String> {
        let mut conn = self.subscriber.clone();
        let info: redis::streams::StreamInfoStreamReply = conn.xinfo_stream(stream).await?;
        Ok(info.last_generated_id)
    }
}
