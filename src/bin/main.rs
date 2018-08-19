#[macro_use]
extern crate clap;
extern crate mysql_find_and_replace;

use clap::App;
use mysql_find_and_replace::{get_affected_columns, replace_in_columns, model::tables_and_columns};

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = matches.value_of("database").unwrap();
    let find = matches.value_of("find").unwrap();
    let replace = matches.value_of("replace").unwrap();

    match get_affected_columns(&database, &find) {
        Ok(c) => {
            {
                let tc = tables_and_columns(&c);
                let raw_column_size = tc.keys().fold(5, |acc, table| { // 5 = "table".len()
                    let len = table.table_schema.len() + table.table_name.len() + 1;
                    if len > acc {
                        len
                    } else {
                        acc
                    }
                });
                let column_size = raw_column_size + 2;
                println!("The following columns will be affected\n");
                println!("{:width$}{}", "TABLE", "COLUMNS", width = column_size);
                for (table, columns) in tc.iter() {
                    let table_name = table.to_string();
                    let columns_names = columns.iter().map(|column| column.column_name.clone()).collect::<Vec<String>>().join(", ");
                    println!("{:width$}{}", table_name, columns_names, width = column_size);
                }
            }

            replace_in_columns(&database, c, find, replace);
        }
        Err(e) => {
            eprintln!("Not OK: {:#?}", e);
            std::process::exit(1);
        },
    }
}
