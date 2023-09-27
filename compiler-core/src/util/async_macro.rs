/// Async iterator wrapper using tokio_stream::StreamExt
macro_rules! async_for {
    ($v:pat in $iter:expr, $body:stmt) => {{
        let mut iter = tokio_stream::iter($iter);
        while let Some($v) = tokio_stream::StreamExt::next(&mut iter).await {
            tokio::task::yield_now().await;
            $body
        }
        let r: Result<(), std::convert::Infallible> = Ok(());
        r
    }};
}
pub(crate) use async_for;

macro_rules! yield_now {
    () => {
        tokio::task::yield_now().await
    };
}
pub(crate) use yield_now;
