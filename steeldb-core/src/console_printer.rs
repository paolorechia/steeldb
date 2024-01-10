use crate::DataType;
use crate::Table;
use crate::VERSION;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct ConsolePrinter {
    padding: i32,
}
impl ConsolePrinter {
    pub fn new(padding: i32) -> ConsolePrinter {
        return ConsolePrinter { padding };
    }
    /// Prints the Database banner when the REPL starts.
    pub fn print_banner(&self) {
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

    /// Prints the available help.
    pub fn print_help(&self) {
        println!("Type 'exit;' to leave this shell");
        println!("Current supported commands: [select]");
        println!("");
    }

    /// Prints the table header (types/schema).
    pub fn print_table_fields(&self, column_names: &Vec<String>) -> HashMap<String, i32> {
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

    /// Prints the table data in columnar format.
    pub fn print_table_columns(
        &self,
        table: Box<dyn Table>,
        number_rows: i32,
        column_widths: HashMap<String, i32>,
    ) {
        for i in 0..number_rows {
            print!("|");
            for j in 0..table.get_select_columns().len() as i32 {
                let col_name = table.get_select_columns().get(j as usize).unwrap();
                let col = table.get_columns().get(col_name).unwrap();

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

                if j < table.get_select_columns().len() as i32 - 1 {
                    print!("|");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
    }

    /// Prints a line separator in the format `|------|`.
    pub fn print_separator_line(&self, number_columns: i32, names_length: i32) {
        let size = self.padding * 2 * number_columns + names_length;
        print!("|");
        for _ in 0..size {
            print!("-");
        }
        println!("|");
    }

    /// Prints a table into a pretty format to the standard output.
    pub fn print_table(&self, table: Box<dyn Table>) {
        let number_columns = table.get_select_columns().len() as i32;
        let mut is_empty = false;
        let mut names_length: i32 = 0;
        let mut number_rows: i32 = 0;

        let mut columns_iter = table.get_columns().iter();
        let maybe_column = columns_iter.next();

        // get number of rows from first column
        match maybe_column {
            Some((key, _)) => {
                let col = table.get_columns().get(key);
                if let Some(vec) = col {
                    number_rows = vec.len() as i32;
                }
            }
            None => {
                is_empty = true;
            }
        };

        // iterate over all columns
        for name in table.get_select_columns().iter() {
            names_length += name.len() as i32;
        }

        self.print_separator_line(number_columns, names_length);

        let column_widths = self.print_table_fields(&table.get_select_columns());
        self.print_separator_line(number_columns, names_length);

        if !is_empty {
            self.print_table_columns(table, number_rows, column_widths);
        }
        self.print_separator_line(number_columns, names_length);

        io::stdout().flush().unwrap();
    }
}
