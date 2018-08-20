pub mod column;
pub mod table;

use std::collections::HashMap;

use self::column::Column;
use self::table::Table;

pub type TablesAndColumns<'a> = HashMap<Table, Vec<&'a Column>>;

pub fn tables_and_columns<'a>(v: &'a Vec<Column>) -> TablesAndColumns<'a> {
    let mut tables_and_columns = HashMap::new();
    v.iter().for_each(|c| {
        let columns_in_table = tables_and_columns
            .entry(c.get_table())
            .or_insert(Vec::new());
        columns_in_table.push(c);
    });
    tables_and_columns
}

pub fn display_tables_and_columns(tables_and_columns: &TablesAndColumns) {
    let raw_column_size = tables_and_columns.keys().fold(5, |acc, table| { // 5 = "table".len()
        let len = table.table_schema.len() + table.table_name.len() + 1;
        if len > acc {
            len
        } else {
            acc
        }
    });
    let column_size = raw_column_size + 2;
    println!("The following columns will be affected\n");
    println!("{:width$}{}", "TABLE", "COLUMNS", width = column_size);
    for (table, columns) in tables_and_columns.iter() {
        let table_name = table.to_string();
        let columns_names = columns.iter().map(|column| column.column_name.clone()).collect::<Vec<String>>().join(", ");
        println!("{:width$}{}", table_name, columns_names, width = column_size);
    }
}
