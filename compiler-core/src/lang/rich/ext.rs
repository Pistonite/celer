//! Rich text extensions
use crate::types::DocRichText;

pub trait RichTextExt {
    fn starts_with(&self, text: &str) -> bool;
}

impl RichTextExt for Vec<DocRichText> {
    fn starts_with(&self, mut text: &str) -> bool {
        if text.is_empty() {
            return true;
        }
        for block in self {
            let t = &block.text;
            let l = t.len();
            if text.len() < l {
                return t.starts_with(text);
            }
            if text.starts_with(t) {
                text = &text[l..];
            } else {
                return false;
            }
        }

        text.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::lang::parse_rich;

    use super::*;

    fn test_a_starts_with_b(a: &str, b: &str) -> bool {
        parse_rich(a).starts_with(b)
    }

    #[test]
    fn test_empty() {
        assert!(test_a_starts_with_b("", ""));
        assert!(test_a_starts_with_b("hello", ""));
        assert!(test_a_starts_with_b(".tag(hello)", ""));

        assert!(!test_a_starts_with_b("", "x"));
    }

    #[test]
    fn test_first_block_match() {
        assert!(test_a_starts_with_b("hello", "hel"));
        assert!(test_a_starts_with_b("hello", "hello"));
        assert!(test_a_starts_with_b(".tag(hello)", "h"));

        assert!(!test_a_starts_with_b("hello", "x"));
        assert!(!test_a_starts_with_b("hello", "xyzws"));
        assert!(!test_a_starts_with_b(".tag(hello)", "xxx"));
    }

    #[test]
    fn test_many_blocks_match() {
        assert!(test_a_starts_with_b("hello .tag(xxx)", "hello x"));
        assert!(test_a_starts_with_b("hello .tag(xxx)", "hello xxx"));
        assert!(test_a_starts_with_b(".tag(hello) xxx", "hello xxx"));

        assert!(!test_a_starts_with_b("hello .tag(yyy)", "hello x"));
        assert!(!test_a_starts_with_b("hello. tag(yyy)", "hello yyya"));
        assert!(!test_a_starts_with_b(".tag(hello) yyy", "hello x"));
    }
}
