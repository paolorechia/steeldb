#![warn(missing_docs)]
//! SteelDB Crate
//!
//! # Introduction
//!
//! SteelDB is a Database created for learning purposes.
//!
//! The goal is to implement a Database from scratch in Rust, and document the process
//! along the way.
//!
//! The main reference is SQLite architecture. However, SteelDB will differ significantly in certain areas.
//!
//! Currently, SteelDB can only be used in an embedded manner, but will eventually also
//! support a server-client architecture.
//!
//! There are two main ways of using it:
//!
//! 1. Using the [SteelDB] struct for a programmatic experience.
//! 2. Using the [Repl] struct for an interactive experience.
//!
//! Note that the current version is extremely limited, as it only supports the SELECT clause.
//! # Examples
//! ### Database API
//!
//! ```no_run
//! use steeldb::{SteelDB, ExecutionResult, SteelDBInterface};
//!
//! let mut database = SteelDB::new();
//! let result = database.execute("select name".to_string());
//! match result {
//!     ExecutionResult::TableResult(table) => {
//!         println!("{:?}", table);
//!     }
//!     ExecutionResult::VoidOK => println!("Command OK"),
//!     ExecutionResult::ParseError(error) => println!("Parse error: {:?}", error),
//!     ExecutionResult::CommandError(error) => println!("Command error: {:?}", error),
//! }
//! ```
//!
//! # REPL
//! To use the REPL, one can simply install SteelDB and execute `cargo run`.
//! Effectively, this is the same as:
//!  
//!```no_run
//!use steeldb::SteelDB;
//!use steeldb_core::Repl;
//!
//!fn main() {
//!    let database = SteelDB::new();
//!    let mut repl = Repl::new(Box::new(database));
//!    repl.main_loop();
//!}
//!
//!```
//!
//! When the shell starts, one provides input which will be fed into the `execute` function of the [SteelDB] struct.
//! The REPL automatically handles pretty printing tables and printing errors back to the standard output.

mod database;

pub use database::config;
pub use database::steeldb::SteelDB;
pub use steeldb_core::{DataType, ExecutionResult, SteelDBInterface, Table, TableErrors};
