#[macro_use]
extern crate clap;
extern crate mysql_find_and_replace;

use clap::{App};
use mysql_find_and_replace::connect_find_replace;

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = matches.value_of("database").unwrap();
    let find = matches.value_of("find").unwrap();
    let replace = matches.value_of("replace").unwrap();

    match connect_find_replace(&database, &find, &replace) {
        Ok(_) => println!("OK"),
        Err(e) => eprintln!("Not OK: {:#?}", e),
    }
}
