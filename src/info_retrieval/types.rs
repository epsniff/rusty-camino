use std::path::PathBuf;

use tantivy::IndexWriter;

/// Defines the interface for obtaining a handle from a catalog to an index
#[async_trait::async_trait]
pub trait IndexServer {
    // TODO fn find_shard(indexname str, shardid u64) -> Server
    /// The current catalog's raft_id
    fn raft_id(&self) -> u64;
}

pub trait IndexHandle {
    fn get_name(&self) -> String ;
    fn index_location(&self) -> IndexLocation ;
    fn get_index(&self) -> Index ;
    fn get_writer(&self) -> Arc<Mutex<IndexWriter>>;
    fn get_space(&self) -> SearcherSpaceUsage;
    async fn search_index(&self, search: Search) -> Result<SearchResults> ;
    async fn add_document(&self, add_doc: AddDocument) -> Result<()> ;
    async fn delete_term(&self, term: DeleteDoc) -> Result<DocsAffected> ;
}


pub struct CanisterSettings {
    base_path: PathBuf,

}

pub struct IndexSettings {
    index_name: str,
}

//////////////////////////////////////////////////////////
// Schema data Types
//
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
enum SchemaField {
    Text    { column_name: String, stored: bool, indexed: bool, indexed_lang_stem: String, indexed_tokenized: bool, indexed_tokenized_with_freqs_positions: bool, indexed_tokenized_with_freqs: bool },
    Keyword { column_name: String, stored: bool },
    UInt64  { column_name: String, stored: bool, indexed: bool, doc_vals: bool },
    Int64   { column_name: String, stored: bool, indexed: bool, doc_vals: bool },
    Float64 { column_name: String, stored: bool, indexed: bool, doc_vals: bool },
    Date    { column_name: String, stored: bool, indexed: bool, doc_vals: bool },
    Facet   { column_name: String },
    Bytes   { column_name: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn schema_serialize_deserialize() {
        let data_a = r#"
        [
            {
                "type": "text",
                "column_name": "a_foo",
                "stored": true,
                "indexed": true,
                "indexed_lang_stem": "en",
                "indexed_tokenized": true, 
                "indexed_tokenized_with_freqs_positions": true,
                "indexed_tokenized_with_freqs": true
            },
            {
                "type": "keyword",
                "column_name": "b_foo",
                "stored": true
            },
            {
                "type": "uint64",
                "column_name": "c_foo",
                "stored": true,
                "doc_values": true, 
                "indexed": true
            },
            {
                "type": "int64",
                "column_name": "d_foo",
                "stored": true,
                "doc_values": true, 
                "indexed": true
            },
            {
                "type": "float64",
                "column_name": "e_foo",
                "stored": true,
                "doc_values": true, 
                "indexed": true
            },
            {
                "type": "date",
                "column_name": "f_foo",
                "stored": true,
                "doc_values": true, 
                "indexed": true
            },
            {
                "type": "facet",
                "column_name": "g_foo"
            },
            {
                "type": "bytes",
                "column_name": "h_foo"
            }
        ]   "#;
        let schema_fields: Vec<SchemaField> = serde_json::from_str(data_a)?;
        assert_eq!(2 + 2, 4);
    }
}

