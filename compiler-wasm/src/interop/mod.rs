//! Interop with JS world

use js_sys::Function;

mod opaque;
pub use opaque::*;
mod promise;
pub use promise::*;

/// Create a stub JS function to fill in for a function slot that is not yet initialized
pub fn stub_function() -> Function {
    Function::new_no_args("throw new Error(\"not initialized\")")
}
