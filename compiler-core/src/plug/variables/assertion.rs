//! Assertion plugin

use serde_json::Value;

use super::Operand;

#[derive(Debug, Clone, Default)]
pub struct AssertionPlugin {

}

struct Assertion {
    pub checks: Vec<(String, Cmp)>,
    pub msg_tyle: String,
    pub message: String,
}

enum Cmp {
    Eq(Operand<'static>),
    Ne(Operand<'static>),
    Gt(Operand<'static>),
    Lt(Operand<'static>),
    Ge(Operand<'static>),
    Le(Operand<'static>),
}
