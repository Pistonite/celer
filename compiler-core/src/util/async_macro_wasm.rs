use std::cell::UnsafeCell;

use js_sys::Promise;
use log::warn;
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

thread_local! {
    static WINDOW: Window = web_sys::window().expect("no global `window` exists");
}

thread_local! {
    static CANCELLED: UnsafeCell<bool> = UnsafeCell::new(false);
}

/// A signal for cancellation
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum WasmError {
    #[error("cancelled")]
    Cancel,
}

pub fn cancel() {
    CANCELLED.with(|cancelled| {
        unsafe {
            *cancelled.get() = true;
        }
    });
}

/// Yield control to the browser with window.setTimeout
///
/// Shared code for server and WASM should use the [`yield_now`] macro instead of calling this directly.
pub async fn set_timeout_yield() -> Result<(), WasmError> {
    let promise = WINDOW.with(|window| {
        Promise::new(&mut |resolve, _| {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                0,
            );
        })
    });
    let _ = JsFuture::from(promise).await;
    CANCELLED.with(|cancelled| {
        unsafe {
            if *cancelled.get() {
                warn!("cancelling...");
                Err(WasmError::Cancel)
            } else {
                Ok(())
            }
        }
    })
}

/// Async iterator wrapper in WASM
///
/// Each iteration will call window.__yield() to yield control back to the browser
macro_rules! async_for {
    ($v:pat in $iter:expr, $body:stmt) => {
        {
            let mut result = Ok(());
            for $v in $iter {
                result = $crate::util::async_macro_wasm::set_timeout_yield().await;
                if result.is_err() {
                    break;
                }
                $body
            }
            result
        }
    };
}
pub(crate) use async_for;

macro_rules! yield_now {
    () => {
        $crate::util::async_macro_wasm::set_timeout_yield().await
    };
}
pub(crate) use yield_now;
