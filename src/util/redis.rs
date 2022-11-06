use std::sync::Arc;

use redis::{self, AsyncCommands, RedisResult};
use rocket::tokio::sync::RwLock;

pub struct Redis {
    client: Arc<RwLock<redis::aio::Connection>>,
}

impl Redis {
    pub async fn connect(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        let con = client
            .get_async_connection()
            .await
            .unwrap_or_else(|_| panic!("Error connecting to redis!!"));
        Self {
            client: Arc::new(RwLock::new(con)),
        }
    }
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u16) -> RedisResult<bool> {
        let client_clone = Arc::clone(&self.client);
        let mut client = client_clone.write().await;
        client
            .set_ex::<&str, &str, bool>(key, value, seconds.into())
            .await
    }
    pub async fn get(&self, key: &str) -> RedisResult<String> {
        let client_clone = Arc::clone(&self.client);
        let mut client = client_clone.write().await;
        client.get::<&str, String>(key).await
    }
    pub async fn sadd(&self, set: &str, member: &str) -> RedisResult<bool> {
        let client_clone = Arc::clone(&self.client);
        let mut client = client_clone.write().await;
        let result = client.sadd::<&str, &str, bool>(set, member).await;
        let _ = client.expire::<&str, bool>(set, 43200).await;
        result
    }
    pub async fn sismember(&self, set: &str, member: &str) -> RedisResult<bool> {
        let client_clone = Arc::clone(&self.client);
        let mut client = client_clone.write().await;
        client.sismember::<&str, &str, bool>(set, member).await
    }
}
