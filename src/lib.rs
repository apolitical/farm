#[macro_use]
extern crate failure;
#[macro_use]
extern crate mysql;

pub mod error;
pub mod model;

use model::column::Column;
use error::Result;
use mysql::Pool;

fn get_db_connection<T>(database_url: T) -> Result<Pool>
    where T: AsRef<str>
{
    // ToDo: Test for `?` in string and use `&` instead or find a better way to set the option
    let url = format!("{}?prefer_socket=false", database_url.as_ref());
    Ok(mysql::Pool::new(url)?)
}

pub fn get_affected_columns<T, U>(database_url: T, find: U) -> Result<Vec<Column>>
    where T: AsRef<str>,
          U: AsRef<str>,
{
    let pool = get_db_connection(database_url)?;

    // ToDo: Use mysql::Opts to work out schema name.
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
                .filter(|column| column.is_string())
                .collect()
        })?;

    Ok(possible_columns)
}
