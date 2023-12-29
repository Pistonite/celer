//! WASM environment implementation

use std::{rc::Rc, future::Future};

/// Ref counted pointer. Wrapper for Rc
#[derive(Debug)]
#[repr(transparent)]
pub struct RefCounted<T>
where
    T: ?Sized,
{
    pub(crate) inner: Rc<T>,
}

impl<T> Clone for RefCounted<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> RefCounted<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl From<&str> for RefCounted<str> {
    #[inline]
    fn from(s: &str) -> Self {
        Self { inner: Rc::from(s) }
    }
}

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

            let global_self: WorkerGlobalScope =
                Reflect::get(&global(), &JsValue::from("self"))?.dyn_into()?;
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

/// Wait for multiple futures to complete and collect their results
/// in the same order
///
/// On WASM, futures are simply executed sequentially
pub async fn iter_futures<I, T>(budget: u32, iter: I) -> Vec<T>
where
    I: IntoIterator,
    I::Item: Future<Output = T>,
{
    let mut results = Vec::new();
    for future in iter {
        yield_budget(budget).await;
        results.push(future.await);
    }
    results
}
