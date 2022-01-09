use async_trait::async_trait;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use tantivy::space_usage::SearcherSpaceUsage;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;

use crate::info_retrieval::types::*;
use crate::Result;

/// A Shard is a local copy of one partition for an Index
pub struct Shard {
    index: Index,
    writer: Arc<Mutex<IndexWriter>>,
    reader: IndexReader,
    current_opstamp: Arc<AtomicUsize>,
    deleted_docs: Arc<AtomicU64>,
    settings: IndexSettings,
    name: String,
}

impl Clone for Shard {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            writer: Arc::clone(&self.writer),
            reader: self.reader.clone(),
            current_opstamp: Arc::clone(&self.current_opstamp),
            deleted_docs: Arc::clone(&self.deleted_docs),
            settings: self.settings.clone(),
            name: self.name.clone(),
        }
    }
}

impl PartialEq for Shard {
    fn eq(&self, other: &Shard) -> bool {
        self.name == *other.name
    }
}

impl Eq for Shard {}

impl Hash for Shard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
    }
}

#[async_trait]
impl IndexHandle for Shard {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_index(&self) -> Index {
        self.index.clone()
    }

    fn get_writer(&self) -> Arc<Mutex<IndexWriter>> {
        Arc::clone(&self.writer)
    }

    fn get_space(&self) -> SearcherSpaceUsage {
        self.reader.searcher().space_usage()
    }
  
    async fn search_index(&self, query: &str) -> Result<SearchResults> {
        let searcher = self.reader.searcher();

        // ### Query
    
        // The query parser can interpret human queries.
        // Here, if the user does not specify which
        // field they want to search, tantivy will search
        // in both title and body.
        let query_parser = QueryParser::for_index(&self.index, vec![]);
        let q = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&q, &TopDocs::with_limit(5).and_offset(0))?;
        let mut places = vec![];
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            places.push(self.index.schema().to_json(&retrieved_doc));
        }
        return Ok(SearchResults{
           docs: places, 
        })
    }

    async fn add_document(&self, data: &str) -> Result<()> {
        
        let index_schema = self.index.schema();
        let w_mutext = self.get_writer();

        let d = index_schema.parse_document(data);
        // format!("Failed to parse document {:?}", e)
        match d {
            Ok(doc) => {
                let mut writer = w_mutext.lock().unwrap();
                writer.add_document(doc);
                // ### Committing
                //
                // At this point our documents are not searchable.
                //
                //
                // We need to call `.commit()` explicitly to force the
                // `index_writer` to finish processing the documents in the queue,
                // flush the current index to the disk, and advertise
                // the existence of new documents.
                //
                // This call is blocking.
                let a = writer.commit();
                log::debug!("indexed doc committed: checkpoint:{}", a.unwrap());
            }
            Err(e) => return Err(crate::Error::new(format!("Failed to parse document {:?}", e))),
        }
        return Ok(());
    }
}

impl Shard {
    pub fn new(index: Index, settings: IndexSettings, name: &str) -> Result<Self> {
        // let i = index.writer(settings.writer_memory)?;

        let i = match index.writer(settings.writer_memory) {
            Ok(i) => i,
            Err(e) => return Err(crate::Error::new(format!("Failed to index write {:?}", e))),
        };

        i.set_merge_policy(settings.get_merge_policy());
        let current_opstamp = Arc::new(AtomicUsize::new(0));
        let writer = Arc::new(Mutex::new(i));

        let reader = match index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into() {
            Ok(reader) => reader,
            Err(e) => return Err(crate::Error::new(format!("Failed to create index reader {:?}", e))),
        };
        Ok(Self {
            index,
            reader,
            writer,
            current_opstamp,
            deleted_docs: Arc::new(AtomicU64::new(0)),
            settings,
            name: name.into(),
        })
    }

    pub fn recreate_writer(self) -> Result<Self> {
        Shard::new(self.index, self.settings.clone(), &self.name)
    }

    pub fn get_opstamp(&self) -> usize {
        log::trace!("Got the opstamp");
        self.current_opstamp.load(Ordering::SeqCst)
    }

    pub fn set_opstamp(&self, opstamp: usize) {
        log::trace!("Setting stamp to {}", opstamp);
        self.current_opstamp.store(opstamp, Ordering::SeqCst)
    }
}
