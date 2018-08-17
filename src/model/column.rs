use model::table::Table;
use mysql::from_row;
use r2d2::PooledConnection;
use r2d2_mysql::MysqlConnectionManager;

use error::Result;

#[derive(Debug)]
pub struct Column {
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub character_maximum_length: Option<u64>,
}

impl Column {
    pub fn is_string(&self) -> bool {
        self.character_maximum_length.is_some()
    }

    pub fn get_table(&self) -> Table {
        Table {
            table_schema: self.table_schema.clone(),
            table_name: self.table_name.clone(),
        }
    }

    pub fn get_all_in_schema<T>(
        conn: &mut PooledConnection<MysqlConnectionManager>,
        schema: T,
    ) -> Result<Vec<Column>>
    where
        T: AsRef<str>
    {
        let schema = schema.as_ref();
        let columns = conn
            .prep_exec(
                "SELECT TABLE_SCHEMA, TABLE_NAME, COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH FROM information_schema.columns where TABLE_SCHEMA = :schema;",
                params!{ schema })
            .map(|result| {
                 result
                    .filter(|row_result| row_result.is_ok())
                    .map(|row_result| row_result.unwrap())
                    .map(|row| {
                        let (table_schema, table_name, column_name, data_type, character_maximum_length) = from_row(row);
                        Column {
                            table_schema,
                            table_name,
                            column_name,
                            data_type,
                            character_maximum_length,
                        }
                    })
                    .collect()
            })?;
        Ok(columns)
    }

    pub fn contains<T>(
        &self,
        conn: &mut PooledConnection<MysqlConnectionManager>,
        find: T,
    ) -> Result<bool>
    where
        T: AsRef<str>,
    {
        let query = format!(
            "SELECT count(*) FROM {}.{} WHERE {} LIKE :find ",
            self.table_schema, self.table_name, self.column_name,
        );
        let find = format!("%{}%", find.as_ref());
        let result = conn
            .prep_exec(query, params! { find })
            .map(|result| {
                result
                    .filter(|row_result| row_result.is_ok())
                    .map(|row_result| row_result.unwrap())
                    .take(1)
                    .fold(false, |acc, row| {
                        let count: u64 = from_row(row);
                        acc || count > 0
                    })
            })?;
        Ok(result)
    }
}
