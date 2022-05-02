use sqlparser::ast::*;

// sqlparser::ast::ObjectName
// sqlparser::ast::Ident
// sqlparser::ast::Query
pub fn process_insert(table_name: &ObjectName, columns: &Vec<Ident>, source: &Box<Query>) -> crate::Result<()> {
    // println!("table  - {:?}", table_name);
    // for (idx, column) in columns.iter().enumerate() {
    //    println!(" - {} - {:?}", idx, column);
    // }

    match &source.body {
        SetExpr::Values(Values(values)) => {
            for (row_id, row) in values.iter().enumerate() {
                // print_type_of(row);
                let _ = process_insert_row(&table_name, &columns, row_id, &row);
            }
        }
        _ => {
            return Err(crate::Error::new(format!(
                "Unsupported Insert Set Type (example of supported types include Values."
            )))
        }
    }
    //for (_, column) in source.body.values.iter().enumerate() {
    //    println!(" - {:?}", column);
    //}
    return Ok(());
}

fn process_insert_row(
    table_name: &ObjectName,
    columns: &Vec<Ident>,
    row_id: usize,
    row: &Vec<Expr>,
) -> crate::Result<()> {
    println!(" {:?}", row);
    for (i, val) in row.iter().enumerate() {
        let name = &columns[i].value;
        match val {
            Expr::Value(Value::SingleQuotedString(ref s)) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t SingleString:{:?}",
                    table_name, row_id, name, s
                );
            }
            Expr::Value(Value::NationalStringLiteral(ref s)) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t NationalString:{:?}",
                    table_name, row_id, name, s
                );
            }
            Expr::Value(Value::HexStringLiteral(s)) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t HexString:{:?}",
                    table_name, row_id, name, s
                );
            }
            Expr::Value(Value::DoubleQuotedString(s)) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t DoubleString:{:?}",
                    table_name, row_id, name, s
                );
            }
            Expr::Value(Value::Number(n, _)) => {
                match n.parse::<i64>() {
                    Ok(n) => {
                        println!(
                            "   - table: {} - row_id: {} - col: {} -> \t Number(i64):{:?}",
                            table_name, row_id, name, n
                        );
                        continue;
                    }
                    Err(_) => {}
                }
                match n.parse::<f64>() {
                    Ok(f) => {
                        println!(
                            "   - table: {} - row_id: {} - col: {} -> \t Number(f64):{:?}",
                            table_name, row_id, name, f
                        );
                        continue;
                    }
                    Err(_) => {
                        return Err(crate::Error::new(format!(
                            "Unsupported Number Type (only supported i64/f64"
                        )))
                    }
                }
            }
            Expr::Value(Value::Boolean(b)) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t Bool:{:?}",
                    table_name, row_id, name, *b
                );
            }
            Expr::Value(Value::Null) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t Null",
                    table_name, row_id, name
                );
            }
            Expr::Identifier(ref id) => {
                println!(
                    "   - table: {} - row_id: {} - col: {} -> \t Identifier:{:?}",
                    table_name, row_id, name, id
                );
            }
            Expr::Value(Value::Interval { .. }) => {
                return Err(crate::Error::new(format!("Unsupported Value.Interval Type")))
            }
            Expr::Case { .. } => return Err(crate::Error::new(format!("Unsupported Case Type"))),
            _ => return Err(crate::Error::new(format!("Unsupported/Unknown Type"))),
        }
    }
    Ok(())
}
