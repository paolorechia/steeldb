use crate::{Table, DataType};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TableJSON {
    pub table_name: String,
    pub columns: HashMap<String, Vec::<DataType>>,
    pub select_columns: Vec::<String>
}

impl TableJSON {
    pub fn from_table(table: Box::<dyn Table>) -> TableJSON {
        TableJSON {
            table_name: table.get_table_name(),
            columns: table.move_columns(),
            select_columns: table.move_select_columns(),
        }
    }
}