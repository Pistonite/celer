/// Async iterator wrapper using tokio_stream::StreamExt
macro_rules! async_for {
    ($v:pat in $iter:expr, $body:stmt) => {{
        let mut iter = tokio_stream::iter($iter);
        while let Some($v) = tokio_stream::StreamExt::next(&mut iter).await {
            $body
        }
    }};
}
pub(crate) use async_for;
