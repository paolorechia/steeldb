use crate::database::command::{Command, CommandResult};
use crate::database::table::Table;
use std::collections::VecDeque;

pub struct VirtualMachine {
    pub command_stack: VecDeque<Command>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        return VirtualMachine {
            command_stack: VecDeque::<Command>::new(),
        };
    }

    pub fn execute(&self, commands: Vec<Command>) {
        commands
            .into_iter()
            .map(|command: Command| -> CommandResult {
                match command {
                    Command::SelectFrom(columns, table_name) => {
                        let table = Table::load(table_name, columns);
                        return CommandResult::RetrievedDataSuccess(table);
                    }
                    _ => {
                        return CommandResult::UnrecognizedCommand();
                    }
                }
            });
    }
}
