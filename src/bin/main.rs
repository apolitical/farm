#[macro_use]
extern crate clap;
extern crate mysql_find_and_replace;

use clap::App;
use mysql_find_and_replace::get_affected_columns;
use mysql_find_and_replace::model::tables_and_columns;

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = matches.value_of("database").unwrap();
    let find = matches.value_of("find").unwrap();
    let _replace = matches.value_of("replace").unwrap();

    match get_affected_columns(&database, &find) {
        Ok(c) => {
            let tc = tables_and_columns(&c);
            let tables: Vec<String> = tc.keys().map(|table| table.to_string()).collect();
            let raw_column_size = tables.iter().fold(1, |acc, table| {
                let len = table.len();
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
        Err(e) => {
            eprintln!("Not OK: {:#?}", e);
            std::process::exit(1);
        },
    }
}
