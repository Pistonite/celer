//! Known resource types.

use super::ResPath;

pub enum ResType {
    Yaml,
    Json,
    Png,
    Jpeg,
    Gif,
    Webp,
}

impl ResType {
    /// Get the media type of the resource (e.g. "application/json")
    pub fn media_type(&self) -> &'static str {
        match self {
            Self::Yaml => "application/x-yaml", // unofficial
            Self::Json => "application/json",
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
        }
    }

    /// Return if the resource is an image
    pub fn is_image(&self) -> bool {
        match self {
            Self::Png | Self::Jpeg | Self::Gif | Self::Webp => true,
            _ => false,
        }
    }
}

impl<'a> ResPath<'a> {
    /// Guess the resource type based on the extension
    pub fn get_type(&self) -> Option<ResType> {
        let ext = match self {
            Self::Local(path) => path.extension()?,
            Self::Remote(_, path) => path.extension()?,
        };
        if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") {
            return Some(ResType::Yaml);
        }
        if ext.eq_ignore_ascii_case("json") {
            return Some(ResType::Json);
        }
        if ext.eq_ignore_ascii_case("png") {
            return Some(ResType::Png);
        }
        if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") {
            return Some(ResType::Jpeg);
        }
        if ext.eq_ignore_ascii_case("gif") {
            return Some(ResType::Gif);
        }
        if ext.eq_ignore_ascii_case("webp") {
            return Some(ResType::Webp);
        }
        None
    }
}
