use super::Path;

impl Path {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn try_from(path: &str) -> Option<Self> {
        Self::new().join(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_new() {
        let path = Path::new();
        assert_eq!(path.as_ref(), "");
    }

    #[test]
    fn test_path_from_empty() {
        let path = Path::try_from("");
        assert_eq!(path.unwrap().as_ref(), "");
    }

    #[test]
    fn test_path_from_one_seg() {
        let path = Path::try_from("a");
        assert_eq!(path.unwrap().as_ref(), "a");
        let path = Path::try_from("/a");
        assert_eq!(path.unwrap().as_ref(), "a");
        let path = Path::try_from("/a/");
        assert_eq!(path.unwrap().as_ref(), "a");
        let path = Path::try_from("///a\\/");
        assert_eq!(path.unwrap().as_ref(), "a");
    }

    #[test]
    fn test_path_from_multiple_segs() {
        let path = Path::try_from("a/b");
        assert_eq!(path.unwrap().as_ref(), "a/b");
        let path = Path::try_from("/a/b/");
        assert_eq!(path.unwrap().as_ref(), "a/b");
        let path = Path::try_from("/ac/b");
        assert_eq!(path.unwrap().as_ref(), "ac/b");
        let path = Path::try_from("\\\\a\\/b/c/hello/");
        assert_eq!(path.unwrap().as_ref(), "a/b/c/hello");
    }

}
