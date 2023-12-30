#[cfg(test)]
mod tests {
    use crate::database::config::DATA_DIR;
    use crate::database::datatypes::DataType;
    use crate::database::file_io::FileFormat;
    use crate::database::table::{SaveMode, Table};
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
        let load_result = Table::load(
            table_name.to_string(),
            select_columns,
            FileFormat::SimpleColumnar,
        );
        let table = load_result.unwrap();

        let name_column = table.columns.get("name").unwrap();
        let matches: Vec<bool> = vec!["John", "Lenon", "Mary"]
            .iter()
            .zip(name_column.iter())
            .map(|(left, right)| -> bool {
                match right {
                    DataType::String(s) => s == left,
                    _ => panic!("Found not string in name column!"),
                }
            })
            .collect();
        for m in matches.iter() {
            assert!(m);
        }

        let name_column = table.columns.get("annual_salary").unwrap();
        let matches: Vec<bool> = vec![60000, 200000, 30000]
            .iter()
            .zip(name_column.iter())
            .map(|(left, right)| -> bool {
                match right {
                    DataType::Integer32(i) => i == left,
                    _ => panic!("Found not integer in annual salary column!"),
                }
            })
            .collect();
        for m in matches.iter() {
            assert!(m);
        }

        let name_column = table.columns.get("final_grade").unwrap();
        let matches: Vec<bool> = vec![4.0, 3.0, 5.0]
            .iter()
            .zip(name_column.iter())
            .map(|(left, right)| -> bool {
                match right {
                    DataType::Float32(f) => f == left,
                    _ => panic!("Found not float in final grade column!"),
                }
            })
            .collect();
        for m in matches.iter() {
            assert!(m);
        }
    }

    #[test]
    fn test_column_not_found() {
        let table_name = "test_column_not_found";
        write_test_table(&table_name);
        let select_columns = vec!["durp".to_string()];
        let load_result = Table::load(
            table_name.to_string(),
            select_columns,
            FileFormat::SimpleColumnar,
        );
        assert!(load_result.is_err());
    }
}
