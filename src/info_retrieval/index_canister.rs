
use std::path::PathBuf;

use dashmap::DashMap;


use crate::info_retrieval::types::IndexServer;
use crate::info_retrieval::types::CanisterSettings;
use crate::info_retrieval::types::IndexSettings;

use crate::info_retrieval::local_shard::Shard;
use crate::Result;

#[allow(dead_code)] // TODO turn off allow dead_code here after canister is fulling implemented
pub struct IndexCanister {
    settings: CanisterSettings,
    shards: DashMap<String, Shard>,
    base_path: PathBuf,
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
        let index_can = IndexCanister {
            settings: settings,
            shards: local_idxs,
            base_path: base_path,
        };

        Ok(index_can)
    }

    pub fn add_index(&self, settings: IndexSettings) -> Result<String> {
    Ok(format!(
        "index: {}, index_create",
        settings.index_name,
    ))
    }
}
