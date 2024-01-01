//! Uses the [steeldb_parser] crate to map parsing into commands that will be executed by the VirtualMachine.

use super::command::Command;
use super::config::DEFAULT_TABLE;
pub use steeldb_parser::{parse_select, ParseError};

/// The entrypoint of the parser is the [parse] function.
pub fn parse(input: String) -> Result<Vec<Command>, ParseError> {
    let result = parse_select(input);
    match result {
        Ok(columns) => {
            return Ok(vec![Command::SelectFrom(
                columns,
                DEFAULT_TABLE.to_string(),
            )]);
        }
        Err(error) => {
            return Err(error);
        }
    }
}
