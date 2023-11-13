//! Rich string
mod grammar;
mod ext;
pub use ext::RichTextExt;
mod parse;
pub use parse::parse_rich;
