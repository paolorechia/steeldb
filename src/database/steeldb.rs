use crate::database::command::CommandResult;
use crate::database::config::DATA_DIR;
use crate::database::parser::{parse, ParseError};
use crate::database::table::Table;
use crate::database::virtual_machine::VirtualMachine;

pub struct SteelDB {
    virtual_machine: VirtualMachine,
}

pub enum ExecutionResult {
    TableResult(Table),
    VoidOK(),
    ParseError(String),
    CommandError(String),
}

impl SteelDB {
    pub fn new() -> SteelDB {
        return SteelDB {
            virtual_machine: VirtualMachine::new(),
        };
    }
    pub fn execute(&mut self, user_input: String) -> ExecutionResult {
        let result = parse(user_input);
        match result {
            Ok(commands) => {
                let command_result = self.virtual_machine.execute(commands);
                // translate CommandResult into ExecutionResult
                // we do not want to make the outer layer import any enum except ExecutionResult
                match command_result {
                    CommandResult::RetrievedDataSuccess(table) => {
                        return ExecutionResult::TableResult(table);
                    }
                    CommandResult::VoidSuccess => return ExecutionResult::VoidOK(),
                    CommandResult::Error(error) => {
                        return ExecutionResult::CommandError(error);
                    }
                }
            }
            // translate ParseError into ExecutionResult
            Err(ParseError::Error(error)) => {
                return ExecutionResult::ParseError(error);
            }
        }
    }
}
