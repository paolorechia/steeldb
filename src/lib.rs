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
//! ```rust
//! use steeldb::SteelDB;
//!
//! let database = SteelDB::new();
//! let result = database.execute("select name");
//! match result {
//!     TableResult(table) => {
//!         println!("{:?}", table);
//!     }
//!     VoidOK => println("Command OK"),
//!     ParseError(error) => println("Parse error: {:?}", error),
//!     CommandError(error) => println("Command error: {:?}", error),
//! }
//! ```
//!
//! # REPL
//! To use the REPL, one can simply install SteelDB and execute `cargo run`.
//! Effectively, this is the same as:
//!  
//!```rust
//!use steeldb::Repl;
//!
//!fn main() {
//!    let mut repl = Repl::new();
//!    repl.main_loop();
//!}
//!
//!```
//!
//! When the shell starts, one provides input which will be fed into the `execute` function of the [SteelDB] struct.
//! The REPL automatically handles pretty printing tables and printing errors back to the standard output.

mod database;
mod repl;

pub use database::steeldb::SteelDB;
pub use repl::Repl;

/// Crate version defined in `Cargo.toml` file, retrieved at runtime.
/// This is displayed in the REPL.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
