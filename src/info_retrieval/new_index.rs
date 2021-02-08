#[macro_use]
extern crate serde_json;

use serde_json::{Map, Number, Value};

use tantivy::schema::*;

pub struct IndexSchema {
    schema: serde_json::Map,
}

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
                text_indexing_options.set_index_option(IndexRecordOption::Basic);
                if schema_field.indexed_lang == "en" {
                    text_indexing_options.set_tokenizer("en_stem");
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
                schema_builder.add_text_field(column_name, text_options);
            }
        }
        SchemaField::Keyword { column_name, stored } => {
            let mut text_options = TextOptions::default();
            text_options.set_stored();
            let mut text_indexing_options = TextFieldIndexing::default().set_index_option(IndexRecordOption::Basic);
            text_indexing_options = text_indexing_options.set_tokenizer("raw");
            text_options = text_options.set_indexing_options(text_indexing_options);
            schema_builder.add_text_field(schema_field.column_name, text_options);
        }
        SchemaField::UInt64 {
            column_name,
            stored,
            indexed,
            doc_vals,
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
            schema_builder.add_u64_field(column_name, int_options);
        }
        SchemaField::Int64 {
            column_name,
            stored,
            indexed,
            doc_vals,
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
            schema_builder.add_i64_field(column_name, int_options);
        }
        SchemaField::Float64 {
            column_name,
            stored,
            indexed,
            doc_vals,
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
            schema_builder.add_f64_field(column_name, int_options);
        }
        SchemaField::Date {
            column_name,
            stored,
            indexed,
            doc_vals,
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
            schema_builder.add_date_field(column_name, int_options);
        }
        SchemaField::Facet { column_name } => {
            schema_builder.add_facet_field(column_name);
        }
        SchemaField::Bytes { column_name } => {
            schema_builder.add_bytes_field(column_name);
        }
    }
}

fn run_new(directory: PathBuf, schema_fields: Vec<SchemaField>) -> tantivy::Result<()> {
    let mut schema_builder = SchemaBuilder::default();
    for schema_field in schema_fields.iter() {
        add_field(schema_builder, schema_field)
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

// TEST
/*
    let exampleSchema = json!({
        "key1": "value",
        "key2": ["val", "val", "val"],
        "key3": { "keyX": 12 }
    });
*/
