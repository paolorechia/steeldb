use crate::database::datatypes::DataType;
use std::collections::HashMap;

struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
}
