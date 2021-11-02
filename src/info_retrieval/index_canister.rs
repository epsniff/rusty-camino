
use std::path::PathBuf;

use dashmap::DashMap;


use crate::info_retrieval::types::IndexServer;
use crate::info_retrieval::types::CanisterSettings;
use crate::info_retrieval::types::IndexSettings;

use crate::info_retrieval::local_shard::Shard;
use crate::info_retrieval::new_index::run_new;
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

    pub fn add_index(&self, schema: &str, settings: IndexSettings) -> Result<String> {
        let schema_fields = serde_json::from_str(schema).expect("error deserializing schema");
        let index = run_new(&self.base_path, schema_fields).expect("error creating index");
        let name = format!("{}-1", settings.index_name);
        let response = format!("index: {}, index_create", settings.index_name);
        let shard = Shard::new(index, settings, &name[..])?;
        self.shards.insert(name, shard).expect("error adding to map");
        Ok(response)
    }
}
