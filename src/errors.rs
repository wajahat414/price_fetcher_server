use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    Ws(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Binance error: {0}")]
    Binance(#[from] binance::errors::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Other: {0}")]
    Other(String),
}

// Display is derived via `thiserror::Error` using the #[error(..)] attributes above.
