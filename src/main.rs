//! Binary entry.

use medical_document_summarizer::AppState;
use medical_document_summarizer::routes;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,medical_document_summarizer=debug".into()),
        )
        .init();

    let mut state = AppState::default();
    state.config.use_llm = std::env::var("APP_USE_LLM")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);
    if let Ok(url) = std::env::var("APP_OLLAMA_URL") {
        state.config.ollama_url = url;
    }
    if let Ok(model) = std::env::var("APP_OLLAMA_MODEL") {
        state.config.ollama_model = model;
    }

    let port: u16 = std::env::var("APP_PORT")
        .unwrap_or_else(|_| "8010".to_string())
        .parse()
        .expect("APP_PORT must be a valid port");

    let app = routes::router(state).layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    tracing::info!(%port, "medical document summarizer listening");
    axum::serve(listener, app).await?;
    Ok(())
}
