//! Link plugin
//!
//! This plugin looks for the `link` tag and transforms it into a link.

use std::borrow::Cow;

use crate::env::yield_budget;
use crate::comp::CompDoc;
use crate::lang::DocRichTextBlock;
use crate::pack::CompileContext;
use crate::prep::{DocTag, DocTagColor};
use crate::prop;
use crate::macros::async_trait;

use crate::plugin::{PluginResult, PluginRuntime};

pub struct LinkPlugin;

#[async_trait(auto)]
impl PluginRuntime for LinkPlugin {
    async fn on_before_compile<'p>(&mut self, ctx: &mut CompileContext<'p>) -> PluginResult<()> {
        // add the link tag if not defined already
        let link_tag = DocTag {
            color: Some(DocTagColor::LightDark {
                light: Some("var(--link-text-color-light)".to_string()),
                dark: Some("var(--link-text-color-dark)".to_string()),
            }),
            background: Some(DocTagColor::LightDark {
                light: Some("var(--link-text-background-light)".to_string()),
                dark: Some("var(--link-text-background-dark)".to_string()),
            }),
            ..Default::default()
        };
        ctx.config
            .to_mut()
            .tags
            .entry(prop::LINK.to_string())
            .and_modify(|tag| tag.apply_to_default(&link_tag))
            .or_insert(link_tag);
        Ok(())
    }

    async fn on_after_compile<'p>(&mut self, comp_doc: &mut CompDoc<'p>) -> PluginResult<()> {
        for tag in comp_doc.rich_texts_mut().with_preface().with_counter() {
            yield_budget(256).await;
            transform_link_tag(tag);
        }

        Ok(())
    }

    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::BuiltInPlugin::Link.id())
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
        let mut text = DocRichTextBlock::text("hello world");
        let expected = text.clone();
        transform_link_tag(&mut text);
        assert_eq!(expected, text);
    }

    #[test]
    fn test_ignore_link() {
        let mut text = DocRichTextBlock {
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
        let mut text = DocRichTextBlock {
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
        let mut text = DocRichTextBlock {
            tag: Some(prop::LINK.to_string()),
            text: "hello world".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichTextBlock {
                tag: Some(prop::LINK.to_string()),
                text: "hello world".to_string(),
                link: Some("hello world".to_string()),
            }
        );
    }

    #[test]
    fn test_transform_link_tag_with_text() {
        let mut text = DocRichTextBlock {
            tag: Some(prop::LINK.to_string()),
            text: "[hello world] i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichTextBlock {
                tag: Some(prop::LINK.to_string()),
                text: "hello world".to_string(),
                // link should be trimmed
                link: Some("i am link".to_string()),
            }
        );
    }

    #[test]
    fn test_transform_partial_bracket() {
        let mut text = DocRichTextBlock {
            tag: Some(prop::LINK.to_string()),
            text: "[hello world i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichTextBlock {
                tag: Some(prop::LINK.to_string()),
                text: "[hello world i am link".to_string(),
                link: Some("[hello world i am link".to_string()),
            }
        );

        let mut text = DocRichTextBlock {
            tag: Some(prop::LINK.to_string()),
            text: "abc[hello world] i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichTextBlock {
                tag: Some(prop::LINK.to_string()),
                text: "abc[hello world] i am link".to_string(),
                link: Some("abc[hello world] i am link".to_string()),
            }
        );

        let mut text = DocRichTextBlock {
            tag: Some(prop::LINK.to_string()),
            text: "abchello world] i am link".to_string(),
            link: None,
        };
        transform_link_tag(&mut text);
        assert_eq!(
            text,
            DocRichTextBlock {
                tag: Some(prop::LINK.to_string()),
                text: "abchello world] i am link".to_string(),
                link: Some("abchello world] i am link".to_string()),
            }
        );
    }
}
