//! Template string
use serde::{Deserialize, Serialize};

mod grammar;
mod parse;
use parse::*;
mod hydrate;

/// A template string
///
/// Template string can have variables in it represented by $(number),
/// where number is the index of the variable in the list of variables (starts from 0)
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct TempStr(Vec<TempStrBlock>);

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TempStrBlock {
    Lit(String),
    Var(usize),
}

impl TempStr {
    /// Return if the template string has no variables
    pub fn is_literal(&self) -> bool {
        if self.0.len() == 1 {
            return matches!(self.0[0], TempStrBlock::Lit(_));
        }
        self.0.is_empty()
    }
}
