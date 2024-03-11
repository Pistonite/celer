//! Environment setup or things that depend on the execution environment (server vs WASM).

use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;

use crate::macros::late_global;
use crate::res::Loader;

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

/// Global loader instance that can be used to load resources
/// outside of the usual compilation cycle. For example, in plugins
#[late_global(dyn Loader)]
pub mod global_loader {}

impl<T> Deref for RefCounted<T>
where
    T: ?Sized,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Display for RefCounted<T>
where
    T: Display + ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
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
