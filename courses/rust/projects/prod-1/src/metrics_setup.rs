use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

/// Install the Prometheus metrics recorder as the global recorder and return
/// the [`PrometheusHandle`] used to render the `/metrics` endpoint.
pub fn install_prometheus_recorder() -> PrometheusHandle {
    todo!(
        "Part 3: build a PrometheusBuilder, install it, and return the handle"
    )
}
