//! WASM environment implementation

use std::rc::Rc;

/// Ref counted pointer. Wrapper for Rc
pub type RefCounted<T> = Rc<T>;

pub async fn yield_budget(limit: u32) {
    // on wasm we don't need to yield too often
    // multiply the limit by 4 to reduce the number of times we need to yield
    if super::coop_tick_increment(limit * 4) {
        // if yield fails, we'll just continue
        let _ = async {
            use js_sys::{global, Promise, Reflect};
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::WorkerGlobalScope;

            let self_value = JsValue::from("self");
            let global_obj = global();
            let global_self: WorkerGlobalScope =
                Reflect::get(&global_obj, &self_value)?.dyn_into()?;
            let promise = Promise::new(&mut |resolve, _| {
                let _ =
                    global_self.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0);
            });
            JsFuture::from(promise).await?;

            Ok::<(), JsValue>(())
        }
        .await;
    }
}

/// Wait for multiple futures to complete
#[macro_export]
macro_rules! join_futures {
    ($($e:expr),* $(,)?) => {
        // on wasm, since there is only one thread
        // we will just run the futures sequentially
        (
            $(
                $e.await,
            )*
        )
    };
}
pub use join_futures;

/// Spawn futures and collect the results in a vec in the same order
pub async fn join_future_vec<TFuture>(v: Vec<TFuture>) -> Vec<Result<TFuture::Output, String>>
where
    TFuture: std::future::Future,
{
    // on wasm, since there is only one thread
    // we will just run the futures sequentially
    let mut results = Vec::with_capacity(v.len());
    for f in v {
        results.push(Ok(f.await));
    }
    results
}
