use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// Behaves like `await` in JS to Provides interoperability with JS promises. Uses
/// `wasm-bindgen-futures` under the hood
///
/// Need to import the `JsIntoFuture` trait to use the `into_future` conversion,
/// since Rust doesn't allow implementing foreign traits for foreign types.
/// # Example
/// ```ignore
/// // Get a promise from somewhere as a JsValue
/// let my_promise: JsValue = /* ... */;
/// // Await on it
/// let result: Result<JsValue, JsValue> = my_promise.into_future().await;
/// ```
pub enum JsAwait {
    Promise(JsFuture),
    Value(JsValue),
}

pub trait JsIntoFuture {
    fn into_future(self) -> JsAwait;
}

impl From<JsValue> for JsAwait {
    fn from(value: JsValue) -> Self {
        let promise: Result<Promise, JsValue> = value.dyn_into();
        match promise {
            Ok(promise) => Self::Promise(JsFuture::from(promise)),
            Err(value) => Self::Value(value),
        }
    }
}

impl JsIntoFuture for JsValue {
    fn into_future(self) -> JsAwait {
        self.into()
    }
}

impl Future for JsAwait {
    type Output = Result<JsValue, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Self::Value(value) => {
                let mut taken = JsValue::UNDEFINED.clone();
                std::mem::swap(value, &mut taken);
                Poll::Ready(Ok(taken))
            }
            Self::Promise(future) => {
                Pin::new(future).poll(cx)
            }
        }
    }

}
