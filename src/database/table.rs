pub const DEFAULT_TABLE: &str = "test_table";
use crate::database::datatypes::DataType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
}

pub enum TableResult {
    Success(Table),
    LoadError(String),
    ColumnNotFound(String),
}

impl Table {
    pub fn load(table_name: String, select_columns: Vec<String>) -> TableResult {
        // for now return an empty table
        let mut table = Table {
            name: table_name,
            fields: HashMap::<String, DataType>::new(),
        };
        for column in select_columns.into_iter() {
            table
                .fields
                .insert(column.clone(), DataType::String(column));
        }
        return TableResult::Success(table);
    }
}
