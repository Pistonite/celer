//! Utilities for variable text transformation

use std::collections::VecDeque;

pub fn transform_text_fn<FGet, FExact, FRound>(
    text: &str,
    fn_get: FGet,
    fn_exact: FExact,
    fn_round: FRound,
) -> Result<String, String>
where
    FGet: Fn(&str) -> f64,
    FExact: Fn(f64) -> String,
    FRound: Fn(f64) -> String,
{
    let mut iter = text.split(':').rev();
    let variable = match iter.next() {
        None => return Err("variable name cannot be empty".to_string()),
        Some(x) => x,
    };
    let value = fn_get(variable);
    let mut next_op = iter.next();
    let mut new_text = if next_op.is_some() {
        // If there is formatting needed, always round
        fn_round(value)
    } else {
        fn_exact(value)
    }.chars().collect::<VecDeque<_>>();

    while let Some(op) = next_op {
        if let Some(x) = op.strip_prefix("pad") {
            let mut iter = x.chars();
            let pad = iter
                .next()
                .ok_or("`pad` must be followed by the character to pad")?;
            let width = iter.collect::<String>().parse::<usize>().map_err(|_| {
                format!(
                    "`pad` must be followed by the character to pad, then the width as a number"
                )
            })?;
            if new_text.len() < width {
                for _ in 0..(width - new_text.len()) {
                    new_text.push_front(pad);
                }
            }
        } else if let Some(x) = op.strip_prefix("last") {
            let width = x.parse::<usize>().map_err(|_| {
                format!("`last` must be followed by the width as a non-negative number")
            })?;
            if new_text.len() > width {
                for _ in 0..(new_text.len() - width) {
                    new_text.pop_front();
                }
            }
        } else {
            return Err(format!("`{op}` is not a valid format function."));
        }
        next_op = iter.next();
    }

    Ok(new_text.into_iter().collect())
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::convert;

    const U: &str = "\u{01f33d}";
    
    fn exact_fn_for_test(a: f64) -> String {
        convert::float_to_string(a)
    }
    fn round_fn_for_test(a: f64) -> String {
        convert::float_to_string(a.round())
    }

    #[test]
    fn test_single_variable() {
        let text = "test";

        let result = transform_text_fn(text, |_|12.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("12.3".to_string()));
        let result = transform_text_fn(text, |_|12.0, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("12".to_string()));
    }

    #[test]
    fn test_pad_simple() {
        let text = "pad_3:test";

        let result = transform_text_fn(text, |_|12.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("_12".to_string()));
        let result = transform_text_fn(text, |_|12.0, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("_12".to_string()));
    }

    #[test]
    fn test_pad_unicode() {
        let text = &format!("pad{U}5:test");

        let result = transform_text_fn(text, |_|12.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}{U}{U}12")));
        let result = transform_text_fn(text, |_|12.0, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}{U}{U}12")));
    }

    #[test]
    fn test_last_simple() {
        let text = "last3:test";

        let result = transform_text_fn(text, |_|12.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("12".to_string()));
        let result = transform_text_fn(text, |_|123.0, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("123".to_string()));
        let result = transform_text_fn(text, |_|1234.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok("234".to_string()));
    }

    #[test]
    fn test_last_then_pad() {
        let text = &format!("pad{U}5:last3:test");

        let result = transform_text_fn(text, |_|12.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}{U}{U}12")));
        let result = transform_text_fn(text, |_|123.0, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}{U}123")));
        let result = transform_text_fn(text, |_|1234.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}{U}234")));
    }

    #[test]
    fn test_pad_then_last() {
        let text = &format!("last4:pad{U}5:last3:test");

        let result = transform_text_fn(text, |_|12.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}{U}12")));
        let result = transform_text_fn(text, |_|123.0, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}123")));
        let result = transform_text_fn(text, |_|1234.3, exact_fn_for_test, round_fn_for_test);
        assert_eq!(result, Ok(format!("{U}234")));
    }
}
