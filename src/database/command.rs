use crate::database::table::Table;

pub enum Command {
    SelectFrom(Vec<String>, String), // columns, table_name
}

pub enum CommandResult {
    RetrievedDataSuccess(Table),
    VoidSuccess(),
    UnknownFailure(String),
    UnrecognizedCommand,
}
