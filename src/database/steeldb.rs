use crate::database::command::Command;
use crate::database::parser::{parse, ParseError};
use crate::database::table_registry::TableRegistry;

pub struct SteelDB {
    table_registry: TableRegistry,
}

impl SteelDB {
    pub fn new() -> SteelDB {
        return SteelDB {
            table_registry: TableRegistry {},
        };
    }
    pub fn execute(&self, user_input: String) {
        let result = parse(user_input);
        match result {
            Ok(commands) => {
                // invoke virtual machine here
                // Command::SELECT(columns, table_name) => {
                //     // execute select statement here against table
                // }
                println!("Commands succeded!")
            }
            Err(ParseError::Error(error)) => {
                println!("{:?}", error);
            }
        }
    }
}
