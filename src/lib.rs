#[macro_use]
extern crate failure;
#[macro_use]
extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate rayon;

pub mod error;
pub mod model;

use mysql::{OptsBuilder, Opts};
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use rayon::prelude::*;

use error::{Result, error::Error};
use model::column::Column;

fn get_db_connection<T>(database_url: T) -> Result<Pool<MysqlConnectionManager>>
where
    T: AsRef<str>,
{
    let mut opts_builder = OptsBuilder::from_opts(database_url.as_ref());
    opts_builder.prefer_socket(false);
    let manager = MysqlConnectionManager::new(opts_builder);
    let pool = Pool::builder().max_size(15).build(manager)?;
    Ok(pool)
}

pub fn get_affected_columns<T, U>(database_url: T, find: U) -> Result<Vec<Column>>
where
    T: AsRef<str>,
    U: AsRef<str> + Send + Sync,
{
    let opts: Opts = OptsBuilder::from_opts(database_url.as_ref()).into();
    let schema = opts.get_db_name().ok_or(Error::SchemaError(database_url.as_ref().to_string()))?;
    let pool = get_db_connection(database_url)?;

    let mut connection = pool.get()?;
    let all_columns = Column::get_all_in_schema(&mut connection, schema)?;

    let prepared_columns: Vec<(Column, Pool<MysqlConnectionManager>)> = all_columns.into_iter()
        .filter(|column| column.is_string())
        .map(|column| (column, pool.clone()))
        .collect();
    let possible_columns = prepared_columns.into_par_iter()
        .filter(|(column, pool)| {
            let mut conn = pool.get().expect("Connection Pool broke");
            column.contains(&mut conn, &find).expect("Something has gone wrong")
        })
        .map(|(column, _conn)| column)
        .collect();

    Ok(possible_columns)
}

pub fn replace_in_columns<T, U,V>(database_url: T, columns: Vec<Column>, find: U, replace: V) -> Result<bool>
where
    T: AsRef<str>,
    U: AsRef<str> + Send + Sync,
    V: AsRef<str> + Send + Sync,
{
    let pool = get_db_connection(database_url)?;

    let prepared_columns: Vec<(Column, Pool<MysqlConnectionManager>)> = columns
        .into_iter().map(|column| (column, pool.clone())).collect();

    prepared_columns.into_par_iter().for_each(|(column, pool)| {
        let mut conn = pool.get().expect("Connection Pool broke");
        column.replace(&mut conn, &find, &replace);
    });

    // ToDo: Work out what to return here? A Vec of columns with success/failure?
    Ok(false)
}
