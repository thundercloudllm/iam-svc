use serde::{Deserialize, Serialize};

/// A registered OAuth2 client.
#[derive(Debug, Clone)]
pub struct ClientRecord {
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<String>,
}

/// Inbound token request (application/x-www-form-urlencoded).
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scope: Option<String>,
}

/// Outbound token response per RFC 6749 section 5.1.
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
}

/// JWT claims embedded in issued tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
    pub scope: String,
    pub jti: String,
}
