use std::cell::UnsafeCell;

// use js_sys::Promise;
use log::warn;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::WorkerGlobalScope;

thread_local! {
    static CANCELLED: UnsafeCell<bool> = UnsafeCell::new(false);
}

const BUDGET_MAX: u8 = 64;
thread_local! {
    static BUDGET: UnsafeCell<u8> = UnsafeCell::new(BUDGET_MAX);
}

/// A signal for cancellation
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum WasmError {
    #[error("cancelled")]
    Cancel,
}

pub fn set_cancelled(value: bool) {
    CANCELLED.with(|cancelled| unsafe {
        *cancelled.get() = value;
    });
}

/// Yield control to the browser with window.setTimeout
///
/// Shared code for server and WASM should use the [`yield_now`] macro instead of calling this directly.
pub async fn set_timeout_yield() -> Result<(), WasmError> {
    let has_budget = BUDGET.with(|budget| unsafe {
        let b_ref = budget.get();
        if *b_ref == 0 {
            *b_ref = BUDGET_MAX;
            false
        } else {
            *b_ref -= 1;
            true
        }
    });
    if has_budget {
        return Ok(());
    }
    // Promise::new(&mut |resolve, _| {
    //     web_sys::worker_global_scope();
    //     let _ = WorkerGlobalScope::self_().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0);
    // });
    // let _ = JsFuture::from(promise).await;
    CANCELLED.with(|cancelled| unsafe {
        if *cancelled.get() {
            warn!("cancelling...");
            Err(WasmError::Cancel)
        } else {
            Ok(())
        }
    })
}

/// Async iterator wrapper in WASM
///
/// Each iteration will call window.__yield() to yield control back to the browser
macro_rules! async_for {
    ($v:pat in $iter:expr, $body:stmt) => {{
        let mut result: Result<(), $crate::util::async_macro_wasm::WasmError> = Ok(());
        for $v in $iter {
            result = $crate::util::async_macro_wasm::set_timeout_yield().await;
            if result.is_err() {
                break;
            }
            $body
        }
        result
    }};
}
pub(crate) use async_for;

#[macro_export]
macro_rules! yield_now {
    () => {
        $crate::util::async_macro_wasm::set_timeout_yield().await
    };
}
