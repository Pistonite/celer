use std::sync::OnceLock;

static SITE_ORIGIN: OnceLock<String> = OnceLock::new();

/// Set the site origin globally if not already set
pub fn init_site_origin(origin: String) -> Result<(), String> {
    SITE_ORIGIN.set(origin)
}

/// Get the site origin, or default to empty string
pub fn get_site_origin() -> &'static str {
    match SITE_ORIGIN.get() {
        Some(origin) => origin,
        None => "",
    }
}

/// Get the site domain (origin without url scheme)
pub fn get_site_domain() -> &'static str {
    let origin = get_site_origin();
    match origin.strip_prefix("https://") {
        Some(domain) => domain,
        None => origin.strip_prefix("http://").unwrap_or(origin),
    }
}
