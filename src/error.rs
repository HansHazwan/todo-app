
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Static Error: {0}")]
    Static(String),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Serde Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

