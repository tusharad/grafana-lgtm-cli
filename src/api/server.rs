use axum::{Router, routing::post};

use crate::api::handlers::ask_handler;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/api/v1/ask", post(ask_handler));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
