mod path;
pub use path::*;
mod coop;
pub use coop::*;
mod site_origin;
pub use site_origin::*;

/// Maybe Arc. Will be Rc if no-async-send feature is enabled
#[cfg(not(feature = "no-async-send"))]
pub type Marc<T> = std::sync::Arc<T>;
#[cfg(feature = "no-async-send")]
pub type Marc<T> = std::rc::Rc<T>;
