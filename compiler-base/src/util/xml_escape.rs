use std::borrow::Cow;

macro_rules! escape {
    ($escaped:ident, $bytes:literal, $s:ident, $i:ident, $len:literal) => {
        match &mut $escaped {
            None => {
                let mut vec = Vec::with_capacity($s.len() + $len);
                vec.extend_from_slice(&$s.as_bytes()[0..$i]);
                vec.extend_from_slice($bytes);
                $escaped = Some(vec);
            }
            Some(vec) => vec.extend_from_slice($bytes),
        }
    };
}

/// Escapes a string for XML.
///
/// This function escapes the following characters:
/// - `&` becomes `&amp;`
/// - `<` becomes `&lt;`
/// - `>` becomes `&gt;`
/// - `"` becomes `&quot;`
/// - `'` becomes `&apos;`
pub fn xml_escape(s: &str) -> Cow<str> {
    // An ASCII byte always represent a ASCII character
    // so it is safe to treat the input as bytes
    let mut escaped = None;
    for (i, b) in s.bytes().enumerate() {
        match b {
            b'&' => {
                escape!(escaped, b"&amp;", s, i, 5);
            }
            b'<' => {
                escape!(escaped, b"&lt;", s, i, 5);
            }
            b'>' => {
                escape!(escaped, b"&gt;", s, i, 5);
            }
            b'\'' => {
                escape!(escaped, b"&apos;", s, i, 5);
            }
            b'"' => {
                escape!(escaped, b"&quot;", s, i, 5);
            }
            _ => {
                if let Some(vec) = &mut escaped {
                    vec.push(b);
                }
            }
        }
    }
    match escaped {
        Some(vec) => Cow::Owned(String::from_utf8(vec).unwrap()),
        None => Cow::Borrowed(s),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_escape() {
        let input = "This is a test.";
        let expected = Cow::Borrowed(input);
        let escaped = xml_escape(input);
        assert_eq!(escaped, expected);
    }

    #[test]
    fn no_escape_unicode() {
        let input = "\u{0926}This is a test.";
        let expected = Cow::Borrowed(input);
        let escaped = xml_escape(input);
        assert_eq!(escaped, expected);
    }

    #[test]
    fn escape_amp() {
        let input = "\u{4f60}&\u{597d}& Chips";
        let expected: Cow<str> = Cow::Owned(String::from("\u{4f60}&amp;\u{597d}&amp; Chips"));
        let escaped = xml_escape(input);
        assert_eq!(escaped, expected);
    }

    #[test]
    fn escape_lt_gt() {
        let input = "2 < 3 and 3 > 2";
        let expected: Cow<str> = Cow::Owned(String::from("2 &lt; 3 and 3 &gt; 2"));
        let escaped = xml_escape(input);
        assert_eq!(escaped, expected);
    }

    #[test]
    fn escape_quotes() {
        let input = r#"She said, "Hello, world!""#;
        let expected: Cow<str> = Cow::Owned(String::from("She said, &quot;Hello, world!&quot;"));
        let escaped = xml_escape(input);
        assert_eq!(escaped, expected);
    }

    #[test]
    fn escape_apos() {
        let input = "It's a sunny day";
        let expected: Cow<str> = Cow::Owned(String::from("It&apos;s a sunny day"));
        let escaped = xml_escape(input);
        assert_eq!(escaped, expected);
    }
}
