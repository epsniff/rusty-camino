// extern crate rusty_camino;

// use rusty_camino::{startup, Result};

fn main() {

    let inserter = rusty_camino::sql_proc::InMemInserter::new();
    let mut processor = rusty_camino::sql_proc::SQLProcessor{inserter};

    // startup::up().await.context("Failed to startup the server").unwrap();
    log::info!("rusty-camino CLI tools: help == 'if you actually wrote these tools, I'd be a lot more help!!!'");
    // I will some day, for now I'm using this as a playground...


    /* let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC, b";
    */
    let sql = "INSERT INTO table_foo (RowIdentifier, ContactName, ProductName, CityTownName, PostalCode, CountryName)
    VALUES 
      (1, 'Tom B. Test', 'Foo 21', 'Stavanger', '4006', \"Norway\"),
      (2, 'Jim J. Test', 'Foo 22', 'Stavanger', '5006', \"Norway\");";

      match processor.process_sql(sql) {
        Ok(()) => {},
        Err(e) => log::error!("Error processing command: {}", e),
    }
}
