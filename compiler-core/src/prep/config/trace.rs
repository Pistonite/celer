use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// Data to trace location of a config object to help with debugging.
#[derive(Default, Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct ConfigTrace(Vec<usize>);

impl Deref for ConfigTrace {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConfigTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for ConfigTrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "trace=[{}]",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl From<&[usize]> for ConfigTrace {
    fn from(v: &[usize]) -> Self {
        Self(v.to_vec())
    }
}
