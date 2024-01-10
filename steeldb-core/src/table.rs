//! Public interface of table.

use crate::DataType;
use std::collections::HashMap;

// Enums
/// Defines the supported file formats by the Database
#[derive(Debug)]
pub enum FileFormat {
    /// The only supported file for now is the SimpleColumnar, which is a naive ASCII format.
    /// Here is an example of this format:
    /// ```txt
    /// TABLE COLUMNAR FORMAT HEADER
    /// Field name: final_grade; Type: f32; Number of elements: 3
    /// 4.0
    /// 3.2
    /// 5
    /// Field name: name; Type: String; Number of elements: 3
    /// John Man
    /// Lenon
    /// Mary
    /// Field name: annual_salary; Type: i32; Number of elements: 3
    /// 60000
    /// 200000
    /// 3012000
    ///
    /// ```
    /// Notice that the newline at the end is not optional.
    SimpleColumnar,
}

pub trait Table {
    fn save(&self, mode: SaveMode, format: FileFormat) -> Result<(), TableErrors>;
    fn load(
        &self,
        table_name: String,
        select_columns: Vec<String>,
        format: FileFormat,
    ) -> Result<Box<dyn Table>, TableErrors>;
    fn get_table_name(&self) -> String;
    fn get_columns(&self) -> &HashMap<String, Vec<DataType>>;
    fn get_select_columns(&self) -> &Vec<String>;
}

use core::fmt::Debug;
impl Debug for dyn Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let table_name = self.get_table_name();
        write!(f, "Table<{:?} ", table_name)
    }
}



/// The return type given by the [SteelDB::execute] function.
#[derive(Debug)]
pub enum ExecutionResult {
    /// A result where a table was successfully computed/retrieved, and is available for inspection.
    TableResult(Box<dyn Table>),
    /// A result where a command was successfully executed, but with no output.
    VoidOK,
    /// Parse error. The given input string was not valid for the parser.
    ParseError(String),
    /// Command error. Something went wrong when executing the command.
    /// Examples include `ColumnNotFound`, `TableNotFound` etc.
    CommandError(String),
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
