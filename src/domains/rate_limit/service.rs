//các file model
use crate::domains::model::RateLimitRule;

//các thư viện
use deadpool_redis::Pool;

pub struct RateLimiterService {
    redis: deadpool_redis::Pool,
}

// định nghĩa 1 trait cho enum
pub trait RateLimiter {
    async fn allow(&self, key: &str, rule: &RateLimitRule) -> anyhow::Result<bool>;
}

//xử lý logic ở đây chúng tôi định cho
//FixedWindow sẽ là 5 request cho 60s
//SlidingWindow sẽ là 200 request cho 60 giây


#[async_trait::async_trait]
impl RateLimiter for RateLimiterService{
    //nhiệm vụ của hàm này là cho phép gửi otp hoặc không
    //trả lỗi gì, nếu bị block
    //gọi redis
    async fn allow(&self, key: &str, rule: &RateLimitRule) -> anyhow::Result<bool>{
        match self{
            RateLimitRule::Unlimited => ok(true),
            RateLimitRule::FixedWindow{}=>{

            }
            RateLimitRule::SlidingWindow{}=>{
                
            }
        }
    }
}