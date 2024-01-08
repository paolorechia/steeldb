//! The entrypoint module of the database. Defines an embbeddeable Database.
//! In the future, it might also define the database as Server/Client.
use crate::database::command::CommandResult;
use crate::database::logger::logger_init;
use crate::database::parser::{parse, ParseError};
use crate::database::virtual_machine::VirtualMachine;
use log::{error, info};
use steeldb_core::ExecutionResult;

/// The main struct exposed by the crate.
/// See the crate root documentation on how to use it.
pub struct SteelDB {
    /// The VirtualMachine that executes parsed commands.
    /// It should not be used directly by an end user.
    virtual_machine: VirtualMachine,
}

impl SteelDB {
    /// SteelDB constructor, should be called to initialize logger and virtual machine.
    pub fn new() -> SteelDB {
        logger_init();
        info!("SteelDB log initialized");
        return SteelDB {
            virtual_machine: VirtualMachine::new(),
        };
    }
    /// Entrypoint to execute a SQL query.
    pub fn execute(&mut self, user_input: String) -> ExecutionResult {
        info!("Executing user input: {}", user_input);
        let result = parse(user_input);
        match result {
            Ok(commands) => {
                let command_result = self.virtual_machine.execute(commands);
                // translate CommandResult into ExecutionResult
                // we do not want to make the outer layer import any enum except ExecutionResult
                match command_result {
                    CommandResult::RetrievedDataSuccess(table) => {
                        info!("Retrieved data successfully");
                        return ExecutionResult::TableResult(Box::new(table));
                    }
                    CommandResult::VoidSuccess => {
                        info!("Command successful");
                        return ExecutionResult::VoidOK;
                    }
                    CommandResult::Error(error) => {
                        error!("Command failed: {:?}", error);
                        return ExecutionResult::CommandError(error);
                    }
                }
            }
            // translate ParseError into ExecutionResult
            Err(ParseError::Error(error)) => {
                error!("Parse error: {:?}", error);
                return ExecutionResult::ParseError(error);
            }
        }
    }
}
