use std::clone::Clone;
use std::fs;
use std::sync::Arc;
use std::error;
use std::path::PathBuf;

use dashmap::DashMap;
use tantivy::directory::MmapDirectory;
use tantivy::schema::Schema;
use tantivy::Index;

use crate::info_retrieval::types::IndexServer;
use crate::info_retrieval::types::CanisterSettings;
use crate::info_retrieval::types::IndexSettings;

use crate::info_retrieval::local_shard::Shard;
use crate::Result;

pub type Canister = Arc<IndexCanister>;

pub struct IndexCanister {
    settings: CanisterSettings,
    shards: DashMap<String, Shard>,
}

#[async_trait::async_trait]
impl IndexServer for IndexCanister {
    fn raft_id(&self) -> u64 {
        self.settings.server_id
    }
}

impl IndexCanister {
    pub fn new(base_path: PathBuf, settings: CanisterSettings) -> Result<Self> {
        let local_idxs = DashMap::new();
        let mut index_can = IndexCanister {
            settings: settings,
            shards: local_idxs,
        };

        Ok(index_can)
    }

    pub fn add_new_shard(settings: IndexSettings) -> Result<()> {
        Ok(())
    }
}
