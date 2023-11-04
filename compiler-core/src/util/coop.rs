//! Utils for the compiler to run co-operatively. i.e. allow other tasks to run concurrently
//! alongside a compilation on the same thread.

use std::{sync::OnceLock, cell::RefCell};

use log::info;

thread_local! {
    /// Current number of ticks ran without yielding
    static TICKS: RefCell<u32> = RefCell::new(0);
}

/// Yield control to the runtime or worker if the ticks are over the budget limit.
pub async fn yield_budget(budget_limit: u32) {
    // on wasm we don't need to yield too often
    #[cfg(feature = "wasm")]
    let budget_limit = budget_limit * 4;
    let has_budget = TICKS.with(|tick| {
        if *tick.borrow() > budget_limit {
            tick.replace(0);
            false
        } else {
            *tick.borrow_mut() += 1;
            true
        }
    });
    if !has_budget {
        #[cfg(feature = "wasm")]
        let _ = async {
            use wasm_bindgen_futures::JsFuture;
            use wasm_bindgen::prelude::*;
            use web_sys::WorkerGlobalScope;
            use js_sys::{Reflect, global, Promise};

            // info!("yielding to worker...");

            let global_self: WorkerGlobalScope = Reflect::get(&global(), &JsValue::from("self"))?.dyn_into()?;
            let promise = Promise::new(&mut |resolve, _| {
                let _ = global_self.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0);
            });
            JsFuture::from(promise).await?;

            Ok::<(), JsValue>(())
        }.await;
        #[cfg(not(feature = "wasm"))]
        {
            tokio::task::yield_now().await;
        }
    }

}
