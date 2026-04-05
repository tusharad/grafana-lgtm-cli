use crate::core::engine::process_natural_language_query;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AskRequest {
    prompt: String,
}

#[derive(Serialize)]
pub struct AskResponse {
    summary: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn ask_handler(
    Json(payload): Json<AskRequest>,
) -> Result<Json<AskResponse>, Json<ErrorResponse>> {
    match process_natural_language_query(&payload.prompt).await {
        Ok(summary) => Ok(Json(AskResponse { summary })),
        Err(e) => Err(Json(ErrorResponse {
            error: e.to_string(),
        })),
    }
}
