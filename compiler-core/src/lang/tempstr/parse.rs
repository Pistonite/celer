use super::grammar::{self, pt, Ctx};
use super::{TempStr, TempStrBlock};
use regen::sdk::{ASTParser, CreateParseTree, ParseTreeResult, TokenStream};

impl<S> From<S> for TempStr
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let lex_output = grammar::tokenize(s.as_ref());
        let mut ts = TokenStream::new(&lex_output.tokens, 16);

        let asts = match grammar::Parser.parse_ast_all(&mut ts) {
            Ok(asts) => asts,
            Err((asts, _)) => asts, // should never happen
        };

        let pts = asts
            .iter()
            .map(|ast| {
                match ast.parse_pt(Box::default()) {
                    ParseTreeResult::Ok { pt, .. } => pt,
                    ParseTreeResult::Err { pt, .. } => pt, // should never happen
                }
            })
            .collect::<Vec<_>>();

        Self(from_pts(pts))
    }
}

/// Parse parse tree roots to TempStrBlocks
pub fn from_pts(pts: Vec<pt::Block>) -> Vec<TempStrBlock> {
    let mut output = vec![];
    for pt in pts {
        match pt {
            pt::Block::Dollar(mut pt) => {
                let block = pt.take_unchecked();
                match &block {
                    TempStrBlock::Lit(block_str) => match output.last_mut() {
                        None | Some(TempStrBlock::Var(_)) => {
                            output.push(block);
                        }
                        Some(TempStrBlock::Lit(s)) => {
                            s.push_str(block_str);
                        }
                    },
                    TempStrBlock::Var(_) => {
                        output.push(block);
                    }
                }
            }
            pt::Block::NonDollar(pt) => match output.last_mut() {
                None | Some(TempStrBlock::Var(_)) => {
                    let mut str = String::new();
                    append_non_dollar(&pt, &mut str);
                    output.push(TempStrBlock::Lit(str));
                }
                Some(TempStrBlock::Lit(s)) => {
                    append_non_dollar(&pt, s);
                }
            },
        }
    }
    output
}

/// Parse tree hook for the Dollar node
pub fn parse_dollar(pt: &pt::Dollar, _ctx: &mut Ctx) -> Option<TempStrBlock> {
    match pt.m_tail.as_ref() {
        None => {
            // just a dollar sign
            Some(TempStrBlock::Lit("$".to_string()))
        }
        Some(pt_tail) => {
            // set the semantic of first dollar sign to be variable
            // only need to enable for wasm
            #[cfg(feature = "wasm")]
            {
                use super::grammar::Tok;
                _ctx.tbs.set(&pt.ast.m_0, Tok::SVariable);
            }

            match pt_tail.as_ref() {
                pt::DollarTail::Escape(_) => {
                    // double dollar sign
                    Some(TempStrBlock::Lit("$".to_string()))
                }
                pt::DollarTail::Variable(pt_variable) => {
                    match pt_variable.m_arg.parse::<usize>() {
                        Ok(arg) => {
                            // variable
                            Some(TempStrBlock::Var(arg))
                        }
                        Err(_) => {
                            // cannot parse the number as valid usize
                            // treat as string
                            Some(TempStrBlock::Lit(format!("$({})", pt_variable.m_arg)))
                        }
                    }
                }
            }
        }
    }
}

