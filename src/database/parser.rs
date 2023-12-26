use super::command::Command;
use super::table_registry::DEFAULT_TABLE;
use steeldb_parser::select::SelectParser;

pub enum ParseError {
    Error(String),
}

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

fn parse_select(input: String) -> Result<Vec<String>, ParseError> {
    let mut result: Vec<String> = vec![];
    let parser = SelectParser::new();
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
