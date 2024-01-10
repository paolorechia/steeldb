pub mod datatypes;
pub mod table;
pub mod repl;
pub mod console_printer;
pub mod steeldb_interface;

pub use crate::table::{Table, TableErrors, SaveMode, ExecutionResult, FileFormat};
pub use crate::datatypes::DataType;
pub use crate::repl::Repl;
pub use crate::steeldb_interface::SteelDBInterface;

/// Crate version defined in `Cargo.toml` file, retrieved at runtime.
/// This is displayed in the REPL.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