fn append_non_dollar(pt: &pt::NonDollar, out: &mut String) {
    match pt {
        pt::NonDollar::Text(pt) => out.push_str(&pt.m_t),
        pt::NonDollar::Symbol(pt) => out.push_str(&pt.m_t),
        pt::NonDollar::Number(pt) => out.push_str(&pt.m_t),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(TempStr::from("").0, vec![]);
    }

    #[test]
    fn test_single_literal() {
        assert_eq!(
            TempStr::from("abcd").0,
            vec![TempStrBlock::Lit("abcd".to_string())]
        );
    }

    #[test]
    fn test_single_number() {
        assert_eq!(
            TempStr::from("12").0,
            vec![TempStrBlock::Lit("12".to_string())]
        );
    }

    #[test]
    fn test_single_dollar() {
        assert_eq!(
            TempStr::from("$").0,
            vec![TempStrBlock::Lit("$".to_string())]
        );
        assert_eq!(
            TempStr::from("$$").0,
            vec![TempStrBlock::Lit("$".to_string())]
        );
        assert_eq!(
            TempStr::from("$$$").0,
            vec![TempStrBlock::Lit("$$".to_string())]
        );
        assert_eq!(
            TempStr::from("$$$$").0,
            vec![TempStrBlock::Lit("$$".to_string())]
        );
    }

    #[test]
    fn test_single_variable() {
        assert_eq!(TempStr::from("$(0)").0, vec![TempStrBlock::Var(0)]);
        assert_eq!(TempStr::from("$(123)").0, vec![TempStrBlock::Var(123)]);
        assert_eq!(TempStr::from("$(0123)").0, vec![TempStrBlock::Var(123)]);
    }

    #[test]
    fn test_one_var_with_other() {
        assert_eq!(
            TempStr::from("abc$(0)").0,
            vec![TempStrBlock::Lit("abc".to_string()), TempStrBlock::Var(0)]
        );
        assert_eq!(
            TempStr::from("$(1)asdfa").0,
            vec![TempStrBlock::Var(1), TempStrBlock::Lit("asdfa".to_string()),]
        );
        assert_eq!(
            TempStr::from("xxyz$(4)asdfa").0,
            vec![
                TempStrBlock::Lit("xxyz".to_string()),
                TempStrBlock::Var(4),
                TempStrBlock::Lit("asdfa".to_string()),
            ]
        );
    }

    #[test]
    fn test_escape_variable() {
        assert_eq!(
            TempStr::from("$$(1)").0,
            vec![TempStrBlock::Lit("$(1)".to_string()),]
        );
        assert_eq!(
            TempStr::from("$$$(1)").0,
            vec![TempStrBlock::Lit("$".to_string()), TempStrBlock::Var(1),]
        );
    }

    #[test]
    fn test_no_nested() {
        assert_eq!(
            TempStr::from("$($(1))").0,
            vec![
                TempStrBlock::Lit("$(".to_string()),
                TempStrBlock::Var(1),
                TempStrBlock::Lit(")".to_string()),
            ]
        );
    }

    #[test]
    fn test_variable_not_number() {
        assert_eq!(
            TempStr::from("$(a)").0,
            vec![TempStrBlock::Lit("$(a)".to_string()),]
        );
    }

    #[test]
    fn test_multiple_var() {
        assert_eq!(
            TempStr::from("$(0)$(1)").0,
            vec![TempStrBlock::Var(0), TempStrBlock::Var(1),]
        );
        assert_eq!(
            TempStr::from("abc$(0)d$(1)de").0,
            vec![
                TempStrBlock::Lit("abc".to_string()),
                TempStrBlock::Var(0),
                TempStrBlock::Lit("d".to_string()),
                TempStrBlock::Var(1),
                TempStrBlock::Lit("de".to_string()),
            ]
        );
    }

    #[test]
    fn test_complicated() {
        assert_eq!(
            TempStr::from("ad)($)af$$$()he$(0)").0,
            vec![
                TempStrBlock::Lit("ad)($)af$$()he".to_string()),
                TempStrBlock::Var(0),
            ]
        );
        assert_eq!(
            TempStr::from("bar$(3)$(3) $(2)$(1)$(2)").0,
            vec![
                TempStrBlock::Lit("bar".to_string()),
                TempStrBlock::Var(3),
                TempStrBlock::Var(3),
                TempStrBlock::Lit(" ".to_string()),
                TempStrBlock::Var(2),
                TempStrBlock::Var(1),
                TempStrBlock::Var(2),
            ]
        );
    }
}
