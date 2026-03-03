use std::sync::Arc;

use axum::{
    Form, Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

use crate::domain::error::IamError;
use crate::domain::model::{TokenRequest, TokenResponse};
use crate::domain::ports::TokenService;

/// Build the Axum router with all inbound HTTP adapters.
pub fn create_router(token_svc: Arc<dyn TokenService>) -> Router {
    Router::new()
        .route("/token", post(token_handler))
        .route("/health", get(health))
        .with_state(token_svc)
}

/// POST /token - OAuth2 client_credentials token endpoint.
async fn token_handler(
    State(svc): State<Arc<dyn TokenService>>,
    Form(req): Form<TokenRequest>,
) -> Result<Json<TokenResponse>, TokenError> {
    let client_id = req.client_id.as_deref().unwrap_or_default();
    let client_secret = req.client_secret.as_deref().unwrap_or_default();

    let resp = svc
        .issue_token(&req.grant_type, client_id, client_secret, req.scope.as_deref())
        .await?;

    Ok(Json(resp))
}

/// GET /health - liveness probe.
async fn health() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

/// Map domain errors to HTTP responses per RFC 6749 section 5.2.
struct TokenError(IamError);

impl From<IamError> for TokenError {
    fn from(e: IamError) -> Self { Self(e) }
}

impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_code) = match &self.0 {
            IamError::UnsupportedGrantType(_) => (StatusCode::BAD_REQUEST, "unsupported_grant_type"),
            IamError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_client"),
            IamError::InvalidScope(_) => (StatusCode::BAD_REQUEST, "invalid_scope"),
            IamError::SigningFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "server_error"),
        };
        (status, Json(json!({"error": error_code, "error_description": self.0.to_string()}))).into_response()
    }
}
