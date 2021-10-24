#[macro_use]

use std::path::PathBuf;
use std::fs;
use std::io;

use tantivy::schema::*;
use tantivy::Index;

use crate::info_retrieval::types::SchemaField;

fn add_field(schema_builder: &mut SchemaBuilder, schema_field: SchemaField) {
    /*
    v1 https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3b2218ea51b4682d4bde2458c790a699
    v2 (with loop and pattern matching) https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a040ffb9ac6164801415b33e567af390
    */

    match schema_field {
        SchemaField::Text {
            column_name,
            stored,
            indexed,
            indexed_lang_stem,
            indexed_tokenized,
            indexed_tokenized_with_freqs_positions,
            indexed_tokenized_with_freqs,
        } => {
            let mut text_options = TextOptions::default();
            if stored {
                text_options.set_stored();
            }
            let mut text_indexing_options = TextFieldIndexing::default();
            if indexed {
                text_indexing_options = text_indexing_options.set_index_option(IndexRecordOption::Basic);
                if indexed_lang_stem == "en" { // TODO replace magic string
                    text_indexing_options = text_indexing_options.set_tokenizer("en_stem");
                }
                // TODO throw an error if lang not supported.....
                if indexed_tokenized {
                    if indexed_tokenized_with_freqs {
                        if indexed_tokenized_with_freqs_positions {
                            text_indexing_options =
                                text_indexing_options.set_index_option(IndexRecordOption::WithFreqsAndPositions);
                        } else {
                            text_indexing_options =
                                text_indexing_options.set_index_option(IndexRecordOption::WithFreqs);
                        }
                    }
                } else {
                    text_indexing_options = text_indexing_options.set_tokenizer("raw");
                }
                schema_builder.add_text_field(column_name.as_str(), text_options);
            }
        }
        SchemaField::Keyword { column_name, stored } => {
            let mut text_options = TextOptions::default();
            text_options.set_stored();
            let mut text_indexing_options = TextFieldIndexing::default().set_index_option(IndexRecordOption::Basic);
            text_indexing_options = text_indexing_options.set_tokenizer("raw");
            text_options = text_options.set_indexing_options(text_indexing_options);
            schema_builder.add_text_field(column_name.as_str(), text_options);
        }
        SchemaField::UInt64 {
            column_name,
            stored,
            indexed,
            doc_values,
        } => {
            let mut int_options = IntOptions::default();
            if stored {
                int_options = int_options.set_stored();
            }
            if indexed {
                int_options = int_options.set_indexed();
            }
            if doc_values {
                int_options = int_options.set_fast(Cardinality::SingleValue);
            }
            schema_builder.add_u64_field(column_name.as_str(), int_options);
        }
        SchemaField::Int64 {
            column_name,
            stored,
            indexed,
            doc_values,
        } => {
            let mut int_options = IntOptions::default();
            if stored {
                int_options = int_options.set_stored();
            }
            if indexed {
                int_options = int_options.set_indexed();
            }
            if doc_values {
                int_options = int_options.set_fast(Cardinality::SingleValue);
            }
            schema_builder.add_i64_field(column_name.as_str(), int_options);
        }
        SchemaField::Float64 {
            column_name,
            stored,
            indexed,
            doc_values,
        } => {
            let mut int_options = IntOptions::default();
            if stored {
                int_options = int_options.set_stored();
            }
            if indexed {
                int_options = int_options.set_indexed();
            }
            if doc_values {
                int_options = int_options.set_fast(Cardinality::SingleValue);
            }
            schema_builder.add_f64_field(column_name.as_str(), int_options);
        }
        SchemaField::Date {
            column_name,
            stored,
            indexed,
            doc_values,
        } => {
            let mut int_options = IntOptions::default();
            if stored {
                int_options = int_options.set_stored();
            }
            if indexed {
                int_options = int_options.set_indexed();
            }
            if doc_values {
                int_options = int_options.set_fast(Cardinality::SingleValue);
            }
            schema_builder.add_date_field(column_name.as_str(), int_options);
        }
        SchemaField::Facet { column_name } => {
            schema_builder.add_facet_field(column_name.as_str());
        }
        SchemaField::Bytes { column_name } => {
            schema_builder.add_bytes_field(column_name.as_str());
        }
    }
}

fn run_new(directory: PathBuf, schema_fields: Vec<SchemaField>) -> tantivy::Result<()> {
    let mut schema_builder = SchemaBuilder::default();
    for schema_field in schema_fields.iter() {
        add_field(&mut schema_builder, *schema_field)
    }
    let schema = schema_builder.build();

    // println!("\n{}\n", Style::new().fg(Green).paint(schema_json));
    match fs::create_dir(&directory) {
        Ok(_) => (),
        // Proceed here; actual existence of index is checked in Index::create_in_dir
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        Err(e) => panic!("{:?}", e),
    };
    Index::create_in_dir(&directory, schema)?;
    Ok(())
}

