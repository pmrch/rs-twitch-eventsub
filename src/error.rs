#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable parsing error: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Logger error: {0}")]
    TracingError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error("Logging directive parsing error: {0}")]
    DirectiveParseError(#[from] tracing_subscriber::filter::ParseError),

    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Dotenv init error: {0}")]
    DotenvError(#[from] dotenv::Error),

    #[error("None type extraction error: {0}")]
    NoneError(String),

    #[error("General error: {0}")]
    OtherError(#[from] anyhow::Error),
}
