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

    // async fn search_index(&self, search: Search) -> Result<SearchResults> {
    //     let searcher = self.reader.searcher();
    //     let schema = self.index.schema();
    //     let mut multi_collector = MultiCollector::new();
    //
    //     let sorted_top_handle = search.sort_by.clone().and_then(|sort_by| {
    //         log::info!("Sorting with: {}", sort_by);
    //         if let Some(f) = schema.get_field(&sort_by) {
    //             let entry = schema.get_field_entry(f);
    //             if entry.is_int_fast() && entry.is_stored() {
    //                 let c = TopDocs::with_limit(search.limit).order_by_u64_field(f);
    //                 return Some(multi_collector.add_collector(c));
    //             }
    //         }
    //         None
    //     });
    //
    //     let top_handle = multi_collector.add_collector(TopDocs::with_limit(search.limit));
    //     let facet_handle = search.facets.clone().and_then(|f| {
    //         if let Some(field) = schema.get_field(&f.get_facets_fields()) {
    //             let mut col = FacetCollector::for_field(field);
    //             for term in f.get_facets_values() {
    //                 col.add_facet(&term);
    //             }
    //             Some(multi_collector.add_collector(col))
    //        } else {
    //            None
    //        }
    //    });
    //
    //    if let Some(query) = search.query {
    //        let gen_query = match query {
    //            Query::Regex(regex) => regex.create_query(&schema)?,
    //            // Query::Phrase(phrase) => phrase.create_query(&schema)?,
    //            // Query::Fuzzy(fuzzy) => fuzzy.create_query(&schema)?,
    //            // Query::Exact(term) => term.create_query(&schema)?,
    //            // Query::Range(range) => range.create_query(&schema)?,
    //            // Query::Boolean { bool } => bool.create_query(&schema)?,
    //            // Query::Raw { raw } => {
    //            //     let fields: Vec<Field> = schema.fields().filter_map(|f| schema.get_field(f.1.name())).collect();
    //            //     let query_parser = QueryParser::for_index(&self.index, fields);
    //            //     query_parser.parse_query(raw.as_str())?
    //            // }
    //            Query::All => Box::new(AllQuery),
    //        };
    //
    //        log::trace!("{:?}", gen_query);
    //        let mut scored_docs = searcher.search(&*gen_query, &multi_collector)?;
    //
    //        // FruitHandle isn't a public type which leads to some duplicate code like this.
    //        let docs: Vec<ScoredDoc<FlatNamedDocument>> = if let Some(h) = sorted_top_handle {
    //            h.extract(&mut scored_docs)
    //                .into_iter()
    //                .map(|(score, doc)| {
    //                    let d = searcher.doc(doc).expect("Doc not found in segment");
    //                    ScoredDoc::<FlatNamedDocument>::new(Some(score as f32), schema.to_named_doc(&d).into())
    //                })
    //                .collect()
    //        } else {
    //            top_handle
    //                .extract(&mut scored_docs)
    //                .into_iter()
    //                .map(|(score, doc)| {
    //                    let d = searcher.doc(doc).expect("Doc not found in segment");
    //                    ScoredDoc::<FlatNamedDocument>::new(Some(score), schema.to_named_doc(&d).into())
    //                })
    //                .collect()
    //        };
    //
    //        if let Some(facets) = facet_handle {
    //            if let Some(t) = &search.facets {
    //                let facet_counts = facets
    //                    .extract(&mut scored_docs)
    //                    .get(&t.get_facets_values()[0])
    //                    .map(|(f, c)| KeyValue::new(f.to_string(), c))
    //                    .collect();
    //                return Ok(SearchResults::with_facets(docs, facet_counts));
    //            }
    //        }
    //        Ok(SearchResults::new(docs))
    //    } else {
    //        Err(Error::QueryError("Empty Query Provided".into()))
    //    }
    //}
    //
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
