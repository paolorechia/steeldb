pub const DEFAULT_TABLE: &str = "test_table";
use crate::database::datatypes::DataType;
use std::collections::HashMap;

pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
}

impl Table {
    pub fn load(table_name: String, select_columns: Vec<String>) -> Table {
        // for now return an empty table
        let table = Table {
            name: table_name,
            fields: HashMap::<String, DataType>::new(),
        };
        return table;
    }
}
