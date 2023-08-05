use celerctypes::DocRichText;
use regen::sdk::{TokenStream, ASTParser, CreateParseTree, ParseTreeResult};
use super::grammar::{self, pt};

pub fn parse_rich(s: &str) -> Vec<DocRichText> {
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
    from_pts(&pts)
}

fn from_pts(pt: &[pt::Block]) -> Vec<DocRichText> {
    let mut out = vec![];
    for pt in pt {
        match pt {
            pt::Block::Text(pt_text) => {
                match out.last_mut() {
                    Some(DocRichText {tag: None, ref mut text}) => {
                        for pt_unit in pt_text.m_t.iter() {
                            append_unit_to_string(pt_unit, text);
                        }
                    },
                    _ => {
                        let mut text = String::new();
                        for pt_unit in pt_text.m_t.iter() {
                            append_unit_to_string(pt_unit, &mut text);
                        }
                        out.push(DocRichText {
                            tag: None,
                            text
                        });
                    }
                }
            },
            pt::Block::TagExp(pt_tagexp) => {
                out.push(parse_tagexp(pt_tagexp));
            },
            pt::Block::Symbol(pt_symbol) => {
                match out.last_mut() {
                    Some(DocRichText {tag: None, text}) => {
                        text.push_str(&pt_symbol.m_t);
                    },
                    _ => {
                        out.push(DocRichText {
                            tag: None,
                            text: pt_symbol.m_t.to_string()
                        });
                    }
                }
            },
            pt::Block::Space(pt_space) => {
                match out.last_mut() {
                    Some(DocRichText {tag: None, text}) => {
                        text.push_str(&pt_space.m_t);
                    },
                    _ => {
                        out.push(DocRichText {
                            tag: None,
                            text: pt_space.m_t.to_string()
                        });
                    }
                }
            }
        }
    }
    out
}

/// Parse tree hook for TagExp
fn parse_tagexp(pt: &pt::TagExp) -> DocRichText {
    let tag = pt.m_tag.to_string();
    let mut arg = String::new();
    if let Some(str) = &pt.m_space {
        arg.push_str(str);
    }
    for pt_unit in pt.m_arg.iter() {
        append_unit_to_string(pt_unit, &mut arg);
    }
    DocRichText {
        tag: Some(tag),
        text: arg
    }
}

fn append_unit_to_string(pt: &pt::Unit, out: &mut String) {
    match pt {
        pt::Unit::UnitId(pt_id) => {
            out.push_str(&pt_id.m_t);
            if let Some(str) = &pt_id.m_s {
                out.push_str(str);
            }
        },
        pt::Unit::UnitEscape(pt_escape) => {
            // remove the leading backslash
            debug_assert!(pt_escape.m_t.starts_with("\\"));
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
        assert_eq!(parse_rich(""), vec![]);
    }

    #[test]
    fn test_words() {
        assert_eq!(
            parse_rich("hello"),
            vec![DocRichText { 
                tag: None,
                text: "hello".to_string()
            }]
        );
        assert_eq!(
            parse_rich("hello world"),
            vec![DocRichText { 
                tag: None,
                text: "hello world".to_string()
            }]
        );
    }

    #[test]
    fn test_tags() {
        assert_eq!(
            parse_rich(".tag(hello)"),
            vec![
                DocRichText { 
                    tag: Some("tag".to_string()),
                    text: "hello".to_string()
                },
            ]
        );
        assert_eq!(
            parse_rich(".tag(hello).tag2-zzz(world foo bar)"),
            vec![
                DocRichText { 
                    tag: Some("tag".to_string()),
                    text: "hello".to_string()
                },
                DocRichText { 
                    tag: Some("tag2-zzz".to_string()),
                    text: "world foo bar".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_empty_tagged_string() {
        assert_eq!(
            parse_rich("something.tag()"),
            vec![
                DocRichText { 
                    tag: None,
                    text: "something".to_string()
                },
                DocRichText { 
                    tag: Some("tag".to_string()),
                    text: "".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_non_tags() {
        assert_eq!(
            parse_rich("this is a normal sentence. this is normal"),
            vec![
                DocRichText { 
                    tag: None,
                    text: "this is a normal sentence. this is normal".to_string()
                }
            ]
        );
        assert_eq!(
            parse_rich("this is a (normal sentence). this (is) normal"),
            vec![
                DocRichText { 
                    tag: None,
                    text: "this is a (normal sentence). this (is) normal".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(
            parse_rich("\\.tag(hello)"),
            vec![
                DocRichText { 
                    tag: None,
                    text: ".tag(hello)".to_string()
                }
            ]
        );
        assert_eq!(
            parse_rich(".tag(hello\\) continue)"),
            vec![
                DocRichText { 
                    tag: Some("tag".to_string()),
                    text: "hello) continue".to_string()
                }
            ]
        );
        assert_eq!(
            parse_rich(".tag(hello\\continue)"),
            vec![
                DocRichText { 
                    tag: Some("tag".to_string()),
                    text: "hello\\continue".to_string()
                }
            ]
        );
        assert_eq!(
            parse_rich(".\\\\tag(hellocontinue)"),
            vec![
                DocRichText { 
                    tag: None,
                    text: ".\\tag(hellocontinue)".to_string()
                }
            ]
        );
    }
}
