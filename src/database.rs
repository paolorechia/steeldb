//! This is a private module, and not meant to be imported directly.
//! It's the root module of the SteelDB package, being the entrypoint into the Database code.
//! Everything that should be accessed is re-exported in the 'lib.rs' file.

mod command;
pub mod config;
pub mod datatypes;
mod file_io;
mod parser;
pub mod steeldb;
pub mod table;
mod tests;
mod virtual_machine;
