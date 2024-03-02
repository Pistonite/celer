//! Utilities for parsing headers

use axum::http::{HeaderMap, HeaderValue};
use base64::Engine;
use tracing::error;

use celerc::ExportRequest;

/// Get the Celer-Plugin-Options header as a string
/// Returns empty string if header is not present or empty
pub fn get_plugin_options(headers: &HeaderMap) -> Result<String, String> {
    let header_value = match headers.get("Celer-Plugin-Options") {
        None => return Ok(String::new()),
        Some(v) => v,
    };

    let header_str = decode_base64_header_utf8(header_value)?;

    Ok(header_str)
}

pub fn get_export_request(headers: &HeaderMap) -> Result<ExportRequest, String> {
    let header_value = match headers.get("Celer-Export-Request") {
        None => {
            error!("Missing required header");
            return Err("Missing required header".to_string());
        }
        Some(v) => v,
    };

    let header_str = decode_base64_header_utf8(header_value)?;
    let req: ExportRequest = match serde_json::from_str(&header_str) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse header value as JSON: {e}");
            return Err("Invalid header value".to_string());
        }
    };

    Ok(req)
}

/// Decode a base64 encoded header value as UTF-8. Returns error message if decoding fails.
fn decode_base64_header_utf8(header_value: &HeaderValue) -> Result<String, String> {
    let value = match header_value.to_str() {
        Ok(s) => s,
        Err(e) => {
            error!("Raw header value is not valid UTF-8: {e}");
            return Err("Invalid header encoding".to_string());
        }
    };
    if value.is_empty() {
        return Ok(String::new());
    }
    let decoded = match base64::engine::general_purpose::STANDARD.decode(value) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to decode header value from base64: {e}");
            return Err("Invalid header encoding".to_string());
        }
    };
    let header_str = match String::from_utf8(decoded) {
        Ok(s) => s,
        Err(e) => {
            error!("header value is not valid UTF-8: {e}");
            return Err("Invalid header encoding".to_string());
        }
    };
    Ok(header_str)
}
