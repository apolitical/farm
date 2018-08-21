#[macro_use]
extern crate clap;
extern crate cuckoo;

use std::io::{self, BufRead};
use clap::App;
use cuckoo::{
    get_affected_columns,
    replace_in_columns,
    model::tables_and_columns::TablesAndColumns,
};

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = matches.value_of("database").unwrap();
    let find = matches.value_of("find").unwrap();
    let replace = matches.value_of("replace").unwrap();

    match get_affected_columns(&database, &find) {
        Ok(c) => {
            {
                let tc = TablesAndColumns::from_columns(&c);
                println!("The following columns will be affected");
                println!("{}", tc);
            }

            println!("");
            println!("You are about to replace \"{}\" with \"{}\"", find, replace);
            println!("Are you sure y/N");
            let mut confirm = String::with_capacity(8);
            let stdin = io::stdin();
            let _ = stdin.lock().read_line(&mut confirm);
            if confirm.to_lowercase().trim() == "y" {
                // ToDo: Do something with this
                let _ = replace_in_columns(&database, c, find, replace);
            } else {
                eprintln!("Exiting due to user inout");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Not OK: {:#?}", e);
            std::process::exit(1);
        }
    }
}
