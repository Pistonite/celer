use crate::util::async_for;

use super::{TempStr, TempStrBlock};

impl TempStr {
    /// Replace variable in a template string with arguments
    ///
    /// If an argument is missing, it will be replaced with an empty string
    pub async fn hydrate<S>(&self, args: &[S]) -> String
    where
        S: AsRef<str>,
    {
        let mut s = String::new();
        async_for!(block in &self.0, {
            match block {
                TempStrBlock::Lit(lit) => s.push_str(lit),
                TempStrBlock::Var(idx) => {
                    if let Some(arg) = args.get(*idx) {
                        s.push_str(arg.as_ref());
                    }
                }
            }
        });
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_only_literal() {
        let empty_arg: &[&str] = &[];
        assert_eq!(TempStr::from("abcd").hydrate(empty_arg).await, "abcd");
        assert_eq!(
            TempStr::from("abcd").hydrate(&["hello".to_string()]).await,
            "abcd"
        );
        assert_eq!(
            TempStr::from("abcd").hydrate(&["hello", "world"]).await,
            "abcd"
        );
    }

    #[tokio::test]
    async fn test_only_variable() {
        let args = &["hello", "world", "temp"];
        assert_eq!(TempStr::from("$(0)").hydrate(args).await, "hello");
        assert_eq!(TempStr::from("$(1)").hydrate(args).await, "world");
        assert_eq!(TempStr::from("$(2)").hydrate(args).await, "temp");
        assert_eq!(TempStr::from("$(3)").hydrate(args).await, "");
    }

    #[tokio::test]
    async fn test_mixed() {
        let args = &["hello", "world", "temp"];
        assert_eq!(TempStr::from("foo$(0)").hydrate(args).await, "foohello");
        assert_eq!(TempStr::from("$(1)bar").hydrate(args).await, "worldbar");
        assert_eq!(
            TempStr::from("bar$(2)foo$(1)").hydrate(args).await,
            "bartempfooworld"
        );
        assert_eq!(
            TempStr::from("bar$(3)$(3) $(2)$(1)$(2)")
                .hydrate(args)
                .await,
            "bar tempworldtemp"
        );
    }
}
