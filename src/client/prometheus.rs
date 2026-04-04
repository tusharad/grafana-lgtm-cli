use std::time::SystemTime;

/// Calls prometheus api along with client config, query and params
pub fn run_prometheus(query: String, query_params: PrometheusQueryParams, prometheus_config: PrometheusConfig) {
    println!("Executing {} with {:?} and config {:?}", query, query_params, prometheus_config);
}

#[derive(Debug)]
pub struct PrometheusConfig {
    base_url: String,
    timeout: u32   
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self { base_url: "http://localhost:9090".to_string(), timeout: 300 }
    }
}

#[derive(Debug)]
pub struct PrometheusQueryParams {
    limit: Option<u32>,
    start: Option<SystemTime>,
    end: Option<SystemTime>,
}

impl Default for PrometheusQueryParams {
    fn default() -> Self {
        Self { limit: None, start: None, end: None }
    }
}
