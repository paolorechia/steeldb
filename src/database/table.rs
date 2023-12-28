pub const DEFAULT_TABLE: &str = "test_table";
pub const DATA_DIR: &str = "data";
use crate::database::datatypes::DataType;
use crate::database::file_io::{ColumnarWriter, FileFormat, Writer};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

pub enum TableResult {
    Success(Table),
    LoadError(String),
    ColumnNotFound(String),
}

pub enum SaveMode {
    Overwrite,
    Append,
}

pub enum TableErrors {
    TableNotFound,
    TableAlreadyExists,
}

fn format_column_not_found(column_name: &String) -> String {
    return format!("ERROR: Column not found {column_name}");
}

impl Table {
    pub fn save(table: Table, mode: SaveMode, format: FileFormat) -> Result<(), TableErrors> {
        let s = format!("{}/{}.columnar_table", DATA_DIR, table.name);
        let path = Path::new(&s);

        // Pick up correct writer
        let writer: Box<dyn Writer>;
        match format {
            FileFormat::SimpleColumnar => {
                writer = ColumnarWriter::new();
            }
        }
        // Adapt to the given mode
        match mode {
            SaveMode::Overwrite => {
                let f = OpenOptions::new().write(true).create_new(true).open(path);
                if f.is_err() {
                    println!("{:?}", f.unwrap_err());
                    return Err(TableErrors::TableAlreadyExists);
                }
                writer.write(&table.name, &table.fields, &table.columns, f.unwrap());
            }

            SaveMode::Append => {
                let f = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(false)
                    .open(path);
                if f.is_err() {
                    println!("{:?}", f.unwrap_err());
                    return Err(TableErrors::TableNotFound);
                }
                writer.append(&table.name, &table.fields, &table.columns, f.unwrap());
            }
        }
        return Ok(());
    }

    pub fn load(table_name: String, select_columns: Vec<String>) -> TableResult {
        // hardcoded table
        if table_name == "test_table" {
            let mut fields = HashMap::<String, DataType>::new();
            fields.insert("name".to_string(), DataType::String("name".to_string()));
            fields.insert("annual_salary".to_string(), DataType::Integer32(0));
            fields.insert("final_grade".to_string(), DataType::Float32(0.0));

            let mut columns = HashMap::<String, Vec<DataType>>::new();

            let mut name_column = Vec::<DataType>::new();
            let mut annual_salary_column = Vec::<DataType>::new();
            let mut final_grade_column = Vec::<DataType>::new();

            name_column.push(DataType::String("John".to_string()));
            name_column.push(DataType::String("Lenon".to_string()));
            name_column.push(DataType::String("Mary".to_string()));

            annual_salary_column.push(DataType::Integer32(60000));
            annual_salary_column.push(DataType::Integer32(200000));
            annual_salary_column.push(DataType::Integer32(30000));

            final_grade_column.push(DataType::Float32(4.0));
            final_grade_column.push(DataType::Float32(3.0));
            final_grade_column.push(DataType::Float32(5.0));

            columns.insert("name".to_string(), name_column);
            columns.insert("annual_salary".to_string(), annual_salary_column);
            columns.insert("final_grade".to_string(), final_grade_column);

            let mut test_table = Table {
                name: table_name,
                fields: fields,
                columns: columns,
                select_columns: select_columns,
            };

            let mut returned_columns = HashMap::<String, Vec<DataType>>::new();
            for column_name in test_table.select_columns.iter() {
                if !test_table.fields.contains_key(column_name) {
                    return TableResult::ColumnNotFound(format_column_not_found(column_name));
                }
                let retrieved_column = test_table.columns.get(column_name);
                match retrieved_column {
                    Some(col) => {
                        returned_columns.insert(column_name.clone(), col.to_owned());
                    }
                    None => {
                        return TableResult::ColumnNotFound(format_column_not_found(column_name))
                    }
                }
            }
            test_table.columns = returned_columns;
            return TableResult::Success(test_table);
        } else {
            // for now return an empty table
            let mut table = Table {
                name: table_name,
                fields: HashMap::<String, DataType>::new(),
                columns: HashMap::<String, Vec<DataType>>::new(),
                select_columns: select_columns,
            };
            for column in table.select_columns.iter() {
                table
                    .fields
                    .insert(column.clone(), DataType::String(column.clone()));
            }
            return TableResult::Success(table);
        }
    }
}
