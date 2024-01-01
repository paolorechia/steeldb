#![warn(missing_docs)]
//! # SteelDB Parser
//! This crate exposes functions that parse a subset of SQL, used by the SteelDB project.
//! 
//! 
//! You can find more information about the Database here: <https://github.com/paolorechia/steeldb>
//! 
//! 
//! Since this is still work in progress, not much is implemented.
//! 
//! 
//! Currently, the only exposed function is [parse_select], which takes an input string and returns the 
//! columns that were given in the SELECT clause.
//! 
//! 
//! This crate relies on lalrpop library: <https://github.com/lalrpop/lalrpop>
//! 
//! 
//! # Examples
//! Good examples of this crate usage are found in the unit tests in lib.rs
//! For instance:
//! 
//! ```rust
//!     #[test]
//!     fn test_parse_select() {
//!         let result = parse_select("select brigadeiro, churros;".to_string()).unwrap();
//!         let v = vec!["brigadeiro".to_string(), "churros".to_string()];
//!         assert_eq!(v, result);
//!     }
//! ````
//!
//! # Grammar Files  
//! Note that `lalrpop` reads a file of the format `.lalrpop` where the parser grammar is defined,
//! and generated during compilation-time the actual parser code, which is not displayed in the source code repository.
//! 
//! Here's the first implementation of the select clause:
//! 
//! ```txt
//! grammar(v: &mut Vec<String>);
//! 
//! pub Select: () = {
//!     SELECT <c:Columns> SEMICOLON => {}
//! };
//! 
//! Columns: () = {
//!     <l:LITERAL> => v.push(l),
//!     Columns "," <l:LITERAL> => {
//!         v.push(l);
//!     }
//! }
//! 
//! SELECT: String = <s:r"select "> => s.to_string();
//! LITERAL: String = <s:r"[a-z\*_0-9]+"> => s.to_string();
//! SEMICOLON: String = <s:r";"> => s.to_string();
//! ```

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(select); // synthesized by LALRPOP

/// Enum used for propagating the parse error.
/// At the moment it only contains one generic Error.
/// Internally, this library just forwards the lalrpop error as a formatted string:
/// ```rust
/// Err(error) => {
///     let error = format!("{:?}", error);
///     return Err(ParseError::Error(format!(
///         "Failed to parse, error: {}",
///         error
///     )));
/// } 
/// ```
#[derive(Debug)]
pub enum ParseError {
    /// Generic parser error case.
    Error(String),
}

/// Parses a select clause in the format 'select col1, col2;'.
/// 
/// Example:
/// ```rust
/// let result = parse_select("select brigadeiro, churros;".to_string()).unwrap();
/// let v = vec!["brigadeiro".to_string(), "churros".to_string()];
/// assert_eq!(v, result);
/// ```
/// Notice that this function does not yet support the FROM clause.
pub fn parse_select(input: String) -> Result<Vec<String>, ParseError> {
    let mut result: Vec<String> = vec![];
    let parser = select::SelectParser::new();
    let maybe_error = parser.parse(&mut result, input.as_str());
    match maybe_error {
        Ok(_) => {
            return Ok(result);
        }
        Err(error) => {
            let error = format!("{:?}", error);
            return Err(ParseError::Error(format!(
                "Failed to parse, error: {}",
                error
            )));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_parser_single_column() {
        let mut result: Vec<String> = vec![];
        let parser = select::SelectParser::new();
        assert!(parser.parse(&mut result, "select churros;").is_ok());
        let v = vec!["churros".to_string()];
        assert_eq!(v, result);
    }

    #[test]
    fn test_select_parser_multiple_columns() {
        let mut result: Vec<String> = vec![];
        let parser = select::SelectParser::new();
        parser
            .parse(&mut result, "select brigadeiro, churros;")
            .unwrap();
        let v = vec!["brigadeiro".to_string(), "churros".to_string()];
        assert_eq!(v, result);
    }

    #[test]
    fn test_select_support_star() {
        let mut result: Vec<String> = vec![];
        let parser = select::SelectParser::new();
        assert!(parser.parse(&mut result, "select *;").is_ok());
        assert_eq!(result, vec!["*".to_string()]);
    }

    #[test]
    fn test_parse_select() {
        let result = parse_select("select brigadeiro, churros;".to_string()).unwrap();
        let v = vec!["brigadeiro".to_string(), "churros".to_string()];
        assert_eq!(v, result);
    }
}
