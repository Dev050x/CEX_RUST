use dotenvy;
use redis::{AsyncCommands, Client, Value};
use tokio::sync::OnceCell;
use types::engine::EngineResponse;

pub struct RedisManager {
    publisher: redis::aio::MultiplexedConnection,
    subscriber: redis::aio::MultiplexedConnection,
}

static REDIS_INSTANCE: OnceCell<RedisManager> = OnceCell::const_new();

impl RedisManager {
    async fn new() -> Self {
        dotenvy::dotenv().ok();
        let redis_url = std::env::var("REDIS_URL").expect("redis url is missing");
        let publisher = Client::open(redis_url.clone())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let subscriber = Client::open(redis_url)
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        println!("connected with Redis");
        Self {
            publisher,
            subscriber,
        }
    }

    pub async fn get_instance() -> &'static RedisManager {
        REDIS_INSTANCE
            .get_or_init(|| async { RedisManager::new().await })
            .await
    }

    pub async fn publish_message(&self, data: &EngineResponse) -> redis::RedisResult<()> {
        let payload = serde_json::to_string(data).unwrap();
        let mut conn = self.publisher.clone();
        conn.xadd("to-backend", "*", &[("message", payload)]).await
    }

    pub async fn read_message(&self, last_id: &String) -> redis::RedisResult<Value> {
        let opts = redis::streams::StreamReadOptions::default()
            .block(0)
            .count(1);
        let mut conn = self.subscriber.clone();
        conn.xread_options(&["to-engine"], &[last_id], &opts).await
    }
}
