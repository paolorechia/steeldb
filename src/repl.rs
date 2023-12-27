use crate::database::datatypes::DataType;
use crate::database::steeldb::{ExecutionResult, SteelDB};
use crate::database::table::Table;
use crate::VERSION;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct Repl {
    buffer: String,
    previous_lines: Vec<String>,
    database: SteelDB,
    is_in_multiline: bool,
    padding: i32,
}

impl Repl {
    pub fn new() -> Repl {
        return Repl {
            buffer: String::new(),
            previous_lines: Vec::<String>::new(),
            database: SteelDB::new(),
            is_in_multiline: false,
            padding: 4,
        };
    }

    fn print_banner(&self) {
        println!("------------------------------------------------");
        println!("|                                               |");
        println!("|   SteelDB                                     |");
        println!(
            "{}",
            format!("|   version: {}                              |", VERSION)
        );
        println!("|                                               |");
        println!("------------------------------------------------");
        println!("");
    }

    fn print_help(&self) {
        println!("Type 'exit;' to leave this shell");
        println!("Current supported commands: [select]");
        println!("");
    }

    fn print_table_fields(&self, column_names: &Vec<String>) -> HashMap<String, i32> {
        let mut column_widths = HashMap::<String, i32>::new();
        print!("|");
        for i in 0..column_names.len() as i32 {
            let mut column_width = 0;
            let column = column_names.get(i as usize).unwrap();
            for _ in 0..self.padding {
                print!(" ");
                column_width += 1;
            }
            print!("{}", column);
            column_width += column.len() as i32;
            for _ in 0..self.padding - 1 {
                print!(" ");
                column_width += 1;
            }
            if i < column_names.len() as i32 - 1 {
                print!("|");
            } else {
                print!(" ");
            }
            column_widths.insert(column.clone(), column_width);
        }
        println!("|");
        return column_widths;
    }

    fn print_table_columns(
        &self,
        table: &Table,
        column_names: &Vec<String>,
        number_rows: i32,
        column_widths: HashMap<String, i32>,
        names_length: i32,
    ) {
        for i in 0..number_rows {
            print!("|");
            for j in 0..column_names.len() as i32 {
                let col_name = column_names.get(j as usize).unwrap();
                let col = table.columns.get(col_name).unwrap();

                let maybe_value = col.get(i as usize);
                let size_of_value: i32;
                let mut val = "".to_string();

                match maybe_value {
                    // Have value for this row
                    Some(value) => match value {
                        DataType::String(string_value) => {
                            size_of_value = string_value.len() as i32;
                            val = string_value.clone();
                        }
                        DataType::Float32(float_value) => {
                            val = float_value.to_string();
                            size_of_value = val.len() as i32;
                        }
                        DataType::Integer32(integer_value) => {
                            val = integer_value.to_string();
                            size_of_value = val.len() as i32;
                        }
                    },
                    // Missing value case
                    None => size_of_value = 0,
                }

                let column_width = column_widths.get(col_name).unwrap();
                let padding_size = column_width - size_of_value;
                let mut half_padding = padding_size / 2;

                for _ in 0..half_padding {
                    print!(" ");
                }

                // If column width is odd
                // We have one extra space since division by integer rounds down
                if padding_size % 2 != 0 {
                    half_padding += 1;
                }

                print!("{}", val);
                for _ in 0..half_padding {
                    print!(" ");
                }

                if j < column_names.len() as i32 - 1 {
                    print!("|");
                } else {
                    print!(" ");
                }
            }
            println!("|");
            self.print_separator_line(column_names.len() as i32, names_length)
        }
    }

    fn print_separator_line(&self, number_columns: i32, names_length: i32) {
        let size = self.padding * 2 * number_columns + names_length;
        print!("|");
        for _ in 0..size {
            print!("-");
        }
        println!("|");
    }

    fn print_table(&self, table: &Table) {
        println!("DEBUG PRINT {:?}", table);
        println!("");

        let number_columns = table.columns.len() as i32;
        let mut is_empty = false;
        let mut names_length: i32 = 0;
        let mut number_rows: i32 = 0;

        let mut column_names: Vec<String> = vec![];

        let mut columns_iter = table.columns.iter();
        let maybe_column = columns_iter.next();

        // get number of rows from first column
        match maybe_column {
            Some((key, _)) => {
                let col = table.columns.get(key);
                if let Some(vec) = col {
                    number_rows = vec.len() as i32;
                }
            }
            None => {
                is_empty = true;
            }
        };

        // iterate over all columns
        for (key, _) in table.columns.iter() {
            names_length += key.len() as i32;
            column_names.push(key.clone());
        }

        self.print_separator_line(number_columns, names_length);

        let column_widths = self.print_table_fields(&column_names);
        self.print_separator_line(number_columns, names_length);

        if !is_empty {
            self.print_table_columns(
                &table,
                &column_names,
                number_rows,
                column_widths,
                names_length,
            );
        }
        io::stdout().flush().unwrap();
    }

    pub fn main_loop(&mut self) {
        self.print_banner();
        self.print_help();
        loop {
            if self.is_in_multiline {
                print!("| ");
            } else {
                print!(">> ");
            }
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut self.buffer).unwrap();
            self.previous_lines.push(self.buffer.clone());
            // Command ended
            if self.buffer.contains(";") {
                if self.buffer.contains("exit") {
                    break;
                }
                self.is_in_multiline = false;
                let execution_result = self
                    .database
                    .execute(self.previous_lines.join(" ").to_lowercase());

                match execution_result {
                    ExecutionResult::VoidOK() => {
                        println!("OK!");
                    }
                    ExecutionResult::TableResult(table) => {
                        self.print_table(&table);
                    }
                    ExecutionResult::ParseError(error) => {
                        println!("");
                        println!("");
                        println!("<------------------- PARSE ERROR ------------------->");
                        println!("{:?}", error);
                        println!("");
                        println!("Please check your input");
                        println!("<--------------------------------------------------->");
                        println!("");
                    }
                    ExecutionResult::CommandError(error) => {
                        println!("");
                        println!("");
                        println!("<------------------ COMMAND FAILED ------------------>");
                        println!("{:?}", error);
                        println!("");
                        println!("<---------------------------------------------------->");
                        println!("");
                    }
                }
                self.buffer.clear();
                self.previous_lines.clear();
            }
            // Multine line command, keep reading
            else {
                self.is_in_multiline = true;
                self.buffer.clear();
            }
        }
    }
}
