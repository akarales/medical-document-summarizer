//! Shared state.

#[derive(Debug, Clone)]
pub struct AppState {
    pub http: reqwest::Client,
    pub config: Config,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// false = use the deterministic rule extractor (offline default).
    pub use_llm: bool,
    pub ollama_url: String,
    pub ollama_model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_llm: false,
            ollama_url: "http://localhost:11434".into(),
            ollama_model: "qwen3:14b".into(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            http: reqwest::Client::new(),
            config: Config::default(),
        }
    }
}
