use std::io;
use std::io::Write;

use base64::Engine;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::macros::derive_wasm;

/// The data in the export
#[derive(Debug, Clone)]
#[derive_wasm]
#[serde(tag = "type", content = "data")]
pub enum ExpoBlob {
    /// UTF-8 text
    Text(String),
    /// Binary data encoded in base64
    Base64(String),
    /// Binary data gzipped and encoded in base64
    Base64Gzip(String),
}

impl ExpoBlob {
    /// Create a blob from UTF-8 string
    pub fn from_utf8(s: String) -> Self {
        Self::Text(s)
    }

    /// Create a blob from binary data, uncompressed
    ///
    /// Useful if the binary data is small or is already compressed
    pub fn from_bytes(b: &[u8]) -> Self {
        let encoded = base64::engine::general_purpose::STANDARD.encode(b);
        Self::Base64(encoded)
    }

    /// Create a blob from binary data, compressed with gzip format
    pub fn from_bytes_gzipped(b: Vec<u8>) -> io::Result<Self> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&b)?;
        let bytes = encoder.finish()?;
        Ok(Self::from_bytes(&bytes))
    }
}
