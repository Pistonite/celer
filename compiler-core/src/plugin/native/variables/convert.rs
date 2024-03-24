/// Convert a `f64` to a decimal string
///
/// If the number is close to an integer, the string will not contain the decimal point.
pub fn float_to_string(a: f64) -> String {
    let rounded = a.round();
    if super::float_eq(a, rounded) {
        (rounded as i64).to_string()
    } else {
        a.to_string()
    }
}

/// Convert a `f64` to a lower case hex string
///
/// If the number is negative, returns the input (not rounded) as a decimal string
pub fn to_hex(a: f64) -> String {
    match round_nonneg(a) {
        Some(i) => format!("{i:x}"),
        None => float_to_string(a),
    }
}

/// Convert a `f64` to a upper case hex string
///
/// If the number is negative, returns the input (not rounded) as a decimal string
pub fn to_hex_upper(a: f64) -> String {
    match round_nonneg(a) {
        Some(i) => format!("{i:X}"),
        None => float_to_string(a),
    }
}

/// Convert a `f64` to a roman numeral lower case.
///
/// If the input is out of the representable range, returns the input (not rounded) as a string
pub fn to_roman(a: f64) -> String {
    match to_roman_core(a) {
        Some(mut s) => {
            s.make_ascii_lowercase();
            s
        }
        None => float_to_string(a),
    }
}

/// Convert a `f64` to a roman numeral upper case.
///
/// If the input is out of the representable range, returns the input (not rounded) as a string
pub fn to_roman_upper(a: f64) -> String {
    to_roman_core(a).unwrap_or_else(|| float_to_string(a))
}

/// Convert a `f64` to a roman numeral upper case.
///
/// Returns None if the input is out of the range representable by roman numerals.
/// Otherwise the input is rounded.
#[inline]
fn to_roman_core(a: f64) -> Option<String> {
    match round_nonneg(a) {
        Some(i) if i >= 1 && i <= roman::MAX as u64 => roman::to(i as i32),
        _ => None,
    }
}

/// Round the input if it is non-negative, otherwise return `None`.
#[inline]
fn round_nonneg(a: f64) -> Option<u64> {
    let i = a.round() as i64;
    if i < 0 {
        None
    } else {
        Some(i as u64)
    }
}
