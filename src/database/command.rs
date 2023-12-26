use crate::database::table::Table;

pub enum Command {
    SelectFrom(Vec<String>, String), // columns, table_name
    Invalid(),
}

pub enum CommandResult {
    RetrievedDataSuccess(Table),
    VoidSuccess(),
    Error(String),
}
