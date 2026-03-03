use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;

use crate::domain::model::ClientRecord;
use crate::domain::ports::ClientRepository;

/// In-memory client store - driven adapter for persistence port.
pub struct InMemoryClientStore {
    clients: Arc<RwLock<HashMap<String, ClientRecord>>>,
}

impl InMemoryClientStore {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Seed a client record for bootstrapping.
    pub async fn insert(&self, record: ClientRecord) {
        self.clients.write().await
            .insert(record.client_id.clone(), record);
    }
}

#[async_trait]
impl ClientRepository for InMemoryClientStore {
    async fn find_by_id(&self, client_id: &str) -> Option<ClientRecord> {
        self.clients.read().await.get(client_id).cloned()
    }

    async fn validate_secret(&self, client_id: &str, secret: &str) -> bool {
        self.clients.read().await
            .get(client_id)
            .map(|c| c.client_secret == secret)
            .unwrap_or(false)
    }
}
