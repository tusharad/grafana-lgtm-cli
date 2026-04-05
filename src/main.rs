mod client;
mod llm;

use crate::client::prometheus::{
    PrometheusConfig, PrometheusQueryParams, PrometheusResponse, run_prometheus,
};
use crate::llm::gemini::{GeminiConfig, PrometheusQuery, gen_query, gen_summary};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prom_query: PrometheusQuery = gen_query(
        "highest number of request per seconds in last 1 hour",
        GeminiConfig::default(),
    )
    .await?;
    let prom_resp: PrometheusResponse = run_prometheus(
        prom_query.query.clone(),
        PrometheusQueryParams::default(),
        PrometheusConfig::default(),
    )
    .await?;
    let query = prom_query.query.clone();
    let prompt: String = format!("{query}\n Result:{:?}", prom_resp);
    println!("prompt {:?}", prompt);
    let res = gen_summary(&prompt, GeminiConfig::default()).await;
    println!("{:?}", res);
    Ok(())
}
