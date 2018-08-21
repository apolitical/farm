use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;
use model::column::Column;
use model::table::Table;

pub struct TablesAndColumns<'a> {
    tables_and_columns: HashMap<Table, Vec<&'a Column>>,
}

impl<'a> TablesAndColumns<'a> {
    pub fn from_columns(v: &'a Vec<Column>) -> TablesAndColumns<'a> {
        let mut tables_and_columns = HashMap::new();
        v.iter().for_each(|c| {
            let columns_in_table = tables_and_columns
                .entry(c.get_table())
                .or_insert(Vec::new());
            columns_in_table.push(c);
        });
        TablesAndColumns { tables_and_columns }
    }
}

impl<'a> Display for TablesAndColumns<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Work out the size of the first column
        let raw_column_size = self.tables_and_columns.keys().fold(5, |acc, table| { // 5 = "table".len()
            let len = table.table_schema.len() + table.table_name.len() + 1; // "schema.table".len()
            if len > acc {
                len
            } else {
                acc
            }
        });
        let padding = 2;
        let column_size = raw_column_size + padding;

        // Construct the output
        let mut output = String::with_capacity(80 * self.tables_and_columns.keys().len()); // Rough guess at size of string to minimise resizing
        output.push_str(&format!("{:width$}{}", "TABLE", "COLUMNS", width = column_size));
        for (table, columns) in self.tables_and_columns.iter() {
            let table_name = table.to_string();
            let columns_names = columns.iter().map(|column| column.column_name.clone()).collect::<Vec<String>>().join(", ");
            output.push_str(&format!("\n{:width$}{}", table_name, columns_names, width = column_size));
        }

        write!(f, "{}", output)
    }
}
