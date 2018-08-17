#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Table {
    pub table_schema: String,
    pub table_name: String,
}

impl ToString for Table {
    fn to_string(&self) -> String {
        format!("{}.{}", self.table_schema, self.table_name)
    }
}
