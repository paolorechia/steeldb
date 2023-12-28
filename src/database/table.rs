use crate::database::config::{DATA_DIR, DEFAULT_TABLE};
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

#[derive(Debug)]
pub enum TableErrors {
    TableNotFound,
    TableAlreadyExists,
    WriteError(String),
    Error(String),
}

#[derive(Debug)]
pub enum TableResult {
    Success(Table),
    LoadError(String),
    ColumnNotFound(String),
}

pub enum SaveMode {
    Overwrite,
    Append,
}

fn format_column_not_found(column_name: &String) -> String {
    return format!("ERROR: Column not found {column_name}");
}

impl Table {
    pub fn init_data_dir() {
        if !Path::new(DATA_DIR).exists() {
            let result = std::fs::create_dir_all(DATA_DIR);
            result.unwrap();
        }
    }

    pub fn save(&self, mode: SaveMode, format: FileFormat) -> Result<(), TableErrors> {
        let s = format!("{}/{}.columnar", DATA_DIR, self.name);
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
                let write_result = writer.write(&self.fields, &self.columns, f.unwrap());
                if write_result.is_err() {
                    let s = format!("{:?}", write_result.unwrap_err());
                    return Err(TableErrors::WriteError(s));
                }
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
                let write_result = writer.append(&self.fields, &self.columns, f.unwrap());
                if write_result.is_err() {
                    let s = format!("{:?}", write_result.unwrap_err());
                    return Err(TableErrors::WriteError(s));
                }
            }
        }
        return Ok(());
    }

    pub fn load(table_name: String, select_columns: Vec<String>) -> TableResult {
        // hardcoded table
        if table_name == DEFAULT_TABLE {
            let table_result = Table::load_test_table(table_name, select_columns);
            let table = table_result.unwrap();
            return TableResult::Success(table);
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

    // Used for testing only
    // Should eventually be deleted or entirely moved into tests directory
    pub fn load_test_table(
        table_name: String,
        select_columns: Vec<String>,
    ) -> Result<Table, TableErrors> {
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
                return Err(TableErrors::Error(format_column_not_found(column_name)));
            }
            let retrieved_column = test_table.columns.get(column_name);
            match retrieved_column {
                Some(col) => {
                    returned_columns.insert(column_name.clone(), col.to_owned());
                }
                None => return Err(TableErrors::Error(format_column_not_found(column_name))),
            }
        }
        test_table.columns = returned_columns;
        return Ok(test_table);
    }
}
