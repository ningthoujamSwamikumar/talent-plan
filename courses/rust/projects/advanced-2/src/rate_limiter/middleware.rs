use std::sync::Arc;
use std::task::{Context, Poll};

use tower::{Layer, Service};

use crate::rate_limiter::token_bucket::TokenBucket;

/// A Tower `Layer` that wraps services with rate limiting.
///
/// Each request must acquire a token from the shared `TokenBucket`
/// before being forwarded to the inner service.
#[derive(Clone)]
pub struct RateLimitLayer {
    _bucket: Arc<TokenBucket>,
}

impl RateLimitLayer {
    /// Create a new rate-limiting layer.
    ///
    /// # Arguments
    /// - `capacity` — maximum burst size.
    /// - `refill_rate` — tokens per second.
    pub fn new(_capacity: u32, _refill_rate: f64) -> Self {
        // TODO: Build a TokenBucket and wrap in Arc.
        todo!()
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, _inner: S) -> Self::Service {
        // TODO: Wrap the inner service with the shared bucket.
        todo!()
    }
}

/// A Tower `Service` that enforces rate limits before delegating to an
/// inner service.
#[derive(Clone)]
pub struct RateLimitService<S> {
    _inner: S,
    _bucket: Arc<TokenBucket>,
}

impl<S, Req> Service<Req> for RateLimitService<S>
where
    S: Service<Req> + Clone + Send + 'static,
    S::Future: Send,
    S::Response: Send,
    S::Error: Send,
    Req: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // TODO: Check if the inner service is ready.
        todo!()
    }

    fn call(&mut self, _req: Req) -> Self::Future {
        // TODO: Try to acquire a token. If successful, forward the request.
        // TODO: If not, return an appropriate error or rejection.
        todo!()
    }
}
