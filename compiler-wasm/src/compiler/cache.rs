
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use celerc::lang::DocDiagnostic;
use instant::Instant;
use log::{error, info};
use wasm_bindgen::prelude::*;

use celerc::pack::PackError;
use celerc::{
    CompDoc, CompileContext, Compiler, ContextBuilder, ExecContext, PluginOptions, PreparedContext,
};

use crate::interop::OpaqueExpoContext;
use crate::loader::{self, LoadFileOutput, LoaderInWasm};
use crate::plugin;

thread_local! {
    static CACHED_COMPILER_CONTEXT: RefCell<Option<PreparedContext<LoaderInWasm>>> = RefCell::new(None);
}

/// Guard for acquiring the cached context and takes care of releasing it
pub struct CachedContextGuard(Option<PreparedContext<LoaderInWasm>>);
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
            },
            None => None,
        }
    }

    /// Put a new context into the cache upon drop
    pub fn new(prep_ctx: PreparedContext<LoaderInWasm>) -> Self {
        CachedContextGuard(Some(prep_ctx))
    }
}
impl Drop for CachedContextGuard {
    fn drop(&mut self) {
        CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| *x = self.0.take());
    }
}
impl AsRef<PreparedContext<LoaderInWasm>> for CachedContextGuard {
    fn as_ref(&self) -> &PreparedContext<LoaderInWasm> {
        self.0.as_ref().unwrap()
    }
}

// pub async fn is_cache_valid(entry_path: Option<&String>) -> bool {
//     let root_project_result = loader::load_file_check_changed("project.yaml").await;
//     if !matches!(root_project_result, Ok(LoadFileOutput::NotModified)) {
//         info!("root project.yaml is modified");
//         return false;
//     }
//     if let Some(entry_path) = entry_path {
//         let entry_path = match entry_path.strip_prefix('/') {
//             Some(x) => x,
//             None => entry_path,
//         };
//         let entry_result = loader::load_file_check_changed(entry_path).await;
//         if !matches!(entry_result, Ok(LoadFileOutput::NotModified)) {
//             info!("entry project.yaml is modified");
//             return false;
//         }
//     }
//     let is_same = CACHED_COMPILER_ENTRY_PATH.with_borrow(|x| x.as_ref() == entry_path);
//     if !is_same {
//         info!("entry changed");
//         return false;
//     }
// }

