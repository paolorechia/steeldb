use crate::database::datatypes::DataType;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

const COLUMNAR_HEADER: [u8; 29] = *b"TABLE COLUMNAR FORMAT HEADER\n";

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

#[derive(Debug)]
pub enum ReadError {
    InvalidFileSize,
    InvalidFieldMeta(String),
    FieldParseError(String),
    StdIoError(std::io::Error),
}

pub trait Reader {
    fn read(
        &self,
        file_: File,
        select_columns: Vec<String>,
    ) -> Result<(HashMap<String, DataType>, HashMap<String, Vec<DataType>>), ReadError>;
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
        if fields.len() == 0 {
            panic!("Cannot write empty table without schema - TODO: Handle this case, it should propagate an error and not panic");
        }

        let mut written_bytes: usize = 0;

        written_bytes += file_.write(&COLUMNAR_HEADER)?;

        for (key, value) in fields.iter() {
            let column = columns.get(key).unwrap();
            let s = format!(
                "Field name: {}; Type: {}; Number of elements: {}\n",
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

    fn read_metadata(line: &str, line_number: i32) -> Result<(String, String, i32), ReadError> {
        // "Field name: {:?}; Type: {:?}; Number of elements: {:?}\n",
        let field_meta: Vec<&str> = line.split(";").collect();
        // Basic check
        if field_meta.len() != 3 {
            let s = format!(
                "Error at line: {}. Expected 3 meta fields, found {} instead",
                line_number,
                field_meta.len()
            );
            return Err(ReadError::InvalidFieldMeta(s));
        }

        // collect number of elements;
        let number_split: Vec<&str> = field_meta.get(2).unwrap().split(":").collect();

        if number_split.len() != 2 {
            return Err(ReadError::InvalidFieldMeta(format!(
                "Error at line: {}. Could not split meta 'number of elements'",
                line_number,
            )));
        }
        let maybe_number = number_split.get(1).unwrap().replace(" ", "").parse::<i32>();

        if maybe_number.is_err() {
            return Err(ReadError::FieldParseError(format!(
                "Error at line: {}. Could not read meta 'number of elements'. Error: {}",
                line_number,
                maybe_number.unwrap_err()
            )));
        }

        let field_number_of_elements = maybe_number.unwrap();

        // collect field type
        let type_split: Vec<&str> = field_meta.get(1).unwrap().split(":").collect();

        if type_split.len() != 2 {
            return Err(ReadError::InvalidFieldMeta(format!(
                "Error at line: {}. Could not split meta 'type'",
                line_number,
            )));
        }

        let field_type = type_split.get(1).unwrap().replace(" ", "");

        // collect field name
        let name_split: Vec<&str> = field_meta.get(0).unwrap().split(":").collect();
        if name_split.len() != 2 {
            return Err(ReadError::InvalidFieldMeta(
                "Could not split meta 'name'".to_string(),
            ));
        }
        let field_name = name_split.get(1).unwrap().replace(" ", "");

        return Ok((field_name, field_type, field_number_of_elements));
    }
}

impl Reader for ColumnarReader {
    fn read(
        &self,
        mut file_: File,
        select_columns: Vec<String>,
    ) -> Result<(HashMap<String, DataType>, HashMap<String, Vec<DataType>>), ReadError> {
        // Prepare return output
        let mut fields = HashMap::<String, DataType>::new();
        let mut columns = HashMap::<String, Vec<DataType>>::new();

        // Read file
        let mut buffer = String::new();
        let result = file_.read_to_string(&mut buffer);
        if result.is_err() {
            return Err(ReadError::StdIoError(result.unwrap_err()));
        }
        // if result
        let lines: Vec<&str> = buffer.split("\n").collect();
        if lines.len() < 2 {
            return Err(ReadError::InvalidFileSize);
        }

        let field_header_line = lines.get(1).unwrap();

        let result = ColumnarReader::read_metadata(field_header_line, 1);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        let (mut field_name, mut field_type, mut field_number_of_elements) = result.unwrap();

        // Start collecting at third line (zero-indexed)
        let mut line = 2;

        // read loop here
        while line < lines.len() as i32 {
            let block_end = field_number_of_elements + line;

            if (lines.len() as i32) < block_end {
                return Err(ReadError::InvalidFileSize);
            }

            // collect data only if requested
            if select_columns.contains(&field_name) {
                let dtype: DataType;
                if field_type == "i32" {
                    dtype = DataType::Integer32(0);
                } else if field_type == "f32" {
                    dtype = DataType::Float32(0.0);
                } else {
                    dtype = DataType::String(field_name.to_string());
                }

                fields.insert(field_name.to_string(), dtype);
                columns.insert(field_name.to_string(), vec![]);
                let column = columns.get_mut(&field_name).unwrap();
                for i in line..block_end {
                    let line = lines.get(i as usize).unwrap();
                    let val: DataType;
                    if field_type == "i32" {
                        let result = line.parse::<i32>();
                        if result.is_err() {
                            return Err(ReadError::FieldParseError(format!(
                                "Failed to read integer at line {}",
                                i
                            )));
                        }
                        val = DataType::Integer32(result.unwrap());
                    } else if field_type == "f32" {
                        let result = line.parse::<f32>();
                        if result.is_err() {
                            return Err(ReadError::FieldParseError(format!(
                                "Failed to read integer at line {}",
                                i
                            )));
                        }
                        val = DataType::Float32(result.unwrap());
                    } else {
                        val = DataType::String(line.to_string());
                    }
                    column.push(val);
                }
            }

            line = block_end;
            if line >= lines.len() as i32 {
                // reached EOF
                break;
            }
            let unwrapped_line = lines.get(line as usize).unwrap();
            if unwrapped_line.len() == 0 {
                break;
            }

            // Read next field metadata
            let result = ColumnarReader::read_metadata(&unwrapped_line, line);
            if result.is_err() {
                return Err(result.unwrap_err());
            }
            (field_name, field_type, field_number_of_elements) = result.unwrap();
            // Prepare to read data
            line += 1;
        }

        return Ok((fields, columns));
    }
}
