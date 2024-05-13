use std::cell::RefCell;

use log::info;

use celerc::PrepCtx;

use crate::loader::{self, LoadFileOutput, LoaderInWasm};

thread_local! {
    static CACHED_COMPILER_CONTEXT: RefCell<Option<PrepCtx<LoaderInWasm>>> = const { RefCell::new(None) };
}

/// Guard for acquiring the cached context and takes care of releasing it
pub struct CachedContextGuard(Option<PrepCtx<LoaderInWasm>>);
impl CachedContextGuard {
    /// Acquire the cached context if it's valid
    pub async fn acquire(entry_path: Option<&String>) -> Option<Self> {
        match CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| x.take()) {
            Some(prep_ctx) => {
                // check if cache is still valid
                if prep_ctx.entry_path.as_ref() != entry_path {
                    info!("invalidating compiler cache because entry path changed");
                    return None;
                }
                // TODO #173: prep phase need to output local dependencies
                let mut dependencies = vec!["/project.yaml".to_string()];
                if let Some(entry_path) = entry_path {
                    dependencies.push(entry_path.clone());
                }
                for dep in &dependencies {
                    // strip leading slash if needed
                    let dep_path = match dep.strip_prefix('/') {
                        Some(x) => x,
                        None => dep,
                    };
                    let changed = loader::load_file_check_changed(dep_path).await;
                    if !matches!(changed, Ok(LoadFileOutput::NotModified)) {
                        info!("invalidating compiler cache because dependency changed: {dep}");
                        return None;
                    }
                }
                Some(CachedContextGuard(Some(prep_ctx)))
            }
            None => None,
        }
    }

    /// Put a new context into the cache upon drop
    pub fn new(prep_ctx: PrepCtx<LoaderInWasm>) -> Self {
        CachedContextGuard(Some(prep_ctx))
    }
}
impl Drop for CachedContextGuard {
    fn drop(&mut self) {
        CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| *x = self.0.take());
    }
}
impl AsRef<PrepCtx<LoaderInWasm>> for CachedContextGuard {
    fn as_ref(&self) -> &PrepCtx<LoaderInWasm> {
        self.0.as_ref().unwrap()
    }
}
