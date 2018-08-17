#[macro_use]
extern crate failure;
extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;

pub mod error;
pub mod model;

use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;

use model::column::Column;
use error::Result;

fn get_db_connection<T>(database_url: T) -> Result<Pool<MysqlConnectionManager>>
    where T: AsRef<str>
{
    let mut opts_builder = mysql::OptsBuilder::from_opts(database_url.as_ref());
    opts_builder.prefer_socket(false);
    let manager = MysqlConnectionManager::new(opts_builder);
    let pool = Pool::builder().max_size(15).build(manager)?;
    Ok(pool)
}

pub fn get_affected_columns<T, U>(database_url: T, find: U) -> Result<Vec<Column>>
    where T: AsRef<str>,
          U: AsRef<str>,
{
    let pool = get_db_connection(database_url)?;

    // ToDo: Use mysql::Opts to work out schema name.
    let mut connection = pool.get()?;
    let possible_columns: Vec<Column> = connection
        .prep_exec("SELECT TABLE_SCHEMA, TABLE_NAME, COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH FROM information_schema.columns where TABLE_SCHEMA = 'wordpress';", ())
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
