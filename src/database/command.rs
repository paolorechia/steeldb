//! Defines commands that the VirtualMachine may execute.
use crate::database::table::Table;

/// All known commands are defined in this enum.
pub enum Command {
    /// The Select From Command, returns columns and table_name parsed from the string.
    SelectFrom(Vec<String>, String),
    /// Stub, will be removed once a second command is added.
    Stub,
}

/// Defines possible results from a command execution.
pub enum CommandResult {
    /// Command returned a table.
    RetrievedDataSuccess(Table),
    /// Command failed by an unexpected reason.
    Error(String),
    /// Command succeded but has no output.
    VoidSuccess,
}
