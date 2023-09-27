use super::Path;

impl Path {
    pub fn join<T>(&self, path: &T) -> Option<Self>
    where
        T: AsRef<str> + ?Sized,
    {
        let mut segs = if self.0.is_empty() {
            vec![]
        } else {
            self.0.split('/').collect::<Vec<_>>()
        };
        let path = path.as_ref().replace('\\', "/");
        for seg in path.split('/') {
            if seg.is_empty() {
                continue;
            }
            if seg == "." {
                continue;
            }
            if seg == ".." {
                segs.pop()?;
            } else {
                segs.push(seg);
            }
        }

        Some(Self(segs.join("/")))
    }

    #[inline]
    pub fn join_in_place<T>(self, path: &T) -> Result<Self, Self>
    where
        T: AsRef<str> + ?Sized,
    {
        self.join(path).ok_or(self)
    }

    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.join("..")
    }

    #[inline]
    pub fn parent_in_place(self) -> Result<Self, Self> {
        self.parent().ok_or(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_resolve_empty() {
        let path = Path::try_from("a/b").unwrap();
        let result = path.join("");
        assert_eq!(result.unwrap().as_ref(), "a/b");
        let result = path.join("/");
        assert_eq!(result.unwrap().as_ref(), "a/b");
        let result = path.join("//");
        assert_eq!(result.unwrap().as_ref(), "a/b");
        let result = path.join("\\");
        assert_eq!(result.unwrap().as_ref(), "a/b");
        let result = path.join("\\/\\");
        assert_eq!(result.unwrap().as_ref(), "a/b");
        let result = path.join("./.\\");
        assert_eq!(result.unwrap().as_ref(), "a/b");
    }

    #[test]
    fn test_resolve_one_seg() {
        let path = Path::try_from("a/b").unwrap();
        let result = path.join("c");
        assert_eq!(result.unwrap().as_ref(), "a/b/c");
        let result = path.join("\\c");
        assert_eq!(result.unwrap().as_ref(), "a/b/c");
        let result = path.join("/c/.");
        assert_eq!(result.unwrap().as_ref(), "a/b/c");
    }

    #[test]
    fn test_resolve_multiple_segs() {
        let path = Path::try_from("a/b").unwrap();
        let result = path.join("c/d/e");
        assert_eq!(result.unwrap().as_ref(), "a/b/c/d/e");
        let result = path.join("/c/./e\\");
        assert_eq!(result.unwrap().as_ref(), "a/b/c/e");
    }

    #[test]
    fn test_resolve_with_parent() {
        let path = Path::try_from("a/b").unwrap();
        let result = path.join("..");
        assert_eq!(result.unwrap().as_ref(), "a");
        let result = path.join("../c");
        assert_eq!(result.unwrap().as_ref(), "a/c");
        let result = path.join("/c/..");
        assert_eq!(result.unwrap().as_ref(), "a/b");
        let result = path.join("../..");
        assert_eq!(result.unwrap().as_ref(), "");
    }

    #[test]
    fn test_error() {
        assert!(Path::try_from("a/b").unwrap().join("../../..").is_none());
        assert!(Path::try_from("a").unwrap().join("../..").is_none());
        assert!(Path::try_from("").unwrap().join("..").is_none());
    }
}
