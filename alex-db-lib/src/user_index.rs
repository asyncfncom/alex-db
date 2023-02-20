use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::RwLock};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub api_key: RwLock<BTreeMap<Uuid, Uuid>>,
    pub created_at: RwLock<BTreeMap<i64, Uuid>>,
    pub updated_at: RwLock<BTreeMap<i64, Uuid>>,
    pub username: RwLock<BTreeMap<String, Uuid>>,
}

impl UserIndex {
    pub fn new() -> Self {
        Self {
            api_key: RwLock::new(BTreeMap::new()),
            created_at: RwLock::new(BTreeMap::new()),
            updated_at: RwLock::new(BTreeMap::new()),
            username: RwLock::new(BTreeMap::new()),
        }
    }
}

impl Default for UserIndex {
    fn default() -> Self {
        Self::new()
    }
}
