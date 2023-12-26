use crate::database::steeldb::{ExecutionResult, SteelDB};
use crate::database::table::Table;
use crate::VERSION;
use std::io;
use std::io::Write;

pub struct Repl {
    buffer: String,
    previous_lines: Vec<String>,
    database: SteelDB,
    is_in_multiline: bool,
}

impl Repl {
    pub fn new() -> Repl {
        return Repl {
            buffer: String::new(),
            previous_lines: Vec::<String>::new(),
            database: SteelDB::new(),
            is_in_multiline: false,
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

    fn print_table(&self, _table: Table) {
        println!("Printing table...");
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
                        self.print_table(table);
                    }
                    ExecutionResult::ParseError(error) => {
                        println!("<------------------- PARSE ERROR ------------------->");
                        println!("{:?}", error);
                        println!("");
                        println!("Please check your input");
                        println!("<--------------------------------------------------->");
                    }
                    ExecutionResult::CommandError(error) => {
                        println!("Command failed");
                        println!("{:?}", error);
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
