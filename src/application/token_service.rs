use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::domain::error::IamError;
use crate::domain::model::{Claims, TokenResponse};
use crate::domain::ports::{ClientRepository, TokenService};

const TOKEN_TTL_SECS: u64 = 3600;

/// Application service: implements the token issuance use case.
pub struct TokenServiceImpl {
    repo: Arc<dyn ClientRepository>,
    signing_key: String,
    issuer: String,
}

impl TokenServiceImpl {
    pub fn new(
        repo: Arc<dyn ClientRepository>,
        signing_key: String,
        issuer: String,
    ) -> Self {
        Self { repo, signing_key, issuer }
    }
}

#[async_trait]
impl TokenService for TokenServiceImpl {
    async fn issue_token(
        &self,
        grant_type: &str,
        client_id: &str,
        client_secret: &str,
        requested_scope: Option<&str>,
    ) -> Result<TokenResponse, IamError> {
        // Only client_credentials grant is supported
        if grant_type != "client_credentials" {
            return Err(IamError::UnsupportedGrantType(grant_type.to_string()));
        }

        // Authenticate the client
        if !self.repo.validate_secret(client_id, client_secret).await {
            return Err(IamError::InvalidCredentials);
        }

        // Resolve granted scopes
        let client = self.repo.find_by_id(client_id).await
            .ok_or(IamError::InvalidCredentials)?;

        let granted_scopes = match requested_scope {
            Some(req) => {
                let requested: Vec<&str> = req.split_whitespace().collect();
                for s in &requested {
                    if !client.scopes.iter().any(|cs| cs == s) {
                        return Err(IamError::InvalidScope(s.to_string()));
                    }
                }
                requested.into_iter().map(String::from).collect::<Vec<_>>()
            }
            None => client.scopes.clone(),
        };

        let scope_str = granted_scopes.join(" ");
        let now = Utc::now().timestamp();

        let claims = Claims {
            sub: client_id.to_string(),
            iss: self.issuer.clone(),
            exp: now + TOKEN_TTL_SECS as i64,
            iat: now,
            scope: scope_str.clone(),
            jti: Uuid::new_v4().to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.signing_key.as_bytes()),
        ).map_err(|e| IamError::SigningFailed(e.to_string()))?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".into(),
            expires_in: TOKEN_TTL_SECS,
            scope: scope_str,
        })
    }
}
