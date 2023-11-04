//TODO #78: This will be made a runtime that wraps tokio

use std::cell::RefCell;
use std::future::Future;

use js_sys::Function;
use log::{info, error};
use tokio::runtime::Builder;
use tokio::task::{self, LocalSet};
use wasm_bindgen::prelude::*;

use crate::interop::{self, JsIntoFuture};
use crate::logger;

type TokioRuntime = tokio::runtime::Runtime;

thread_local! {
    /// Callback function to yield control to the worker (for handling other messages, etc)
    static YIELD_FN: RefCell<Function> = RefCell::new(interop::stub_function());
}

/// Bind the yield function
pub fn bind_yield(yield_fn: Function) {
    YIELD_FN.replace(yield_fn);
}

async fn yield_to_worker() -> Result<(), JsValue> {
    YIELD_FN.with_borrow(|f| f.call0(&JsValue::UNDEFINED))?
    .into_future().await?;

    Ok(())
}

pub struct Runtime {
    tokio_runtime: TokioRuntime,
}

impl Runtime {
    /// Create and initialize the runtime
    pub fn create() -> Self {
        let tokio_runtime = Builder::new_current_thread().build().expect("failed to initialize tokio runtime");
        tokio_runtime.spawn(async {
            let local_set = LocalSet::new();
            local_set.spawn_local(async {
                loop {
                    info!("yielding");
                    task::yield_now().await;
                    if let Err(e) = yield_to_worker().await {
                        error!("failed to yield to worker.");
                        logger::raw_error(&e);
                    }
                }
            });
        });

        Self { tokio_runtime }
    }

    pub fn run_until<F, FOutput>(&self, f: F) -> FOutput
    where
        F: Future<Output = FOutput>,
    {
        self.tokio_runtime.block_on(f)
    }
}
