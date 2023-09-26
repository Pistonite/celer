use std::convert::Infallible;
use std::task::{Context, Poll};

use axum::body::Bytes;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use futures::future::BoxFuture;
use http_body::Body;
use tower::Service;
use tower_http::services::ServeDir;

/// Service for propagating redirect responses from the inner service
#[derive(Debug, Clone)]
pub struct NestedRouteRedirectService<Fallback> {
    /// Server route such as /docs
    route: &'static str,
    /// Inner service
    inner: ServeDir<Fallback>,
}

impl<F> NestedRouteRedirectService<F> {
    pub fn new(route: &'static str, inner: ServeDir<F>) -> Self {
        Self { route, inner }
    }
}

impl<ReqBody, F, FResBody> Service<Request<ReqBody>> for NestedRouteRedirectService<F>
where
    F: Service<Request<ReqBody>, Response = Response<FResBody>, Error = Infallible> + Clone + Send + 'static,
    F::Future: Send + 'static,
    FResBody: Body<Data = Bytes> + Send + 'static,
    FResBody::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    ReqBody: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Response, Infallible>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <ServeDir<F> as Service<Request<ReqBody>>>::poll_ready(&mut self.inner, cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let future = self.inner.call(req);
        let route = self.route;
        Box::pin(async move {
            let mut res = future.await?;
            // if the response is a redirect, we need to prepend the route
            // to the location header
            if matches!(
                res.status(),
                StatusCode::MOVED_PERMANENTLY
                    | StatusCode::PERMANENT_REDIRECT
                    | StatusCode::TEMPORARY_REDIRECT
            ) {
                if let Some(location) = res.headers().get("location") {
                    // prepend route to location
                    if let Ok(location) = location.to_str() {
                        if let Ok(location) = format!("{route}{location}").parse() {
                            res.headers_mut().insert("location", location);
                        };
                    }
                }
            }
            Ok(res.into_response())
        })
    }
}

