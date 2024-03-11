//! Conversion between data url and bytes

use std::borrow::Cow;

use base64::Engine;

pub fn to_data_url_base64(mime: &str, data: &[u8]) -> String {
    let mut data_url = format!("data:{mime};base64,");
    base64::engine::general_purpose::STANDARD.encode_string(data, &mut data_url);
    data_url
}

#[derive(Debug, thiserror::Error)]
pub enum DataUrlParseError {
    #[error("Data url should have `data:` prefix")]
    InvalidPrefix,
    #[error("Cannot determine data type from data url")]
    InvalidType,
    #[error("Cannot find data in data url")]
    NoData,
    #[error("Error decoding base64 from data url: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    
}

/// Decode data url to bytes. Supports base64 and URL encoding.
pub fn bytes_from_data_url(data_url: &str) -> Result<Cow<[u8]>, DataUrlParseError> {
    let data = match data_url.strip_prefix("data:") {
        Some(data) => data,
        None => return Err(DataUrlParseError::InvalidPrefix),
    };
    let mut parts = data.splitn(2, ',');
    let type_and_encoding = parts.next().ok_or(DataUrlParseError::InvalidType)?;
    let data = parts.next().ok_or(DataUrlParseError::NoData)?;
    if type_and_encoding.ends_with(";base64") {
        let data = base64::engine::general_purpose::STANDARD.decode(data)?;
        return Ok(Cow::Owned(data));
    }

    let data = urlencoding::decode_binary(data.as_bytes());
    Ok(data)
}
