use regen::sdk::{ASTParser, CreateParseTree, ParseTreeResult, TokenStream};

use super::grammar::{self, pt};
use super::{DocRichText, DocRichTextBlock};

pub fn parse_rich(s: &str) -> DocRichText {
    let lex_output = grammar::tokenize(s);
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
    DocRichText(from_pts(&pts))
}

fn from_pts(pt: &[pt::Block]) -> Vec<DocRichTextBlock> {
    let mut out = vec![];
    for pt in pt {
        match pt {
            pt::Block::Text(pt_text) => match out.last_mut() {
                Some(DocRichTextBlock {
                    tag: None,
                    ref mut text,
                    ..
                }) => {
                    for pt_unit in pt_text.m_t.iter() {
                        append_unit_to_string(pt_unit, text);
                    }
                }
                _ => {
                    let mut text = String::new();
                    for pt_unit in pt_text.m_t.iter() {
                        append_unit_to_string(pt_unit, &mut text);
                    }
                    out.push(DocRichTextBlock {
                        tag: None,
                        text,
                        link: None,
                    });
                }
            },
            pt::Block::TagExp(pt_tagexp) => {
                out.push(parse_tagexp(pt_tagexp));
            }
            pt::Block::Symbol(pt_symbol) => match out.last_mut() {
                Some(DocRichTextBlock {
                    tag: None, text, ..
                }) => {
                    text.push_str(&pt_symbol.m_t);
                }
                _ => {
                    out.push(DocRichTextBlock {
                        tag: None,
                        text: pt_symbol.m_t.to_string(),
                        link: None,
                    });
                }
            },
            pt::Block::Space(pt_space) => match out.last_mut() {
                Some(DocRichTextBlock {
                    tag: None, text, ..
                }) => {
                    text.push_str(&pt_space.m_t);
                }
                _ => {
                    out.push(DocRichTextBlock {
                        tag: None,
                        text: pt_space.m_t.to_string(),
                        link: None,
                    });
                }
            },
        }
    }
    out
}

/// Parse tree hook for TagExp
fn parse_tagexp(pt: &pt::TagExp) -> DocRichTextBlock {
    let tag = pt.m_tag.to_string();
    let mut arg = String::new();
    if let Some(str) = &pt.m_space {
        arg.push_str(str);
    }
    for pt_unit in pt.m_arg.iter() {
        append_unit_inside_tag_to_string(pt_unit, &mut arg);
    }
    DocRichTextBlock {
        tag: Some(tag),
        text: arg,
        link: None,
    }
}
fn append_unit_inside_tag_to_string(pt: &pt::UnitInsideTag, out: &mut String) {
    match pt {
        pt::UnitInsideTag::Unit(pt) => {
            append_unit_to_string(pt, out);
        }
        pt::UnitInsideTag::UnitDotSymbol(_) => {
            out.push('.');
        }
        pt::UnitInsideTag::UnitOpenParenSymbol(_) => {
            out.push('(');
        }
    }
}

fn append_unit_to_string(pt: &pt::Unit, out: &mut String) {
    match pt {
        pt::Unit::UnitId(pt_id) => {
            out.push_str(&pt_id.m_t);
            if let Some(str) = &pt_id.m_s {
                out.push_str(str);
            }
        }
        pt::Unit::UnitEscape(pt_escape) => {
            // remove the leading backslash
            debug_assert!(pt_escape.m_t.starts_with('\\'));
            if pt_escape.m_t == "\\" {
                out.push('\\');
            } else {
                out.push_str(&pt_escape.m_t[1..]);
            }
            if let Some(str) = &pt_escape.m_s {
                out.push_str(str);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(parse_rich(""), DocRichText(vec![]));
    }

    #[test]
    fn test_words() {
        assert_eq!(
            parse_rich("hello"),
            DocRichText(vec![DocRichTextBlock {
                tag: None,
                text: "hello".to_string(),
                link: None,
            }])
        );
        assert_eq!(
            parse_rich("hello world"),
            DocRichText(vec![DocRichTextBlock {
                tag: None,
                text: "hello world".to_string(),
                link: None,
            }])
        );
    }

    #[test]
    fn test_tags() {
        assert_eq!(
            parse_rich(".tag(hello)"),
            DocRichText(vec![DocRichTextBlock {
                tag: Some("tag".to_string()),
                text: "hello".to_string(),
                link: None,
            }])
        );
        assert_eq!(
            parse_rich(".tag(hello).tag2-zzz(world foo bar)"),
            DocRichText(vec![
                DocRichTextBlock {
                    tag: Some("tag".to_string()),
                    text: "hello".to_string(),
                    link: None,
                },
                DocRichTextBlock {
                    tag: Some("tag2-zzz".to_string()),
                    text: "world foo bar".to_string(),
                    link: None,
                }
            ])
        );
    }

    #[test]
    fn test_empty_tagged_string() {
        assert_eq!(
            parse_rich("something.tag()"),
            DocRichText(vec![
                DocRichTextBlock {
                    tag: None,
                    text: "something".to_string(),
                    link: None
                },
                DocRichTextBlock {
                    tag: Some("tag".to_string()),
                    text: "".to_string(),
                    link: None
                }
            ])
        );
    }

    #[test]
    fn test_non_tags() {
        assert_eq!(
            parse_rich("this is a normal sentence. this is normal"),
            DocRichText(vec![DocRichTextBlock {
                tag: None,
                text: "this is a normal sentence. this is normal".to_string(),
                link: None
            }])
        );
        assert_eq!(
            parse_rich("this is a (normal sentence). this (is) normal"),
            DocRichText(vec![DocRichTextBlock {
                tag: None,
                text: "this is a (normal sentence). this (is) normal".to_string(),
                link: None
            }])
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(
            parse_rich("\\.tag(hello)"),
            DocRichText(vec![DocRichTextBlock {
                tag: None,
                text: ".tag(hello)".to_string(),
                link: None
            }])
        );
        assert_eq!(
            parse_rich(".tag(hello\\) continue)"),
            DocRichText(vec![DocRichTextBlock {
                tag: Some("tag".to_string()),
                text: "hello) continue".to_string(),
                link: None
            }])
        );
        assert_eq!(
            parse_rich(".tag(hello\\continue)"),
            DocRichText(vec![DocRichTextBlock {
                tag: Some("tag".to_string()),
                text: "hello\\continue".to_string(),
                link: None
            }])
        );
        assert_eq!(
            parse_rich(".\\\\tag(hellocontinue)"),
            DocRichText(vec![DocRichTextBlock {
                tag: None,
                text: ".\\tag(hellocontinue)".to_string(),
                link: None
            }])
        );
    }

    #[test]
    fn test_dot_in_text() {
        assert_eq!(
            parse_rich(".tag([hello]continue.me)"),
            DocRichText(vec![DocRichTextBlock {
                tag: Some("tag".to_string()),
                text: "[hello]continue.me".to_string(),
                link: None
            }])
        );
    }

    #[test]
    fn test_open_paren_in_text() {
        assert_eq!(
            parse_rich(".tag([hello]co(ntinue.me)"),
            DocRichText(vec![DocRichTextBlock {
                tag: Some("tag".to_string()),
                text: "[hello]co(ntinue.me".to_string(),
                link: None
            }])
        );
    }
}
