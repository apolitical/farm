#[macro_use]
extern crate clap;
extern crate mysql_find_and_replace;

use clap::{App};
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
            for (table, columns) in tc.iter() {
                println!("{}.{}:", table.table_schema, table.table_name);
                print!("   ");
                columns.iter().for_each(|c| print!(" {},", c.column_name));
                println!();
                println!();
            }
        },
        Err(e) => eprintln!("Not OK: {:#?}", e),
    }
}
