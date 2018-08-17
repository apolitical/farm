pub mod column;
pub mod table;

use std::collections::HashMap;

use self::column::Column;
use self::table::Table;

pub type TablesAndColumns<'a> = HashMap<Table, Vec<&'a Column>>;

pub fn tables_and_columns<'a>(v: &'a Vec<Column>) -> TablesAndColumns<'a> {
    let mut tables_and_columns = HashMap::new();
    v.iter().for_each(|c| {
        let columns_in_table = tables_and_columns.entry(c.get_table()).or_insert(Vec::new());
        columns_in_table.push(c);
    });
    tables_and_columns
}

