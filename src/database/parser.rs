use steeldb_parser::select::SelectParser;

pub fn parse_select(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let parser = SelectParser::new();
    parser.parse(&mut result, input.as_str()).unwrap();
    return result;
}