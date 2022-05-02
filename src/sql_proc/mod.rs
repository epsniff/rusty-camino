mod insert;

use sqlparser::ast::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
// use sqlparser::dialect::*;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use crate::sql_proc::insert::*;

// ~~~~~~~~~~  StorageLayer  ~~~~~~~~~~~~
//
//

pub trait StorageLayer {
    // fn insert(&self) -> Result<()>;
}

pub struct CanisterStorageLayer;
impl CanisterStorageLayer {
    pub fn new() -> Arc<Mutex<dyn StorageLayer>> {
        return Arc::new(Mutex::new(Self {}));
    }
}
impl StorageLayer for CanisterStorageLayer {}

// ~~~~~~~~~~  SQLProcessor  ~~~~~~~~~~~~
//
//

pub trait SQLProcessor {
    fn process_sql(&mut self, raw_sql: &str) -> crate::Result<()>;
}

pub struct SQLProcessorImpl {
    pub storage_layer: Arc<Mutex<dyn StorageLayer>>,
}

impl SQLProcessorImpl {
    pub fn new(storage_layer: Arc<Mutex<dyn StorageLayer>>) -> Arc<Mutex<dyn SQLProcessor>> {
        return Arc::new(Mutex::new(Self {
            storage_layer: storage_layer,
        }));
    }
}
impl SQLProcessor for SQLProcessorImpl {
    fn process_sql(&mut self, raw_sql: &str) -> crate::Result<()> {
        let dialect = GenericDialect {}; // or AnsiDialect

        let statements: Vec<Statement>;
        match Parser::parse_sql(&dialect, raw_sql) {
            Ok(_statements) => statements = _statements,
            Err(e) => return Err(crate::Error::new(format!("Unsupported SQL statement: error:{}", e))),
        }

        for (i, statement) in statements.iter().enumerate() {
            // let statement = Parser::parse_sql(&dialect, raw_sql).unwrap().pop().unwrap();
            match statement {
                Statement::Insert {
                    table_name,
                    columns,
                    source,
                    ..
                } => match process_insert(table_name, columns, source) {
                    Err(e) => {
                        return Err(crate::Error::new(format!(
                            "Error processing statement: statement number:{}, error:{}",
                            i, e
                        )))
                    }
                    _ => continue,
                },
                _ => {
                    return Err(crate::Error::new(format!(
                        "Unsupported statement type: statement number:{} statement:{}",
                        i, statement
                    )))
                }
            }
        }

        return Ok(());
    }
}
