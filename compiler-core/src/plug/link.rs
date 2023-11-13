//! Link plugin
//!
//! This plugin looks for the `link` tag and transforms it into a link.

use crate::api::{CompilerContext, CompilerMetadata};
use crate::comp::CompDoc;
use crate::macros::async_trait;
use crate::pack::PackerResult;
use crate::prop;
use crate::types::{DocColor, DocRichText, DocTag, DocRichTextBlock};

use super::{operation, PlugResult, PluginRuntime};

pub struct LinkPlugin;
#[async_trait(?Send)]
impl PluginRuntime for LinkPlugin {
    async fn on_pre_compile(&mut self, ctx: &mut CompilerContext) -> PackerResult<()> {
        // add the link tag if not defined already
        let link_tag = DocTag {
            color: Some(DocColor::LightDark {
                light: Some("var(--link-text-color-light)".to_string()),
                dark: Some("var(--link-text-color-dark)".to_string()),
            }),
            background: Some(DocColor::LightDark {
                light: Some("var(--link-text-background-light)".to_string()),
                dark: Some("var(--link-text-background-dark)".to_string()),
            }),
            ..Default::default()
        };
        ctx.phase0
            .project
            .tags
            .entry(prop::LINK.to_string())
            .and_modify(|tag| tag.apply_to_default(&link_tag))
            .or_insert(link_tag);
        Ok(())
    }
    async fn on_compile(&mut self, _: &CompilerMetadata, comp_doc: &mut CompDoc) -> PlugResult<()> {
        for preface in comp_doc.preface.iter_mut() {
            for block in preface.iter_mut() {
                transform_link_tag(block);
            }
        }
        operation::for_each_line!(line in comp_doc {
            operation::for_each_rich_text_except_counter!(rich_text in line {
                transform_link_tag(rich_text);
            });
            if let Some(t) = line.counter_text.as_mut() {
                transform_link_tag(t);
            }
            line
        });

        Ok(())
    }
}

fn transform_link_tag(rich_text: &mut DocRichTextBlock) {
    if rich_text
        .tag
        .as_ref()
        .filter(|tag| tag == &prop::LINK)
        .is_none()
    {
        return;
    }
    if rich_text.link.is_some() {
        return;
    }

    if rich_text.text.starts_with('[') {
        match rich_text.text.find(']') {
            Some(i) => {
                rich_text.link = Some(rich_text.text[i + 1..].trim().to_string());
                rich_text.text = rich_text.text[1..i].to_string();
            }
            None => {
                rich_text.link = Some(rich_text.text.trim().to_string());
            }
        }
    } else {
        rich_text.link = Some(rich_text.text.trim().to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ignore_no_tag() {
        let mut text = DocRichText::text("hello world");
        let expected = text.clone();
        transform_link_tag(&mut text);
        assert_eq!(expected, text);
    }

    #[test]
    fn test_ignore_link() {
        let mut text = DocRichText {
            tag: Some("link".to_string()),
            text: "hello world".to_string(),
            link: Some("https://example.com".to_string()),
        };
        let expected = text.clone();
        transform_link_tag(&mut text);
        assert_eq!(expected, text);
    }

    #[test]
    fn test_ignore_non_link_tag() {
        let mut text = DocRichText {
            tag: Some("test".to_string()),
            text: "hello world".to_string(),
            link: None,
        };
        let expected = text.clone();
        transform_link_tag(&mut text);
        assert_eq!(expected, text);
    }

    #[test]
    fn test_transform_link_tag() {
        let mut text = DocRichText {
            tag: Some(prop::LINK.to_string()),
            text: "hello world".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichText {
                tag: Some(prop::LINK.to_string()),
                text: "hello world".to_string(),
                link: Some("hello world".to_string()),
            }
        );
    }

    #[test]
    fn test_transform_link_tag_with_text() {
        let mut text = DocRichText {
            tag: Some(prop::LINK.to_string()),
            text: "[hello world] i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichText {
                tag: Some(prop::LINK.to_string()),
                text: "hello world".to_string(),
                // link should be trimmed
                link: Some("i am link".to_string()),
            }
        );
    }

    #[test]
    fn test_transform_partial_bracket() {
        let mut text = DocRichText {
            tag: Some(prop::LINK.to_string()),
            text: "[hello world i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichText {
                tag: Some(prop::LINK.to_string()),
                text: "[hello world i am link".to_string(),
                link: Some("[hello world i am link".to_string()),
            }
        );

        let mut text = DocRichText {
            tag: Some(prop::LINK.to_string()),
            text: "abc[hello world] i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichText {
                tag: Some(prop::LINK.to_string()),
                text: "abc[hello world] i am link".to_string(),
                link: Some("abc[hello world] i am link".to_string()),
            }
        );

        let mut text = DocRichText {
            tag: Some(prop::LINK.to_string()),
            text: "abchello world] i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichText {
                tag: Some(prop::LINK.to_string()),
                text: "abchello world] i am link".to_string(),
                link: Some("abchello world] i am link".to_string()),
            }
        );
    }
}
