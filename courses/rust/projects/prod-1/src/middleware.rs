use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use axum::body::Body;
use axum::http::{Request, Response};
use tower::{Layer, Service};
use tower_http::trace::TraceLayer;

// ---------------------------------------------------------------------------
// Part 2: Request Tracing
// ---------------------------------------------------------------------------

/// Return a configured [`TraceLayer`] that creates a span per request and logs
/// the status code + elapsed time on response.
pub fn trace_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    todo!("Part 2: build and return a TraceLayer configured with make_span_with and on_response")
}

// ---------------------------------------------------------------------------
// Part 5: Custom Metrics Layer
// ---------------------------------------------------------------------------

/// Tower [`Layer`] that wraps services with [`MetricsService`].
#[derive(Clone)]
pub struct MetricsLayer;

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MetricsService { inner }
    }
}

/// Tower [`Service`] that records request latency, counts, and active
/// connections via the `metrics` crate.
#[derive(Clone)]
pub struct MetricsService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MetricsService<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        // NB: swap so the *ready* clone is used for this call.
        std::mem::swap(&mut self.inner, &mut inner);

        Box::pin(async move {
            todo!(
                "Part 5: record start time, call inner, compute elapsed, \
                 record histogram/counter/gauge via metrics crate"
            )
        })
    }
}

/// Convenience function to create a [`MetricsLayer`].
pub fn metrics_layer() -> MetricsLayer {
    MetricsLayer
}
