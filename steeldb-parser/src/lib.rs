use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub select); // synthesized by LALRPOP

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
        parser.parse(&mut result, "select brigadeiro, churros;").unwrap();
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
}