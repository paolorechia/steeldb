//! The entrypoint module of the database. Defines an embbeddeable Database.
//! In the future, it might also define the database as Server/Client.
use crate::database::command::CommandResult;
use crate::database::parser::{parse, ParseError};
use crate::database::table::Table;
use crate::database::virtual_machine::VirtualMachine;

/// The main struct exposed by the crate.
/// See the crate root documentation on how to use it.
pub struct SteelDB {
    /// The VirtualMachine that executes parsed commands.
    /// It should not be used directly by an end user.
    virtual_machine: VirtualMachine,
}

/// The return type given by the [SteelDB::execute] function.
pub enum ExecutionResult {
    /// A result where a table was successfully computed/retrieved, and is available for inspection.
    TableResult(Table),
    /// A result where a command was successfully executed, but with no output.
    VoidOK,
    /// Parse error. The given input string was not valid for the parser.
    ParseError(String),
    /// Command error. Something went wrong when executing the command.
    /// Examples include `ColumnNotFound`, `TableNotFound` etc.
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
                    CommandResult::VoidSuccess => return ExecutionResult::VoidOK,
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
