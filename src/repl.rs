use crate::database::steeldb::SteelDB;
use crate::VERSION;
use std::io;
use std::io::Write;

pub struct Repl {
    buffer: String,
    read_lines: Vec<String>,
    database: SteelDB,
    is_in_multiline: bool,
}

impl Repl {
    pub fn new() -> Repl {
        return Repl {
            buffer: String::new(),
            read_lines: Vec::<String>::new(),
            database: SteelDB::new(),
            is_in_multiline: false,
        };
    }

    fn print_banner(&self) {
        println!("------------------------------------------------");
        println!("|                                               |");
        println!("|   SteelDB                                     |");
        println!("{}", format!("|   version: {}                              |", VERSION));
        println!("|                                               |");
        println!("------------------------------------------------");
        println!("");
    }

    fn print_help(&self) {
        println!("Type 'exit;' to leave this shell");
        println!("Current supported commands: [select]");
        println!("");
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
            self.read_lines.push(self.buffer.clone());
            // Command ended
            if self.buffer.contains(";") {
                if self.buffer.contains("exit") {
                    break;
                }
                self.is_in_multiline = false;
                self.database
                    .execute(self.read_lines.join(" ").to_lowercase());
            }
            // Multine line command, keep reading
            else {
                self.is_in_multiline = true;
                self.buffer.clear();
            }
        }
    }
}
