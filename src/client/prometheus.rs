use std::time::SystemTime;
use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct PrometheusError {
    msg: String
}

impl fmt::Display for PrometheusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Something went wrong with Prometheus {0}", self.msg)
    }
}

impl Error for PrometheusError {}

impl From<reqwest::Error> for PrometheusError {
    fn from(err: reqwest::Error) -> Self {
        PrometheusError {msg: format!("HTTP request failed {}", err) }
    }
} 

#[derive(Debug, Deserialize, Serialize)]
pub struct PrometheusResponse {
    status: String,
    data: PrometheusData
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "resultType", content = "result", rename_all = "camelCase")]
pub enum PrometheusData {
    Vector(Vec<VectorResult>),
    Matrix(Vec<MatrixResult>)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VectorResult {
    pub metric: HashMap<String, String>,
    pub value: (f64, String)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatrixResult {
    pub metric: HashMap<String, String>,
    pub values: Vec<(f64, String)>
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

/// Calls prometheus api along with client config, query and params
pub fn run_prometheus(query: String, query_params: PrometheusQueryParams, prometheus_config: PrometheusConfig) 
        -> Result<PrometheusResponse, PrometheusError> {
    // sanity check: 1. ensure query string is non empty
    if query.trim().is_empty() {
        return Err(PrometheusError{msg: "Query string cannot be empty".to_string()})
    }
    
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(prometheus_config.timeout as u64))
        .build()?;

    let url = format!("{}/api/v1/query",prometheus_config.base_url.trim_end_matches('/'));
    let mut params = vec![("query".to_string(), query.clone())];

    if let Some(limit) = query_params.limit {
        params.push(("limit".to_string(), limit.to_string()))
    }

    if let Some(start_time) = query_params.start {
        if let Ok(duration) = start_time.duration_since(std::time::UNIX_EPOCH) {
            params.push(("start".to_string(), duration.as_secs().to_string()));
        } else {
            return Err(PrometheusError{msg: format!("Error with time {:?}", start_time)})
        }
    }

    if let Some(end_time) = query_params.end {
        if let Ok(duration) = end_time.duration_since(std::time::UNIX_EPOCH) {
            params.push(("end".to_string(), duration.as_secs().to_string()));
        } else {
            return Err(PrometheusError{msg: format!("Error with time {:?}", end_time)});
        }
    }

    let response  = client.get(&url).query(&params).send()?;
    if !response.status().is_success() {
        return Err(PrometheusError{msg : format!("Prometheus return API error code: {:?}", response.text())});
    }

    let prometheus_res: PrometheusResponse = response.json()?;
    Ok(prometheus_res)
}
