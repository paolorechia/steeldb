//! Public interface of table.
use crate::database::datatypes::DataType;
use std::collections::HashMap;

/// This defines a way to keep the data in-memory by the SteelDB.
/// It also represents the Table that the user receives back when querying the database.
/// This is currently in a columnar format.
/// Most of the exposed functionality here is a low level API meant to be used during the
/// database development. It is not meant to be used directly by database users.
#[derive(Debug)]
pub struct Table {
    /// The table name, this is used as an identifier for retrieving the correct table.
    pub name: String,
    /// The table fields or schema.
    pub fields: HashMap<String, DataType>,
    /// The actual data stored in columnar format.
    pub columns: HashMap<String, Vec<DataType>>,
    /// Used when retrieving data, allowing for projection push-down on query.
    /// That is, the Database do not read columns that were not specified in the query.  
    pub select_columns: Vec<String>,
}

/// The defined errors that might occur when loading or saving a table.
/// This is forwarded back by the VirtualMachine.
#[derive(Debug)]
pub enum TableErrors {
    /// The table with the given name was not found.
    TableNotFound,
    /// Attempted to save a table with a name that already exists.
    TableAlreadyExists,
    /// The select column was not found in the table.
    ColumnNotFound(String),
    /// Unspecified write error when saving the table.
    WriteError(String),
    /// Unspecified read error when loading the table.
    ReadError(String),
    /// Generic unspecified error.
    Error(String),
}

/// Defines how the table should be saved.
/// This is a low level API and not meant to used directly
/// by database users.
#[derive(Debug)]
pub enum SaveMode {
    Overwrite,
    Append,
}
