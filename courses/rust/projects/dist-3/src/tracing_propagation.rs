use axum::{
    extract::Request,
    http::HeaderName,
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Header name used to propagate trace IDs across services.
pub static TRACE_ID_HEADER: HeaderName = HeaderName::from_static("x-trace-id");

/// A single span in a distributed trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub trace_id: String,
    pub service_name: String,
    pub operation: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Shared collector for spans produced during request processing.
#[derive(Debug, Clone, Default)]
pub struct SpanCollector {
    pub spans: Arc<Mutex<Vec<Span>>>,
}

impl SpanCollector {
    pub fn new() -> Self {
        Self {
            spans: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn record(&self, span: Span) {
        self.spans.lock().unwrap().push(span);
    }
}

/// Axum middleware that reads or generates an `x-trace-id` header.
///
/// - If the incoming request contains `x-trace-id`, it is preserved.
/// - Otherwise a new UUID v4 is generated.
/// - The trace ID is stored in request extensions and echoed on the response.
pub async fn trace_id_middleware(req: Request, next: Next) -> Response {
    // TODO:
    // 1. Extract or generate the trace ID.
    // 2. Insert it into request extensions.
    // 3. Call next.run(req).
    // 4. Insert the trace ID into the response headers.
    let _trace_id = req
        .headers()
        .get(&TRACE_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let _ = &next;
    todo!("trace_id_middleware")
}

/// A wrapper around `reqwest::Client` that injects the current trace ID into
/// every outgoing request.
#[derive(Debug, Clone)]
pub struct TracedClient {
    inner: reqwest::Client,
}

impl TracedClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    /// Send a GET request with the given trace ID attached.
    pub async fn get_with_trace(
        &self,
        url: &str,
        trace_id: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // TODO: send a GET request, injecting x-trace-id header
        let _ = (url, trace_id);
        let _ = &self.inner;
        todo!("TracedClient::get_with_trace")
    }

    /// Send a POST request with the given trace ID and JSON body attached.
    pub async fn post_with_trace<T: serde::Serialize>(
        &self,
        url: &str,
        trace_id: &str,
        body: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // TODO: send a POST request, injecting x-trace-id header
        let _ = (url, trace_id, body);
        let _ = &self.inner;
        todo!("TracedClient::post_with_trace")
    }
}

impl Default for TracedClient {
    fn default() -> Self {
        Self::new()
    }
}
