use crate::database::datatypes::DataType;
use std::collections::HashMap;
use std::fs::File;

// Enums
pub enum FileFormat {
    SimpleColumnar,
}

// Traits
pub trait Writer {
    fn write(
        &self,
        name: &String,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    );
    fn append(
        &self,
        name: &String,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    );
}

pub trait Reader {
    fn read(&self, name: &String, fields: &HashMap<String, DataType>, file_: File);
}

// Writer Implementations
pub struct ColumnarWriter {}

impl Writer for ColumnarWriter {
    fn write(
        &self,
        name: &String,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    ) {
    }
    fn append(
        &self,
        name: &String,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    ) {
    }
}

impl ColumnarWriter {
    pub fn new() -> Box<ColumnarWriter> {
        return Box::new(ColumnarWriter {});
    }
}

// Reader Implementations
pub struct ColumnarReader {}
impl ColumnarReader {
    pub fn new() -> ColumnarReader {
        return ColumnarReader {};
    }
}
impl Reader for ColumnarReader {
    fn read(&self, name: &String, fields: &HashMap<String, DataType>, file_: File) {}
}
