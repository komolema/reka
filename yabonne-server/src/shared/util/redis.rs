extern crate r2d2_redis;

use crate::shared::util::config::RedisConfig;
use r2d2_redis::{r2d2, RedisConnectionManager};
use crate::shared::domain::types::Email;

pub trait RedisRepository: Sync + Send{
     fn account_exists(&self, email: Email) -> bool;
}

#[derive(Clone)]
pub struct RedisRepositoryImpl {
    redis_pool: r2d2::Pool<RedisConnectionManager>,
}

//TODO: implement methods to query redis here
impl RedisRepositoryImpl {
    pub fn new(config: RedisConfig) -> RedisRepositoryImpl {
        let url = format!("redis://{}/", config.hostname);
        let manager = RedisConnectionManager::new(url).unwrap();
        let redis_pool = r2d2::Pool::builder().build(manager).unwrap();

        RedisRepositoryImpl { redis_pool }
    }
}

impl RedisRepository for RedisRepositoryImpl {
    fn account_exists(&self, email: Email) -> bool{
        false
    }
}
