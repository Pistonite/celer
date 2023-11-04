mod path;
pub use path::*;

// #[cfg(not(feature = "wasm"))]
// mod async_macro;
// #[cfg(not(feature = "wasm"))]
// pub(crate) use async_macro::*;
// #[cfg(feature = "wasm")]
// pub mod async_macro_wasm;
// #[cfg(feature = "wasm")]
// pub(crate) use async_macro_wasm::*;

mod site_origin;
pub use site_origin::*;

mod coop;
pub use coop::*;

/// Maybe Arc. Will be Rc if no-async-send feature is enabled
#[cfg(not(feature = "no-async-send"))]
pub type Marc<T> = std::sync::Arc<T>;
#[cfg(feature = "no-async-send")]
pub type Marc<T> = std::rc::Rc<T>;
