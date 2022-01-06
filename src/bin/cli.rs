// extern crate rusty_camino;

// use rusty_camino::{startup, ResultExt};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

#[tokio::main]
async fn main() {
    // startup::up().await.context("Failed to startup the server").unwrap();
    log::info!("rusty-camino CLI tools: help == 'if you actually wrote these tools, I'd be a lot more help!!!'");

    let dialect = GenericDialect {}; // or AnsiDialect

    let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";
    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:?}", ast);
}
