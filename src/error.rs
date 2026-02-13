use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required field: meta.username")]
    MissingUsername,

    #[error("unknown template: {0}")]
    UnknownTemplate(String),

    #[error("failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
