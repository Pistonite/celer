use serde::{Deserialize, Serialize};

use crate::comp::CompError;
use crate::json::{Cast, Coerce, SafeRouteBlob};
use crate::lang;
use crate::lang::DocRichText;
use crate::macros::derive_wasm;

use super::LineContext;

/// Document note block
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[serde(tag = "type")]
pub enum DocNote {
    Text { content: DocRichText },
    Image { link: String },
    Video { link: String },
}

impl DocNote {
    /// Create a text note, parsing the string as rich text
    pub fn text(s: &str) -> Self {
        Self::Text {
            content: lang::parse_rich(s),
        }
    }
}

impl<'c, 'p> LineContext<'c, 'p> {
    /// Compile a note block and add to self
    pub fn compile_note(&mut self, prop_name: &str, value: SafeRouteBlob<'_>) {
        let value = match value.try_into_array() {
            Err(value) => value,
            Ok(_) => {
                self.errors
                    .push(CompError::InvalidLinePropertyType(prop_name.to_string()));
                return;
            }
        };
        let value = match value.try_into_object() {
            Err(value) => value,
            Ok(_) => {
                self.errors
                    .push(CompError::InvalidLinePropertyType(prop_name.to_string()));
                return;
            }
        };
        // TODO #174: image and video
        let note = DocNote::text(&value.coerce_into_string());
        self.line.notes.push(note);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::{json, Value};

    use crate::json::IntoSafeRouteBlob;
    use crate::pack::Compiler;

    impl<'c, 'p> LineContext<'c, 'p> {
        pub fn test_compile_note(&mut self, value: Value) {
            self.compile_note("test", value.into_unchecked())
        }

        pub fn assert_notes(&self, expected: DocNote) {
            assert_eq!(self.line.notes.len(), 1);
            assert_eq!(self.line.notes.first().unwrap(), &expected);
        }
    }

    #[test]
    pub fn test_primitive() {
        let compiler = Compiler::default();
        let mut ctx = LineContext::with_compiler(&compiler);
        ctx.test_compile_note(json!(1));
        ctx.assert_notes(DocNote::text("1"));
        ctx.line.notes.clear();

        ctx.test_compile_note(json!(null));
        ctx.assert_notes(DocNote::text(""));
        ctx.line.notes.clear();

        ctx.test_compile_note(json!(true));
        ctx.assert_notes(DocNote::text("true"));
        ctx.line.notes.clear();

        ctx.test_compile_note(json!(false));
        ctx.assert_notes(DocNote::text("false"));
        ctx.line.notes.clear();

        ctx.test_compile_note(json!("hello"));
        ctx.assert_notes(DocNote::text("hello"));
        ctx.line.notes.clear();

        ctx.test_compile_note(json!(""));
        ctx.assert_notes(DocNote::text(""));
        ctx.line.notes.clear();
    }

    #[test]
    pub fn test_invalid_type() {
        let compiler = Compiler::default();
        let mut ctx = LineContext::with_compiler(&compiler);
        ctx.test_compile_note(json!([]));
        ctx.assert_notes(DocNote::text("[object array]"));
        assert_eq!(
            ctx.errors,
            vec![CompError::InvalidLinePropertyType("test".to_string())]
        );
        ctx.line.notes.clear();
        ctx.errors.clear();

        ctx.test_compile_note(json!({}));
        ctx.assert_notes(DocNote::text("[object object]"));
        assert_eq!(
            ctx.errors,
            vec![CompError::InvalidLinePropertyType("test".to_string())]
        );
        ctx.line.notes.clear();
        ctx.errors.clear();
    }
}
