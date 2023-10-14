//TODO #78: This will be made a runtime that wraps tokio

use std::future::Future;

pub struct Runtime {}

impl Runtime {
    /// Create and initialize the runtime
    pub fn create() -> Self {
        Self {}
    }

    pub fn run_until<F, FOutput>(&mut self, f: F) -> impl Future<Output = FOutput>
    where
        F: Future<Output = FOutput>,
    {
        f
    }
}
