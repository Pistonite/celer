//! Environment setup or things that depend on the execution environment (server vs WASM).

use std::cell::RefCell;
use std::sync::OnceLock;
use std::ops::Deref;

#[cfg(feature = "wasm")]
pub mod env_wasm;
#[cfg(feature = "wasm")]
pub use env_wasm::*;

#[cfg(not(feature = "wasm"))]
pub mod env_native;
#[cfg(not(feature = "wasm"))]
pub use env_native::*;

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

impl<T> Deref for RefCounted<T> where T: ?Sized{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

thread_local! {
    /// Current number of ticks ran without yielding in cooperative multitasking
    static COOP_TICKS: RefCell<u32> = RefCell::new(0);
}

/// Increment the ticks and return if the tick limit has been reached. If the limit
/// is reached, reset the tick to 0.
pub(crate) fn coop_tick_increment(limit: u32) -> bool {
    COOP_TICKS.with_borrow_mut(|ticks| {
        *ticks += 1;
        if *ticks > limit {
            *ticks = 0;
            true
        } else {
            false
        }
    })
}
