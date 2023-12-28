#[cfg(test)]
mod tests {
    use crate::database::config::DATA_DIR;
    use crate::database::file_io::FileFormat;
    use crate::database::table::{SaveMode, Table, TableResult};
    use std::path::Path;

    fn write_test_table(table_name: &str) {
        Table::init_data_dir();
        let mut filename = table_name.to_string();
        filename.push_str(".columnar");

        let file_path = Path::new(DATA_DIR).join(Path::new(&filename));
        print!("Checking if file exists at {:?}... ", file_path);
        if file_path.exists() {
            print!("exists, deleting...");
            let result = std::fs::remove_file(file_path);
            result.unwrap();
            println!("OK");
        } else {
            println!("does not exist");
        }

        let select_columns = vec![
            "name".to_string(),
            "annual_salary".to_string(),
            "final_grade".to_string(),
        ];
        let table_result = Table::load_test_table(table_name.to_string(), select_columns);
        let table = table_result.unwrap();
        let save_result = table.save(SaveMode::Overwrite, FileFormat::SimpleColumnar);
        save_result.unwrap();
    }

    #[test]
    fn test_write_columnar_table() {
        write_test_table("test_write_table");
    }

    #[test]
    fn test_read_columnar_table() {
        let table_name = "test_read_table";
        write_test_table(&table_name);
        let select_columns = vec![
            "name".to_string(),
            "annual_salary".to_string(),
            "final_grade".to_string(),
        ];
        let load_result = Table::load(table_name.to_string(), select_columns);
        if let TableResult::Success(table) = load_result {
            // TODO: check table here
        } else {
            println!("{:?}", load_result);
            panic!("Error!");
        }
    }
}
