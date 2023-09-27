use regen::sdk::{ASTParser, CreateParseTree, ParseTreeResult, TokenStream};

use super::grammar::{self, pt};
use super::PresetInst;

impl PresetInst {
    /// Parse a string to a preset instantiation
    ///
    /// Returns None if the string is not a valid preset instantiation
    pub fn try_parse(s: &str) -> Option<Self> {
        let lex_output = grammar::tokenize(s);
        let mut ts = TokenStream::new(&lex_output.tokens, 16);
        let ast = grammar::Parser.parse_ast(&mut ts)?;
        if !ts.is_exhausted() {
            return None;
        }
        let pt = match ast.parse_pt(Box::default()) {
            ParseTreeResult::Ok { pt, .. } => pt,
            _ => return None,
        };
        Some(from_pt(&pt))
    }
}

fn from_pt(pt: &pt::Preset) -> PresetInst {
    let mut name = pt.m_namespace.to_string();
    for pt_sub in &pt.m_sub_namespaces {
        append_sub_namespace(pt_sub, &mut name);
    }
    let mut args = vec![];
    if let Some(pt_args) = &pt.m_args {
        args.push(parse_arg(&pt_args.m_first));
        for pt_arg in &pt_args.m_rest {
            args.push(parse_arg(&pt_arg.m_arg));
        }
    }

    PresetInst { name, args }
}

fn append_sub_namespace(pt: &pt::SubNamespace, out: &mut String) {
    out.push_str("::");
    out.push_str(&pt.m_name);
}

fn parse_arg(pt: &pt::ArgText) -> String {
    let mut out = String::new();
    for pt_block in &pt.m_blocks {
        match pt_block {
            pt::ArgBlock::Arg(pt_arg) => {
                out.push_str(&pt_arg.m_t);
            }
            pt::ArgBlock::ArgEscape(pt_escape) => {
                if pt_escape.m_t == "\\" {
                    out.push('\\');
                } else {
                    // remove the leading backslash
                    out.push_str(&pt_escape.m_t[1..]);
                }
            }
            pt::ArgBlock::ArgSymbol(_) => {
                out.push(':');
            }
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(PresetInst::try_parse(""), None);
    }

    #[test]
    fn test_main_namespace() {
        assert_eq!(
            PresetInst::try_parse("hello").unwrap(),
            PresetInst {
                name: "hello".to_string(),
                args: vec![],
            }
        );
    }

    #[test]
    fn test_trailing_colon() {
        assert_eq!(PresetInst::try_parse("hello:"), None);
        assert_eq!(PresetInst::try_parse("_hello::"), None);
        assert_eq!(PresetInst::try_parse("hello::world:"), None);
        assert_eq!(PresetInst::try_parse("_hello::world::"), None);
    }

    #[test]
    fn test_subnamespace() {
        assert_eq!(
            PresetInst::try_parse("hello::world").unwrap(),
            PresetInst {
                name: "hello::world".to_string(),
                args: vec![],
            }
        );
        assert_eq!(
            PresetInst::try_parse("_hello::world::2").unwrap(),
            PresetInst {
                name: "_hello::world::2".to_string(),
                args: vec![],
            }
        );
    }

    #[test]
    fn test_empty_args_not_allowed() {
        assert_eq!(PresetInst::try_parse("hello<>"), None);
        assert_eq!(PresetInst::try_parse("_hello::world<>"), None);
        assert_eq!(PresetInst::try_parse("_hello::world>"), None);
        assert_eq!(PresetInst::try_parse("_hello::world<"), None);
    }

    #[test]
    fn test_no_escape_in_namespace() {
        assert_eq!(PresetInst::try_parse("he\\\\llo"), None);
        assert_eq!(PresetInst::try_parse("_hel\\>lo::wo\\rld"), None);
        assert_eq!(PresetInst::try_parse("_hel\\,lo::world"), None);
        assert_eq!(PresetInst::try_parse("_hello::w\\\\orld"), None);
    }

    #[test]
    fn test_args() {
        assert_eq!(
            PresetInst::try_parse("hello<world>").unwrap(),
            PresetInst {
                name: "hello".to_string(),
                args: vec!["world".to_string()],
            }
        );
        assert_eq!(
            PresetInst::try_parse("hello<wo\\\\rld\\,>").unwrap(),
            PresetInst {
                name: "hello".to_string(),
                args: vec!["wo\\rld,".to_string()],
            }
        );
        assert_eq!(
            PresetInst::try_parse("hello::world<foo,bar>").unwrap(),
            PresetInst {
                name: "hello::world".to_string(),
                args: vec!["foo".to_string(), "bar".to_string()],
            }
        );
        assert_eq!(
            PresetInst::try_parse("hello::world<f\\o:o\\,bar, biz\\>>").unwrap(),
            PresetInst {
                name: "hello::world".to_string(),
                args: vec!["f\\o:o,bar".to_string(), " biz>".to_string()],
            }
        );
        assert_eq!(
            PresetInst::try_parse("hello::world<f\\o:o\\,bar\\\\, biz\\>>").unwrap(),
            PresetInst {
                name: "hello::world".to_string(),
                args: vec!["f\\o:o,bar\\".to_string(), " biz>".to_string()],
            }
        );
    }

    #[test]
    fn test_no_trailing() {
        assert_eq!(PresetInst::try_parse("hello<world> "), None);
        assert_eq!(PresetInst::try_parse("hello<world>a"), None);
    }
}
