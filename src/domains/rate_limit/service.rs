//thư viện
use redis::{Client, TypedCommands};
use anyhow::{Ok, Result};
use chrono::{DateTime, TimeZone, Utc};

//các file model
use crate::domains::rate_limit::model::RateLimitRule;

pub trait RateLimiter  {
    async fn allow(&self, key: &str, rule: &RateLimitRule) -> Result<bool>;
}

pub struct RateLimiterService;

impl RateLimiterService {
    async fn allow_fixed_window(&self, key: &str, max: &u32, window: &u64) -> Result<bool> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        let now = Utc::now().timestamp() as u64;
        let window_start = now / window;
        let redis_key = format!("ratelimit{}:{}", key, window_start);

        let count:u32 = redis::cmd("INCR").arg(&redis_key).query(&mut con)?;
        
        if count == 1{
            redis::cmd("EXPIRE").arg(&redis_key).query::<()>(&mut con)?;
        }
        Ok(count <= *max)
    }
}

impl RateLimiter for RateLimiterService {
    async fn allow(&self, key: &str, rule: &RateLimitRule) -> Result<bool> {
        match rule {
            RateLimitRule::Unlimited => Ok(true),
            RateLimitRule::FixedWindow { max_requests, window_seconds } => {
                self.allow_fixed_window(key, &max_requests, &window_seconds).await
            },
            RateLimitRule::SlidingWindow { max_requests, window_seconds } => {
                Ok(true)
            },
        };
        Ok(true)
    }
}





