#[macro_use]
extern crate failure;
#[macro_use]
extern crate mysql;

mod error;

use error::Result;
use mysql::{Pool, Opts};

fn get_db_connection<T>(database_url: T) -> Result<Pool>
    where T: AsRef<str>
{
    let url = format!("{}?prefer_socket=false", database_url.as_ref());
    Ok(mysql::Pool::new(url)?)
}

#[derive(Debug)]
struct Column {
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub character_maximum_length: Option<u64>,
}

pub fn connect_find_replace<T, U, V>(database_url: T, find: U, replace: V) -> Result<()>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str>,
{
    let pool = get_db_connection(database_url)?;

    let possible_columns: Vec<Column> = pool.prep_exec("SELECT TABLE_SCHEMA, TABLE_NAME, COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH FROM information_schema.columns where TABLE_SCHEMA = 'wordpress';", ())
        .map(|result| {
            result
                .filter(|row_result| row_result.is_ok())
                .map(|row_result| row_result.unwrap())
                .map(|row| {
                    let (table_schema, table_name, column_name, data_type, character_maximum_length) = mysql::from_row(row);
                    Column {
                        table_schema,
                        table_name,
                        column_name,
                        data_type,
                        character_maximum_length,
                    }
                })
                .filter(|column| column.character_maximum_length.is_some())
                .collect()
        })?;

    possible_columns.iter().for_each(|c| println!("{:?}",c));

    Ok(())
}
