use std::fmt::Display;

use crate::macros::derive_wasm;

/// Document rich text
///
/// This is a collection of [`DocRichTextBlock`]s
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct DocRichText(pub Vec<DocRichTextBlock>);

/// Document rich text block
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct DocRichTextBlock {
    /// The tag name of the text
    ///
    /// Each block only contains one tag
    pub tag: Option<String>,
    /// The text content
    pub text: String,
    /// The hyperlink of the text.
    pub link: Option<String>,
}

impl DocRichText {
    /// Create a new [`DocRichText`] with a single [`DocRichTextBlock`] with the text content and
    /// no tags
    #[inline]
    pub fn text(text: &str) -> Self {
        Self(vec![DocRichTextBlock::text(text)])
    }

    /// Iterate over the [`DocRichTextBlock`]s
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &DocRichTextBlock> {
        self.0.iter()
    }

    /// Iterate over the [`DocRichTextBlock`]s
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut DocRichTextBlock> {
        self.0.iter_mut()
    }

    /// Test if the text starts with the given prefix. Tags are ignored.
    pub fn starts_with(&self, mut prefix: &str) -> bool {
        if prefix.is_empty() {
            return true;
        }
        for block in self.iter() {
            let t = &block.text;
            let l = t.len();
            if prefix.len() < l {
                return t.starts_with(prefix);
            }
            if prefix.starts_with(t) {
                prefix = &prefix[l..];
            } else {
                return false;
            }
        }

        prefix.is_empty()
    }
}

impl DocRichTextBlock {
    /// Create a rich text block with no tag
    pub fn text(text: &str) -> Self {
        Self {
            tag: None,
            text: text.to_string(),
            link: None,
        }
    }

    /// Create a rich text block with a tag
    pub fn with_tag(tag: &str, text: &str) -> Self {
        Self {
            tag: Some(tag.to_string()),
            text: text.to_string(),
            link: None,
        }
    }
}

impl Display for DocRichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.iter() {
            write!(f, "{}", block.text)?;
        }
        Ok(())
    }
}

impl IntoIterator for DocRichText {
    type Item = DocRichTextBlock;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::lang;

    fn test_a_starts_with_b(a: &str, b: &str) -> bool {
        lang::parse_rich(a).starts_with(b)
    }

    #[test]
    fn test_starts_with_empty() {
        assert!(test_a_starts_with_b("", ""));
        assert!(test_a_starts_with_b("hello", ""));
        assert!(test_a_starts_with_b(".tag(hello)", ""));

        assert!(!test_a_starts_with_b("", "x"));
    }

    #[test]
    fn test_starts_with_first_block_match() {
        assert!(test_a_starts_with_b("hello", "hel"));
        assert!(test_a_starts_with_b("hello", "hello"));
        assert!(test_a_starts_with_b(".tag(hello)", "h"));

        assert!(!test_a_starts_with_b("hello", "x"));
        assert!(!test_a_starts_with_b("hello", "xyzws"));
        assert!(!test_a_starts_with_b(".tag(hello)", "xxx"));
    }

    #[test]
    fn test_starts_with_many_blocks_match() {
        assert!(test_a_starts_with_b("hello .tag(xxx)", "hello x"));
        assert!(test_a_starts_with_b("hello .tag(xxx)", "hello xxx"));
        assert!(test_a_starts_with_b(".tag(hello) xxx", "hello xxx"));

        assert!(!test_a_starts_with_b("hello .tag(yyy)", "hello x"));
        assert!(!test_a_starts_with_b("hello. tag(yyy)", "hello yyya"));
        assert!(!test_a_starts_with_b(".tag(hello) yyy", "hello x"));
    }
}
