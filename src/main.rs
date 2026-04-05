mod client;
mod llm;

use crate::client::prometheus::{PrometheusConfig, PrometheusQueryParams, run_prometheus};
use crate::llm::gemini::run_gemini;

#[tokio::main]
async fn main() {
    let query: String = format!(
        "histogram_quantile(0.99, sum by (le) (rate(bob_http_request_duration_seconds_bucket[10m])))"
    );
    let res = run_prometheus(
        query.clone(),
        PrometheusQueryParams::default(),
        PrometheusConfig::default(),
    ).await;
    println!("Hello, world! {:?}", res);
    let prompt: String = format!("{query}\n Result:{:?}", res);
    let res = run_gemini(&prompt).await;
    println!("{:?}", res);
}
