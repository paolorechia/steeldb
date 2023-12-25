use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub select); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_parser() {
        assert!(select::SelectParser::new().parse("select churros;").is_ok());
        let v = vec!["churros".to_string()];
        let parsed = select::SelectParser::new().parse("select churros;").unwrap();
        assert_eq!(v, parsed);
    }
}
