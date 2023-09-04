//! Poor string parsing

use celerctypes::DocPoorText;

pub fn parse_poor(s: &str) -> Vec<DocPoorText> {
    let mut output = vec![];
    if s.is_empty() {
        return output;
    }
    let mut current = String::new();
    for part in s.split(' ') {
        if is_part_link(part) {
            if !current.is_empty() {
                output.push(DocPoorText::Text(current));
                current = String::new();
            }
            if part.ends_with('.') {
                let mut link = part.to_string();
                link.pop();
                output.push(DocPoorText::Link(link));
                current.push_str(". ");
            } else {
                output.push(DocPoorText::Link(part.to_string()));
                current.push(' ');
            }
        } else {
            current.push_str(part);
            current.push(' ');
        }
    }
    let current = current.trim_end().to_string();
    if !current.is_empty() {
        output.push(DocPoorText::Text(current));
    }
    output
}

fn is_part_link(part: &str) -> bool {
    if part.starts_with("http://") {
        return part.len() > 7;
    }
    if part.starts_with("https://") {
        return part.len() > 8;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(parse_poor(""), vec![]);
    }

    #[test]
    fn test_text_only() {
        assert_eq!(
            parse_poor("hello world"),
            vec![DocPoorText::Text("hello world".to_string())]
        );
        assert_eq!(
            parse_poor("hello world https"),
            vec![DocPoorText::Text("hello world https".to_string())]
        );
    }

    #[test]
    fn test_text_ends_with_link() {
        assert_eq!(
            parse_poor("hello world https://www.example.com"),
            vec![
                DocPoorText::Text("hello world ".to_string()),
                DocPoorText::Link("https://www.example.com".to_string()),
            ]
        );
    }

    #[test]
    fn test_text_starts_with_link() {
        assert_eq!(
            parse_poor("https://www.example.com boo"),
            vec![
                DocPoorText::Link("https://www.example.com".to_string()),
                DocPoorText::Text(" boo".to_string()),
            ]
        );
    }

    #[test]
    fn test_multiple_links() {
        assert_eq!(
            parse_poor("hello world https://www.example.com and http://example2.com and more"),
            vec![
                DocPoorText::Text("hello world ".to_string()),
                DocPoorText::Link("https://www.example.com".to_string()),
                DocPoorText::Text(" and ".to_string()),
                DocPoorText::Link("http://example2.com".to_string()),
                DocPoorText::Text(" and more".to_string()),
            ]
        );
    }

    #[test]
    fn test_ends_with_dot() {
        assert_eq!(
            parse_poor("hello world https://www.example.com."),
            vec![
                DocPoorText::Text("hello world ".to_string()),
                DocPoorText::Link("https://www.example.com".to_string()),
                DocPoorText::Text(".".to_string()),
            ]
        );
        assert_eq!(
            parse_poor("hello  world https://www.example.com. boo"),
            vec![
                DocPoorText::Text("hello  world ".to_string()),
                DocPoorText::Link("https://www.example.com".to_string()),
                DocPoorText::Text(". boo".to_string()),
            ]
        );
    }

    #[test]
    fn test_just_http() {
        assert_eq!(
            parse_poor("hello world https://"),
            vec![DocPoorText::Text("hello world https://".to_string()),]
        );
    }
}
