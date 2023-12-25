use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub select); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_parser_single_column() {
        let parser = select::SelectParser::new();
        assert!(parser.parse("select churros;").is_ok());
        let v = vec!["churros".to_string()];
        let parsed = parser.parse("select churros;").unwrap();
        assert_eq!(v, parsed);
    }

    fn test_select_support_star() {
        let parser = select::SelectParser::new();
        assert!(parser.parse("select *;").is_ok());
        let r = parser.parse("select *;").unwrap();
        assert_eq!(r, vec!["*".to_string()]);
    }
}
