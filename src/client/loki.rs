use std::fmt;
use std::error::Error;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LokiError {
    msg: String
}

impl Error for LokiError {}

impl Display for LokiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loki error: {0}", self.msg)
    }
}

impl From<reqwest::Error> for LokiError {
    fn from(err: reqwest::Error) -> LokiError {
        LokiError {
            msg : format!("HTTP request failed: {}", err)
        }
    }
}

#[derive(Debug)]
pub enum LokiDirection {
    Forward, Backward
}

#[derive(Debug)]
pub struct LokiQueryParams {
    limit: Option<u32>,
    direction: Option<LokiDirection>,
    time: Option<u64>
}

impl Default for LokiQueryParams {
    fn default() -> LokiQueryParams {
        LokiQueryParams {
            limit: None,
            direction: None,
            time: None
        }
    }
}

#[derive(Debug)]
pub struct LokiConfig {
    base_url: String,
    timeout: u16
}

impl Default for LokiConfig {
    fn default() -> LokiConfig {
        LokiConfig {
            base_url: "http://localhost:3100".to_string(),
            timeout: 300
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorResult {
    pub metric: HashMap<String, String>,
    pub value: (f64, String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixResult {
    pub metric: HashMap<String, String>,
    pub value: (f64, String)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="resultType", content="result", rename_all="camelCase")]
pub enum LokiData {
    Vector(Vec<VectorResult>),
    Matrix(Vec<MatrixResult>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LokiResponse {
    status: String,
    data: LokiData
}

/// Entry point for talking with Loki, this function takes
/// logql query and other parameters and returns Loki Response
pub async fn run_loki(
    loki_query: String,
    loki_query_params: LokiQueryParams,
    loki_config: LokiConfig
) -> Result<LokiResponse,LokiError> {
    if loki_query.trim().is_empty() {
        return Err(LokiError { msg: "Query cannot be empty".to_string() });
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(loki_config.timeout as u64))
        .build()?;
    
    let url = format!("{}/loki/api/v1/query", loki_config.base_url.trim_end_matches('/'));
    let mut params = vec![("query".to_string(), loki_query.clone())];
    if let Some(limit) = loki_query_params.limit {
        params.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(direction) = loki_query_params.direction {
        let direction_str = match direction {
            LokiDirection::Forward => "forward",
            LokiDirection::Backward => "backward"
        };
        params.push(("direction".to_string(), direction_str.to_string()));
    }

    if let Some(time) = loki_query_params.time {
        params.push(("time".to_string(), time.to_string()));
    }

    let resp = client.get(&url).query(&params).send().await?;
    let loki_response = resp.json::<LokiResponse>().await?;
    Ok(loki_response)
}