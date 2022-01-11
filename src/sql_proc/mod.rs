mod insert;

use crate::Result;
use sqlparser::ast::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
// use sqlparser::dialect::*;

use crate::sql_proc::insert::*;

pub fn process_sql(raw_sql: &str) -> Result<()> {
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
