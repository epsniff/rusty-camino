
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

    pub fn get_shard(&self, index_name: &str) -> Result<Shard> {
        let name = format!("{}-1", index_name);
        self.shards.get(&name)
        .map(|r| r.value().to_owned())
        .ok_or_else(|| crate::Error::new(format!("get failed: {}", name)))
    }

    pub fn add_index(&self, schema: &str, settings: IndexSettings) -> Result<String> {
        let name = format!("{}-1", settings.index_name);
        match self.get_shard(&settings.index_name){
            Ok(_) => return Ok(format!("index: {}, already exists", name)),
            Err(_) => (),
        };
        let schema_fields = serde_json::from_str(schema).expect("error deserializing schema");
        let mut shard_path: PathBuf = self.base_path.clone();
        shard_path.push(&settings.index_name);
        //shard_path.push("1");
        let index = run_new(&shard_path, schema_fields).expect("error creating index");
        let shard = Shard::new(index, settings, &name[..])?;
        let response = format!("index: {}, index_create", name);
        self.shards.insert(name, shard);
        Ok(response)
    }
}
