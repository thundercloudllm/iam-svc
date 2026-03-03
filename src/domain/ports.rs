use async_trait::async_trait;
use super::error::IamError;
use super::model::{ClientRecord, TokenResponse};

/// Driven port: persistence of client records.
#[async_trait]
pub trait ClientRepository: Send + Sync + 'static {
    async fn find_by_id(&self, client_id: &str) -> Option<ClientRecord>;
    async fn validate_secret(&self, client_id: &str, secret: &str) -> bool;
}

/// Driving port: token issuance use case.
#[async_trait]
pub trait TokenService: Send + Sync + 'static {
    async fn issue_token(
        &self,
        grant_type: &str,
        client_id: &str,
        client_secret: &str,
        requested_scope: Option<&str>,
    ) -> Result<TokenResponse, IamError>;
}
