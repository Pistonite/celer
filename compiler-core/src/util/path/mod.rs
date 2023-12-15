// //! A platform-independent path implementation
//
// use std::fmt::Display;
//
// mod path_from;
// pub use path_from::*;
// mod path_join;
// pub use path_join::*;
//
// #[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
// pub struct Path(String);
//
// impl AsRef<str> for Path {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }
//
// impl Display for Path {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }
