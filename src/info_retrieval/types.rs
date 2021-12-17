
use std::sync::Arc;
use std::sync::Mutex;
use std::path::PathBuf;
use async_trait::async_trait;

use tantivy::Index;
use tantivy::space_usage::SearcherSpaceUsage;
use tantivy::IndexWriter;
use tantivy::merge_policy::*;
use tantivy::{Document};

use serde::{Deserialize, Serialize};
use crate::Result;


/// Defines the interface for obtaining a handle from a catalog to an index
#[async_trait::async_trait]
pub trait IndexServer {
    // TODO fn find_shard(index_name str, shard_id u64) -> Server
    /// The current catalog's raft_id
    fn raft_id(&self) -> u64;
}

#[async_trait]
pub trait IndexHandle {
    fn get_name(&self) -> String ;
    fn get_index(&self) -> Index ;
    fn get_writer(&self) -> Arc<Mutex<IndexWriter>>;
    fn get_space(&self) -> SearcherSpaceUsage;
    async fn search_index(&self, search: &str) -> Result<SearchResults> ;
    async fn add_document(&self, doc: Document) -> Result<()> ;
}

/// The request body of a search POST
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    /// Field to sort results by
    #[serde(default)]
    pub sort_by: Option<String>,
    
    #[serde(default)]
    pub query: String,

    #[serde(default)]
    pub limit: u64,
}


pub struct CanisterSettings {
    pub base_path: PathBuf,
    pub server_id: u64, 
}

pub struct IndexSettings {
    pub index_name: String,
    pub writer_memory: usize,
    pub merge_policy: String,
}

impl IndexSettings{
    pub fn get_merge_policy(&self) -> Box<dyn MergePolicy> {
        match self.merge_policy.as_ref() {
            // TODO convert these to an ENUM and make them Serde types, like SchemaField
            "merge_log" => {
                let mp = LogMergePolicy::default();
                //mp.set_level_log_size(self.merge_policy.level_log_size);
                //mp.set_min_layer_size(self.merge_policy.min_layer_size);
                //mp.set_min_merge_size(self.merge_policy.min_merge_size);
                Box::new(mp)
            }
            "merge_no_merge" => Box::new(NoMergePolicy::default()),
            _ => {
                let mp = LogMergePolicy::default();
                Box::new(mp)
            }
        }
    }
}

impl Clone for IndexSettings {
     fn clone(&self) -> Self {
        Self {
            index_name: self.index_name.clone(), 
            writer_memory: self.writer_memory,
            merge_policy: self.merge_policy.clone(),
        }
     }
}

pub struct Query {}
pub struct SearchResults {
    pub docs: Vec<String>,
}

/*
/// Documents are really just a list of couple `(field, value)`.
/// In this list, one field may appear more than once.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Document {
    field_values: Vec<FieldValue>,
}
*/


//////////////////////////////////////////////////////////
// Schema data Types
//
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum SchemaField {
    Text    { column_name: String, stored: bool, indexed: bool, 
        indexed_lang_stem: String, 
        indexed_tokenized: bool, indexed_tokenized_with_freqs_positions: bool, 
        indexed_tokenized_with_freqs: bool },
    Keyword { column_name: String, stored: bool },
    UInt64  { column_name: String, stored: bool, indexed: bool, doc_values: bool },
    Int64   { column_name: String, stored: bool, indexed: bool, doc_values: bool },
    Float64 { column_name: String, stored: bool, indexed: bool, doc_values: bool },
    Date    { column_name: String, stored: bool, indexed: bool, doc_values: bool },
    Facet   { column_name: String },
    Bytes   { column_name: String },
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     use crate::info_retrieval::types::SchemaField;
// 
// fn schema_serialize_deserialize() {
//         let data_a = r#"
//         [
//             {
//                 "type": "text",
//                 "column_name": "a_foo",
//                 "stored": true,
//                 "indexed": true,
//                 "indexed_lang_stem": "en",
//                 "indexed_tokenized": true, 
//                 "indexed_tokenized_with_freqs_positions": true,
//                 "indexed_tokenized_with_freqs": true
//             },
//             {
//                 "type": "keyword",
//                 "column_name": "b_foo",
//                 "stored": true
//             },
//             {
//                 "type": "uint64",
//                 "column_name": "c_foo",
//                 "stored": true,
//                 "doc_values": true, 
//                 "indexed": true
//             },
//             {
//                 "type": "int64",
//                 "column_name": "d_foo",
//                 "stored": true,
//                 "doc_values": true, 
//                 "indexed": true
//             },
//             {
//                 "type": "float64",
//                 "column_name": "e_foo",
//                 "stored": true,
//                 "doc_values": true, 
//                 "indexed": true
//             },
//             {
//                 "type": "date",
//                 "column_name": "f_foo",
//                 "stored": true,
//                 "doc_values": true, 
//                 "indexed": true
//             },
//             {
//                 "type": "facet",
//                 "column_name": "g_foo"
//             },
//             {
//                 "type": "bytes",
//                 "column_name": "h_foo"
//             }
//         ]   "#;
//         let schema_fields: Vec<SchemaField> = serde_json::from_str(data_a)?;
//         assert_eq!(2 + 2, 4);
//     }
// }
// 
// 