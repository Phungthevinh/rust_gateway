//thư viện
use std::sync::Arc;
use tower::{Layer, Service};

   

//sử dụng folder
use crate::domains::rate_limit;

#[derive(Clone)]
pub struct RateLimitLayer {
    service: Arc<rate_limit::service::RateLimiterService>,
    rule: rate_limit::model::RateLimitRule,
}

#[derive(Clone)]
pub struct RateLimitMiddleware<S>{
    inner: S,
    service: Arc<rate_limit::service::RateLimiterService>,
    rule: rate_limit::model::RateLimitRule,
}

impl RateLimitLayer {
    pub fn new(
        service: rate_limit::service::RateLimiterService,
        rule: rate_limit::model::RateLimitRule,
    ) -> Self {
        Self {
            service: Arc::new(service),
            rule: rule,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware{
            inner,
            service: self.service.clone(),
            rule: self.rule.clone()
        }
    }
}

