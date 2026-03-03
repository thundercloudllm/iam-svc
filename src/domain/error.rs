use thiserror::Error;

/// Domain errors for the IAM bounded context.
#[derive(Debug, Error)]
pub enum IamError {
    #[error("unsupported grant type: {0}")]
    UnsupportedGrantType(String),

    #[error("invalid client credentials")]
    InvalidCredentials,

    #[error("requested scope not permitted: {0}")]
    InvalidScope(String),

    #[error("token signing failed: {0}")]
    SigningFailed(String),
}
