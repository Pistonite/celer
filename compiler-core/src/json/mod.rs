//! Utility for interacting with json data

// mod loose_str;

use serde_json::Value;

/// Loosely interpret a json value as a type
pub trait Coerce {
    /// Interpret a json value as a string, without recursively expanding array or object
    ///
    /// # Rules
    /// `Null` -> `""`
    /// `Bool` -> `"true"` or `"false"`
    /// `Number` -> string representation of number
    /// `String` -> string
    /// `Array` -> `[object array]`
    /// `Object` -> `[object object]`
    fn coerce_to_string(&self) -> String;

}

impl Coerce for Value {
    fn coerce_to_string(&self) -> String {
        match self {
            Value::Null => "".to_string(),
            Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
            Value::Array(_) => "[object array]".to_string(),
            Value::Object(_) => "[object object]".to_string(),
        }
    }

}

