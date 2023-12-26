use crate::database::parser::{parse, ParseError};
use crate::database::virtual_machine::VirtualMachine;

pub struct SteelDB {
    virtual_machine: VirtualMachine,
}

impl SteelDB {
    pub fn new() -> SteelDB {
        return SteelDB {
            virtual_machine: VirtualMachine::new(),
        };
    }
    pub fn execute(&mut self, user_input: String) {
        let result = parse(user_input);
        match result {
            Ok(commands) => {
                self.virtual_machine.execute(commands);
            }
            Err(ParseError::Error(error)) => {
                println!("{:?}", error);
            }
        }
    }
}
