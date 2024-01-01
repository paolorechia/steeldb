//! VirtualMachine that takes parsed interpreted as commands and executes them.
//! This effectively maps the Parser output into an actual code.
use crate::database::command::{Command, CommandResult};
use crate::database::file_io::FileFormat;
use crate::database::table::Table;

/// For now, an empty struct, but could be extended.
pub struct VirtualMachine {}

impl VirtualMachine {
    /// Class constructor
    pub fn new() -> VirtualMachine {
        return VirtualMachine {};
    }

    /// Main entry point, executes a vector of [Command] type, in the order given.
    pub fn execute(&self, commands: Vec<Command>) -> CommandResult {
        // keep track of last command execution
        // might be useful when implementing nested commands
        let mut maybe_command_result: Option<CommandResult> = None;

        // the reason we implement this as a list of commands is to supported
        // the execution of nested commands in the future
        // this assumes the parser built a list of commands in the right order of execution
        for command in commands {
            if let Command::SelectFrom(columns, table_name) = command {
                let table_result = Table::load(table_name, columns, FileFormat::SimpleColumnar);

                // if we found an error, we want to immediately abort the nested execution
                if table_result.is_err() {
                    let error = format!("{:?}", table_result.unwrap_err());
                    return CommandResult::Error(error);
                } else {
                    // if our command succeeds, we want to save the result in case the next command needs it
                    let table = table_result.unwrap();
                    maybe_command_result = Some(CommandResult::RetrievedDataSuccess(table));
                }
            } else if let Command::Stub = command {
                return CommandResult::VoidSuccess;
            };
        }

        // once we finish going through the list, the last command result is our final one, let's return it
        match maybe_command_result {
            Some(command_result) => command_result,
            None => CommandResult::Error("Empty command FIFO".to_string()),
        }
    }
}
