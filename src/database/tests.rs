#[cfg(test)]
mod tests {
    use crate::database::config::DEFAULT_TABLE;
    use crate::database::file_io::FileFormat;
    use crate::database::table::{SaveMode, Table};

    #[test]
    fn test_write_columnar_table() {
        Table::init_data_dir();
        let select_columns = vec![
            "name".to_string(),
            "annual_salary".to_string(),
            "final_grade".to_string(),
        ];
        let table_result = Table::load_test_table(DEFAULT_TABLE.to_string(), select_columns);
        let table = table_result.unwrap();
        let save_result = table.save(SaveMode::Overwrite, FileFormat::SimpleColumnar);
        save_result.unwrap();
    }
}
