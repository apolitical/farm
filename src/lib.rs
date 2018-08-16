#[macro_use]
extern crate failure;
#[macro_use]
extern crate diesel;

mod error;
mod schema;

use diesel::prelude::*;
use error::Result;
use schema::columns;


#[derive(Queryable)]
struct Columns {
    table_schema: String,
    table_name: String,
    column_name: String,
    column_type: String,
}

fn get_db_connection<T>(database_url: T) -> Result<MysqlConnection>
    where T: AsRef<str>
{
    Ok(MysqlConnection::establish(&database_url.as_ref())?)
}

pub fn connect_find_replace<T, U, V>(database_url: T, find: U, replace: V) -> Result<()>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str>,
{
    let db = get_db_connection(database_url)?;

    use columns::dsl::*;

    let mut res = columns
        .filter(table_schema.eq("wordpress"))
        .load::<Columns>(&db)?;
    res.into_iter()
        .for_each(|info| {
            println!("{}.{}", info.table_name, info.column_name);
        });
    Ok(())
}
