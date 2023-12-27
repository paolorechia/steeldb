use crate::database::table::Table;

pub enum Command {
    SelectFrom(Vec<String>, String), // columns, table_name
    Stub,
}

pub enum CommandResult {
    RetrievedDataSuccess(Table),
    Error(String),
    VoidSuccess,
}
