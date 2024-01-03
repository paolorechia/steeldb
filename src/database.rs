//! This is a private module, and not meant to be imported directly.
//! It's the root module of the SteelDB package, being the entrypoint into the Database code.
//! Everything that should be accessed is re-exported in the 'lib.rs' file.

pub mod datatypes;

mod logger;

pub mod table;

#[cfg(feature = "database")]
mod table_impl;

#[cfg(feature = "database")]
mod command;

#[cfg(feature = "database")]
pub mod config;

#[cfg(feature = "database")]
pub mod console_printer;

#[cfg(feature = "database")]
mod file_io;

#[cfg(feature = "database")]
mod parser;

#[cfg(feature = "database")]
pub mod repl;

#[cfg(feature = "database")]
pub mod steeldb;

#[cfg(feature = "database")]
mod tests;

#[cfg(feature = "database")]
mod virtual_machine;

#[cfg(feature = "server")]
pub mod server;
