use std::sync::Arc;

mod domain;
mod application;
mod adapters;

use adapters::inbound::http::create_router;
use adapters::outbound::memory_store::InMemoryClientStore;
use application::token_service::TokenServiceImpl;
use domain::model::ClientRecord;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("iam_svc=debug,tower_http=debug")
        .init();

    let store = InMemoryClientStore::new();

    // Seed a default client for development/testing
    store.insert(ClientRecord {
        client_id: "demo-client".into(),
        client_secret: "demo-secret".into(),
        scopes: vec!["read".into(), "write".into()],
    }).await;

    let signing_key = std::env::var("JWT_SIGNING_KEY")
        .unwrap_or_else(|_| "super-secret-dev-key-change-me".into());
    let issuer = std::env::var("TOKEN_ISSUER")
        .unwrap_or_else(|_| "iam-svc".into());

    let token_svc = Arc::new(TokenServiceImpl::new(
        Arc::new(store),
        signing_key,
        issuer,
    ));

    let app = create_router(token_svc);

    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();
    tracing::info!("IAM service listening on {bind}");
    axum::serve(listener, app).await.unwrap();
}
