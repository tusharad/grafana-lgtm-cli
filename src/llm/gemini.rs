use std::fmt;
use std::error::Error;
use std::env;
use gemini_rust::{Gemini, ClientError, Model};

#[derive(Debug)]
pub struct GeminiError {
    msg: String,
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

pub async fn run_gemini(prompt: &str) -> Result<String, GeminiError> {
    println!("Hello from gemini");
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let client = Gemini::with_model(api_key, Model::Gemini25FlashLite).expect("unable to create Gemini API client");
    let response = client
        .generate_content()
        .with_system_prompt("Your goal is to explain/summarize the result of the prometheus query. Only give the explain/summary and nothing else")
        .with_user_message(prompt)
        .execute()
        .await?;
    Ok(response.text())
}
