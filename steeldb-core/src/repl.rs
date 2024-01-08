//! This crate defines a useful REPL to issue query commands interactively with SteelDB.
//! It's a private module and not meant to be imported directly.

use crate::database::console_printer::ConsolePrinter;
use crate::database::steeldb::{ExecutionResult, SteelDB};
use std::io;
use std::io::Write;

/// The main struct that is publicly exposed by this module Repl.
/// See example in the root crate on how to use it.
pub struct Repl {
    buffer: String,
    previous_lines: Vec<String>,
    database: SteelDB,
    is_in_multiline: bool,
    console: ConsolePrinter,
}

impl Repl {
    /// The REPL constructor. Currently not customizable, but could be extended
    /// to read options / configuration.
    pub fn new() -> Repl {
        return Repl {
            buffer: String::new(),
            previous_lines: Vec::<String>::new(),
            database: SteelDB::new(),
            is_in_multiline: false,
            console: ConsolePrinter::new(4),
        };
    }

    /// The main loop (literally, the REPL).
    pub fn main_loop(&mut self) {
        self.console.print_banner();
        self.console.print_help();
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
                    ExecutionResult::VoidOK => {
                        println!("OK!");
                    }
                    ExecutionResult::TableResult(table) => {
                        self.console.print_table(&table);
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
