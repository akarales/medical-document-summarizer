//! Contract tests over the deterministic extractor (no GPU, no network).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::Value;
use tower::ServiceExt;

use medical_document_summarizer::AppState;
use medical_document_summarizer::routes;

async fn post_note(note: &str) -> (StatusCode, Value) {
    let app = routes::router(AppState::default());
    let response = app
        .oneshot(
            Request::post("/api/v1/summarize")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({ "note": note }).to_string()))
                .expect("builds"),
        )
        .await
        .expect("request");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    (status, serde_json::from_slice(&bytes).expect("json"))
}

#[tokio::test]
async fn health_ok() {
    let app = routes::router(AppState::default());
    let response = app
        .oneshot(Request::get("/health").body(Body::empty()).expect("builds"))
        .await
        .expect("request");
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn note_extracts_to_structured_summary() {
    let note = "Patient with hypertension and diabetes on metformin. \
Reports chest pain.";
    let (status, body) = post_note(note).await;
    assert_eq!(status, StatusCode::OK);
    let summary = &body["summary"];
    assert_eq!(summary["backend"], "rule");
    assert!(
        summary["medications"]
            .as_array()
            .expect("meds")
            .iter()
            .any(|m| m == "metformin")
    );
    assert!(
        summary["conditions"]
            .as_array()
            .expect("conditions")
            .iter()
            .any(|c| c == "hypertension")
    );
    assert!(
        summary["disclaimer"]
            .as_str()
            .expect("disclaimer")
            .to_lowercase()
            .contains("not medical advice")
    );
}

#[tokio::test]
async fn empty_note_rejected() {
    let (status, body) = post_note("   ").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body["error"].is_string());
}
