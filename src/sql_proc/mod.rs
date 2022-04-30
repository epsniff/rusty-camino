mod insert;

use crate::Result;
use sqlparser::ast::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
// use sqlparser::dialect::*;
use std::sync::{Arc, Mutex};

use crate::sql_proc::insert::*;

pub trait Inserter {
    // fn insert(&self) -> Result<()>;
}

pub struct InMemInserter;
impl InMemInserter {
    pub fn new() -> Arc<Mutex<dyn Inserter>> {
        return Arc::new(Mutex::new(Self {}));
    }
}
impl Inserter for InMemInserter {
}

pub struct SQLProcessor {
    pub inserter: Arc<Mutex<dyn Inserter>>,
}

impl SQLProcessor {
    pub fn process_sql(&mut self, raw_sql: &str) -> Result<()> {
        let dialect = GenericDialect {}; // or AnsiDialect
        let statement = Parser::parse_sql(&dialect, raw_sql).unwrap().pop().unwrap();

        match statement {
            Statement::Insert {
                table_name,
                columns,
                source,
                ..
            } => {
                return process_insert(table_name, columns, source);
            }
            _ => return Err(crate::Error::new(format!("Unsupported Statement Type"))),
        }
    }
}


