use serde_json::Value;

/// Loosely interpret a json value as a type
pub trait Coerce: Sized {
    /// Check if self is null
    fn is_null(&self) -> bool;

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

    /// Like `coerce_to_string`, but avoids copying if the value is a string
    fn coerce_into_string(self) -> String;

    /// Same as `coerce_to_string`, but `null` is `"null"` instead of empty string.
    fn coerce_to_repl(&self) -> String {
        if self.is_null() {
            "null".to_string()
        } else {
            self.coerce_to_string()
        }
    }

    /// Like `coerce_to_repl`, but avoids copying if the value is a string
    fn coerce_into_repl(self) -> String {
        if self.is_null() {
            "null".to_string()
        } else {
            self.coerce_into_string()
        }
    }

    /// Interpret a json value as a boolean based on if it's truthy
    ///
    /// Returns true for `true`, non-zero numbers, non-empty strings, arrays, and objects.
    fn coerce_truthy(&self) -> bool;

    /// Interpret a number or string as f64
    fn try_coerce_to_f64(&self) -> Option<f64>;

    /// Interpret a number or string as u64
    fn try_coerce_to_u64(&self) -> Option<u64>;

    /// Interpret a number or string as u32
    fn try_coerce_to_u32(&self) -> Option<u32>;

    /// Interpret a number or string as i64
    fn try_coerce_to_i64(&self) -> Option<i64>;

    /// Interpret a null, number (0 or 1), boolean, or string ("true" or "false") as bool
    fn try_coerce_to_bool(&self) -> Option<bool>;
}

impl Coerce for Value {
    fn is_null(&self) -> bool {
        Value::is_null(self)
    }

    fn coerce_to_string(&self) -> String {
        match self {
            Value::Null => "".to_string(),
            Value::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
            Value::Array(_) => "[object array]".to_string(),
            Value::Object(_) => "[object object]".to_string(),
        }
    }

    fn coerce_into_string(self) -> String {
        match self {
            Value::String(s) => s,
            _ => self.coerce_to_string(),
        }
    }

    fn coerce_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_f64().map(|x| x != 0.0).unwrap_or(false),
            Value::String(s) => !s.is_empty(),
            Value::Array(_) => true,
            Value::Object(_) => true,
        }
    }

    fn try_coerce_to_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.trim().parse::<f64>().ok(),
            _ => None,
        }
    }

    fn try_coerce_to_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.as_u64(),
            Value::String(s) => s.trim().parse::<u64>().ok(),
            _ => None,
        }
    }

    fn try_coerce_to_u32(&self) -> Option<u32> {
        let x = self.try_coerce_to_u64()?;
        if x > u32::MAX as u64 {
            None
        } else {
            Some(x as u32)
        }
    }

    fn try_coerce_to_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => s.trim().parse::<i64>().ok(),
            _ => None,
        }
    }

    fn try_coerce_to_bool(&self) -> Option<bool> {
        match self {
            Value::Null => Some(false),
            Value::Bool(b) => Some(*b),
            Value::Number(n) => n.as_f64().and_then(|x| {
                if x == 0.0 {
                    Some(false)
                } else if x == 1.0 {
                    Some(true)
                } else {
                    None
                }
            }),
            Value::String(s) => match s.trim() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_f64() {
        assert_eq!(Some(1.0), json!("1.0").try_coerce_to_f64());
        assert_eq!(Some(13.0), json!("13").try_coerce_to_f64());
        assert_eq!(Some(13.0), json!(" 13 ").try_coerce_to_f64());
        assert_eq!(None, json!("").try_coerce_to_f64());
        assert_eq!(None, json!(null).try_coerce_to_f64());
        assert_eq!(None, json!(true).try_coerce_to_f64());
        assert_eq!(None, json!(false).try_coerce_to_f64());
        assert_eq!(Some(13.0), json!(13).try_coerce_to_f64());
        assert_eq!(Some(13.0), json!(13.0).try_coerce_to_f64());
        assert_eq!(None, json!([]).try_coerce_to_f64());
        assert_eq!(None, json!({}).try_coerce_to_f64());
    }

    #[test]
    fn test_to_bool() {
        assert_eq!(None, json!("1.0").try_coerce_to_bool());
        assert_eq!(None, json!("0").try_coerce_to_bool());
        assert_eq!(None, json!("").try_coerce_to_bool());
        assert_eq!(Some(true), json!("true").try_coerce_to_bool());
        assert_eq!(Some(true), json!(" true ").try_coerce_to_bool());
        assert_eq!(Some(false), json!("false").try_coerce_to_bool());
        assert_eq!(Some(false), json!(null).try_coerce_to_bool());
        assert_eq!(Some(true), json!(true).try_coerce_to_bool());
        assert_eq!(Some(false), json!(false).try_coerce_to_bool());
        assert_eq!(None, json!(13).try_coerce_to_bool());
        assert_eq!(Some(false), json!(0).try_coerce_to_bool());
        assert_eq!(Some(true), json!(1.0).try_coerce_to_bool());
        assert_eq!(None, json!([]).try_coerce_to_bool());
        assert_eq!(None, json!({}).try_coerce_to_bool());
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test_truthy() {
        assert_eq!(true, json!("1.0").coerce_truthy());
        assert_eq!(true, json!(1.0).coerce_truthy());
        assert_eq!(true, json!(1).coerce_truthy());
        assert_eq!(false, json!(0).coerce_truthy());
        assert_eq!(false, json!(0.0).coerce_truthy());
        assert_eq!(false, json!(false).coerce_truthy());
        assert_eq!(true, json!(true).coerce_truthy());
        assert_eq!(true, json!("hello").coerce_truthy());
        assert_eq!(false, json!("").coerce_truthy());
        assert_eq!(true, json!([]).coerce_truthy());
        assert_eq!(false, json!(null).coerce_truthy());
        assert_eq!(true, json!({}).coerce_truthy());
    }
}
