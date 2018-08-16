#[macro_use]
extern crate clap;
extern crate mysql_find_and_replace;

use clap::{App};
use mysql_find_and_replace::test;

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = matches.values_of("DATABASE").unwrap();
    let find = matches.values_of("FIND").unwrap();
    let replace = matches.values_of("REPLACE").unwrap();
}
