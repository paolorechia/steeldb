use crate::database::datatypes::DataType;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

// Enums
pub enum FileFormat {
    SimpleColumnar,
}

// Traits
pub trait Writer {
    fn write(
        &self,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    ) -> Result<usize, std::io::Error>;
    fn append(
        &self,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    ) -> Result<usize, std::io::Error>;
}

pub trait Reader {
    fn read(
        &self,
        file_: File,
        select_columns: Vec<String>,
    ) -> Result<(HashMap<String, DataType>, HashMap<String, Vec<DataType>>), std::io::Error>;
}

// Writer Implementations
pub struct ColumnarWriter {}

impl Writer for ColumnarWriter {
    fn write(
        &self,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        mut file_: File,
    ) -> Result<usize, std::io::Error> {
        let mut written_bytes: usize = 0;

        written_bytes += file_.write(b"TABLE COLUMNAR FORMAT HEADER\n")?;

        for (key, value) in fields.iter() {
            let column = columns.get(key).unwrap();
            let s = format!(
                "Field name: {:?}; Type: {:?}; Number of elements: {:?}\n",
                key,
                value.name(),
                column.len()
            );
            let b = s.as_bytes();
            written_bytes += file_.write(b)?;

            for value in column.iter() {
                match value {
                    DataType::String(str) => {
                        let s = format!("{}\n", str);
                        written_bytes += file_.write(s.as_bytes())?;
                    }
                    DataType::Integer32(str) => {
                        let s = format!("{}\n", str);
                        written_bytes += file_.write(s.as_bytes())?;
                    }
                    DataType::Float32(str) => {
                        let s = format!("{}\n", str);
                        written_bytes += file_.write(s.as_bytes())?;
                    }
                }
            }
        }
        written_bytes += file_.write(b"END OF FILE\n")?;

        return Ok(written_bytes);
    }
    fn append(
        &self,
        fields: &HashMap<String, DataType>,
        columns: &HashMap<String, Vec<DataType>>,
        file_: File,
    ) -> Result<usize, std::io::Error> {
        return Ok(0 as usize);
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
    pub fn new() -> Box<ColumnarReader> {
        return Box::new(ColumnarReader {});
    }
}
impl Reader for ColumnarReader {
    fn read(
        &self,
        mut file_: File,
        select_columns: Vec<String>,
    ) -> Result<(HashMap<String, DataType>, HashMap<String, Vec<DataType>>), std::io::Error> {
        // Prepare return output
        let mut fields = HashMap::<String, DataType>::new();
        let mut columns = HashMap::<String, Vec<DataType>>::new();

        // Read file
        let mut buffer = String::new();
        let mut result = file_.read_to_string(&mut buffer)?;
        // if result
        let lines = buffer.split("\n");

        // TODO: Should update output with file contents here
        return Ok((fields, columns));
    }
}
