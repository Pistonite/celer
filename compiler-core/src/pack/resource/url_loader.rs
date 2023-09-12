use cached::proc_macro::cached;

use crate::pack::{PackerResult, PackerError};

/// Load a resource from URL. Result is automatically cached
#[cached(
    size=512,
    key="String",
    convert = r#"{ url.to_string() }"#,
    // TTL of 5 minutes
    time=300,
)]
pub async fn load_resource_from_url(url: &str) -> PackerResult<Vec<u8>> {
    Err(PackerError::NotImpl(
        "load_resource_from_url is not implemented yet.".to_string(),
    ))
}
