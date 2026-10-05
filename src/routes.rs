//! Routes: summarize a note (LLM with rule fallback).

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::extract::{extract_llm, extract_rules};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/summarize", post(summarize))
        .with_state(state)
}

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }))
}

#[derive(Debug, Deserialize)]
pub struct SummarizeRequest {
    pub note: String,
}

pub async fn summarize(
    State(state): State<AppState>,
    Json(request): Json<SummarizeRequest>,
) -> Response {
    if request.note.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "note must not be empty" })),
        )
            .into_response();
    }

    // LLM when configured; rules otherwise; rules as fallback on LLM failure.
    let summary = if state.config.use_llm {
        match extract_llm(
            &state.http,
            &state.config.ollama_url,
            &state.config.ollama_model,
            &request.note,
        )
        .await
        {
            Ok(summary) => summary,
            Err(err) => {
                tracing::warn!(%err, "llm extraction failed; falling back to rules");
                extract_rules(&request.note)
            }
        }
    } else {
        extract_rules(&request.note)
    };

    (
        StatusCode::OK,
        Json(json!({
            "summary": summary,
            "input_chars": request.note.len(),
        })),
    )
        .into_response()
}
