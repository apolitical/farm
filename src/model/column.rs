use model::table::Table;

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
            table_name: self.table_name.clone()
        }
    }
}
