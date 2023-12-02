mod path;
pub use path::*;
mod coop;
pub use coop::*;
mod site_origin;
pub use site_origin::*;

/// Maybe Arc. Will be Rc for wasm where multi-threading is not needed.
#[cfg(not(feature = "wasm"))]
pub type Marc<T> = std::sync::Arc<T>;
#[cfg(feature = "wasm")]
pub type Marc<T> = std::rc::Rc<T>;
