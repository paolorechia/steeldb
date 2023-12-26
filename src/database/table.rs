use crate::database::datatypes::DataType;
use std::collections::HashMap;

pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
}
