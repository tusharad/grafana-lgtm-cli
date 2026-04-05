use crate::client;
use crate::llm;

use crate::client::prometheus::{
    PrometheusConfig, PrometheusQueryParams, PrometheusResponse, run_prometheus,
};
use crate::llm::gemini::{GeminiConfig, PrometheusQuery, gen_query, gen_summary};

pub async fn process_natural_language_query(
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let prom_query: PrometheusQuery = gen_query(prompt, GeminiConfig::default()).await?;
    let prom_resp: PrometheusResponse = run_prometheus(
        prom_query.query.clone(),
        PrometheusQueryParams::default(),
        PrometheusConfig::default(),
    )
    .await?;
    let query = prom_query.query.clone();
    let prompt: String = format!("{query}\n Result:{:?}", prom_resp);
    println!("prompt {:?}", prompt);
    let summary = gen_summary(&prompt, GeminiConfig::default()).await?;
    Ok(summary)
}
