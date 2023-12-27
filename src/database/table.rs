pub const DEFAULT_TABLE: &str = "test_table";
use crate::database::datatypes::DataType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
}

pub enum TableResult {
    Success(Table),
    LoadError(String),
    ColumnNotFound(String),
}

impl Table {
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
            };

            let mut returned_columns = HashMap::<String, Vec<DataType>>::new();
            for column_name in select_columns.into_iter() {
                if !test_table.fields.contains_key(&column_name) {
                    return TableResult::ColumnNotFound(column_name);
                }
                let retrieved_column = test_table.columns.get(&column_name);
                match retrieved_column {
                    Some(col) => {
                        returned_columns.insert(column_name, col.to_owned());
                    }
                    None => return TableResult::ColumnNotFound(column_name),
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
            };
            for column in select_columns.into_iter() {
                table
                    .fields
                    .insert(column.clone(), DataType::String(column));
            }
            return TableResult::Success(table);
        }
    }
}
