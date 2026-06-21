use dotenvy;
use redis::{AsyncCommands, Client, Value};
use std::sync::OnceLock;
use types::engine::EngineRequest;

pub struct RedisManager {
    publisher: redis::aio::MultiplexedConnection,
    subscriber: redis::aio::MultiplexedConnection,
}

static REDIS_INSTANCE: OnceLock<RedisManager> = OnceLock::new();

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
        Self {
            publisher,
            subscriber,
        }
    }

    pub async fn get_instance() -> &'static RedisManager {
        if let Some(redis_instance) = REDIS_INSTANCE.get() {
            return redis_instance;
        };
        let redis_manager = RedisManager::new().await;
        return REDIS_INSTANCE.get_or_init(|| redis_manager);
    }

    pub async fn publish_message(&self, data: &EngineRequest) -> redis::RedisResult<()> {
        let payload = serde_json::to_string(data).unwrap();
        let mut conn = self.publisher.clone();
        conn.xadd("backend-to-engine", "*", &[("message", payload)])
            .await
    }

    pub async fn read_message(&self) -> redis::RedisResult<Value> {
        let opts = redis::streams::StreamReadOptions::default()
            .block(5000)
            .count(1);
        let mut conn = self.subscriber.clone();
        conn.xread_options(&["engine-to-backend"], &["$"], &opts)
            .await
    }
}
