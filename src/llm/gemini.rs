use gemini_rust::{ClientError, Gemini, Model};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GeminiError {
    pub msg: String,
}

impl Error for GeminiError {}

impl fmt::Display for GeminiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Something went wrong with Gemini {0}", self.msg)
    }
}

impl From<ClientError> for GeminiError {
    fn from(err: ClientError) -> Self {
        GeminiError {
            msg: format!("Gemini request failed {:?}", err),
        }
    }
}

impl From<serde_json::Error> for GeminiError {
    fn from(err: serde_json::Error) -> Self {
        GeminiError {
            msg: format!("JSON parsing of response failed {:?}", err),
        }
    }
}

#[derive(Debug)]
pub struct GeminiConfig {
    pub model: Model,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        GeminiConfig {
            model: Model::Gemini25FlashLite,
        }
    }
}

pub async fn gen_summary(prompt: &str, gemini_config: GeminiConfig) -> Result<String, GeminiError> {
    println!("Hello from gemini");
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let client = Gemini::with_model(api_key, gemini_config.model)
        .expect("unable to create Gemini API client");
    let response = client
        .generate_content()
        .with_system_prompt("Your goal is to explain/summarize the result of the prometheus query. Only give the explain/summary of result and nothing else")
        .with_user_message(prompt)
        .execute()
        .await?;
    Ok(response.text())
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrometheusQuery {
    pub query: String,
    pub explanation: String,
}

/// Generate prometheus query based on natural language
pub async fn gen_query(
    prompt: &str,
    gemini_config: GeminiConfig,
) -> Result<PrometheusQuery, GeminiError> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let client = Gemini::with_model(api_key, gemini_config.model)
        .expect("unable to create Gemini API client");
    let schema = json!({
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Prometheus query that user asked for"
            },
            "explanation": {
                "type": "string",
                "description": "reasoning behind writing this query"
            }
        },
        "required": ["query", "explanation"]
    });
    let response = client.generate_content()
            .with_system_prompt("You write prometheus query based on user requirement. Always prefix http_request metric lable with bob_")
            .with_user_message(prompt)
            .with_response_mime_type("application/json")
            .with_response_schema(schema)
            .execute()
            .await?;
    let query: PrometheusQuery = serde_json::from_str(&response.text())?;
    Ok(query)
}
