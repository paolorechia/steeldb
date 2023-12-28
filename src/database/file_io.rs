use crate::database::datatypes::DataType;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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
    fn read(&self, fields: &HashMap<String, DataType>, file_: File);
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

        let mut result = file_.write(b"TABLE COLUMNAR FORMAT HEADER\n");
        if result.is_err() {
            return result;
        } else {
            written_bytes += result.unwrap();
        }

        for (key, value) in fields.iter() {
            let column = columns.get(key).unwrap();
            let s = format!(
                "Field name: {:?}; Type: {:?}; Number of elements: {:?}\n",
                key,
                value.name(),
                column.len()
            );
            let b = s.as_bytes();
            result = file_.write(b);

            if result.is_err() {
                return result;
            } else {
                written_bytes += result.unwrap();
            }

            for value in column.iter() {
                match value {
                    DataType::String(str) => {
                        let s = format!("{}\n", str);
                        result = file_.write(s.as_bytes());
                    }
                    DataType::Integer32(str) => {
                        let s = format!("{}\n", str);
                        result = file_.write(s.as_bytes());
                    }
                    DataType::Float32(str) => {
                        let s = format!("{}\n", str);
                        result = file_.write(s.as_bytes());
                    }
                }
                if result.is_err() {
                    return result;
                } else {
                    written_bytes += result.unwrap();
                }
            }
        }

        result = file_.write(b"END OF FILE\n");

        if result.is_err() {
            return result;
        } else {
            written_bytes += result.unwrap();
        }

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
    pub fn new() -> ColumnarReader {
        return ColumnarReader {};
    }
}
impl Reader for ColumnarReader {
    fn read(&self, fields: &HashMap<String, DataType>, file_: File) {}
}
