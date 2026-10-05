//! medical-document-summarizer — clinical note → structured summary.
//!
//! Two extraction backends: an **LLM extractor** (Ollama, JSON-schema
//! constrained — requires a GPU) and a **rule extractor** (deterministic
//! keyword/section parsing, the default and the one tests use). Both
//! produce the same typed `StructuredSummary`; the caller can't tell
//! them apart except by a `backend` field — graceful degradation.

pub mod extract;
pub mod routes;
pub mod state;

pub use state::AppState;
