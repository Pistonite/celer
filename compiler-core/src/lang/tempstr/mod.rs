use regen::sdk::{TokenStream, ASTParser, CreateParseTree, ParseTreeResult};
use serde::{Deserialize, Serialize};

mod parse;
use parse::*;
mod grammar;

pub struct TempStr(Vec<TempStrBlock>);

impl From<&str> for TempStr {
    fn from(s: &str) -> Self {
        let lex_output = grammar::tokenize(s);
        let mut ts = TokenStream::new(&lex_output.tokens, 16);
        
        let asts = match grammar::Parser.parse_ast_all(&mut ts) {
            Ok(asts) => asts,
            Err((asts, _)) => asts, // should never happen
        };

        let pts = asts.iter().map(|ast| {
            match ast.parse_pt(Box::default()) {
                ParseTreeResult::Ok { pt, .. } => pt, 
                ParseTreeResult::Err { pt, .. } => pt, // should never happen
            }
        }).collect::<Vec<_>>();

        Self(from_pts(pts))
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TempStrBlock {
    Lit(String),
    Var(usize),
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(TempStr::from("").0, vec![]);
    }

    #[test]
    fn test_single_literal() {
        assert_eq!(TempStr::from("abcd").0, vec![
            TempStrBlock::Lit("abcd".to_string())
        ]);
    }

    #[test]
    fn test_single_number() {
        assert_eq!(TempStr::from("12").0, vec![
            TempStrBlock::Lit("12".to_string())
        ]);
    }

    #[test]
    fn test_single_dollar() {
        assert_eq!(TempStr::from("$").0, vec![
            TempStrBlock::Lit("$".to_string())
        ]);
        assert_eq!(TempStr::from("$$").0, vec![
            TempStrBlock::Lit("$".to_string())
        ]);
        assert_eq!(TempStr::from("$$$").0, vec![
            TempStrBlock::Lit("$$".to_string())
        ]);
        assert_eq!(TempStr::from("$$$$").0, vec![
            TempStrBlock::Lit("$$".to_string())
        ]);
    }

    #[test]
    fn test_single_variable() {
        assert_eq!(TempStr::from("$(0)").0, vec![
            TempStrBlock::Var(0)
        ]);
        assert_eq!(TempStr::from("$(123)").0, vec![
            TempStrBlock::Var(123)
        ]);
        assert_eq!(TempStr::from("$(0123)").0, vec![
            TempStrBlock::Var(123)
        ]);
    }

    #[test]
    fn test_one_var_with_other() {
        assert_eq!(TempStr::from("abc$(0)").0, vec![
            TempStrBlock::Lit("abc".to_string()),
            TempStrBlock::Var(0)
        ]);
        assert_eq!(TempStr::from("$(1)asdfa").0, vec![
            TempStrBlock::Var(1),
            TempStrBlock::Lit("asdfa".to_string()),
        ]);
        assert_eq!(TempStr::from("xxyz$(4)asdfa").0, vec![
            TempStrBlock::Lit("xxyz".to_string()),
            TempStrBlock::Var(4),
            TempStrBlock::Lit("asdfa".to_string()),
        ]);
    }

    #[test]
    fn test_escape_variable() {
        assert_eq!(TempStr::from("$$(1)").0, vec![
            TempStrBlock::Lit("$(1)".to_string()),
        ]);
        assert_eq!(TempStr::from("$$$(1)").0, vec![
            TempStrBlock::Lit("$".to_string()),
            TempStrBlock::Var(1),
        ]);
    }

    #[test]
    fn test_no_nested() {
        assert_eq!(TempStr::from("$($(1))").0, vec![
            TempStrBlock::Lit("$(".to_string()),
            TempStrBlock::Var(1),
            TempStrBlock::Lit(")".to_string()),
        ]);
    }

    #[test]
    fn test_variable_not_number() {
        assert_eq!(TempStr::from("$(a)").0, vec![
            TempStrBlock::Lit("$(a)".to_string()),
        ]);
    }

    #[test]
    fn test_multiple_var() {
        assert_eq!(TempStr::from("$(0)$(1)").0, vec![
            TempStrBlock::Var(0),
            TempStrBlock::Var(1),
        ]);
        assert_eq!(TempStr::from("abc$(0)def$(1)de").0, vec![
            TempStrBlock::Lit("abc".to_string()),
            TempStrBlock::Var(0),
            TempStrBlock::Lit("def".to_string()),
            TempStrBlock::Var(1),
            TempStrBlock::Lit("de".to_string()),
        ]);
    }

    #[test]
    fn test_complicated() {
        assert_eq!(TempStr::from("ad)($)af$$$()he$(0)").0, vec![
            TempStrBlock::Lit("ad)($)af$$()he".to_string()),
            TempStrBlock::Var(0),
        ]);
    }

}
