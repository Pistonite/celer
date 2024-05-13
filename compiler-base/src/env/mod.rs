//! Environment setup or things that depend on the execution environment (server vs WASM).

use std::cell::RefCell;

use crate::macros::late_global;
use crate::res::LoaderFactory;

#[cfg(feature = "wasm")]
pub mod env_wasm;
#[cfg(feature = "wasm")]
pub use env_wasm::*;

#[cfg(not(feature = "wasm"))]
pub mod env_native;
#[cfg(not(feature = "wasm"))]
pub use env_native::*;

/// Site origin global configuration
#[late_global(str)]
pub mod site {
    #[inline]
    pub fn set_origin(origin: &str) {
        let _ = set(RefCounted::from(origin));
    }

    #[inline]
    pub fn get_origin() -> RefCounted<str> {
        match get() {
            Some(origin) => origin,
            None => RefCounted::from(""),
        }
    }

    /// Get the site domain (origin without url scheme)
    pub fn get_domain() -> RefCounted<str> {
        let origin = get_origin();
        match origin.strip_prefix("https://") {
            Some(domain) => RefCounted::from(domain),
            None => match origin.strip_prefix("http://") {
                Some(domain) => RefCounted::from(domain),
                None => origin,
            },
        }
    }
}

/// Factory for getting resource loader instance that can be used to load resources
/// outside of the usual compilation cycle. For example, in plugins
#[late_global(dyn LoaderFactory)]
pub mod global_loader_factory {}

thread_local! {
    /// Current number of ticks ran without yielding in cooperative multitasking
    static COOP_TICKS: RefCell<u32> = const { RefCell::new(0) };
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
