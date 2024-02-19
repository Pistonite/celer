use std::convert::Infallible;
use std::task::{Context, Poll};

use axum::body::{Bytes, HttpBody};
use axum::http::{Request, Response};
use tower::Service;
use tower_http::services::ServeDir;

/// Service that automatically adding .html extension to requests
#[derive(Debug, Clone)]
pub struct AddHtmlExtService<Fallback>(pub ServeDir<Fallback>);

impl<ReqBody, F, FResBody> Service<Request<ReqBody>> for AddHtmlExtService<F>
where
    F: Service<Request<ReqBody>, Response = Response<FResBody>, Error = Infallible> + Clone,
    F::Future: Send + 'static,
    FResBody: HttpBody<Data = Bytes> + Send + 'static,
    FResBody::Error: std::error::Error + Send + Sync,
{
    type Response = <ServeDir<F> as Service<Request<ReqBody>>>::Response;
    type Error = <ServeDir<F> as Service<Request<ReqBody>>>::Error;
    type Future = <ServeDir<F> as Service<Request<ReqBody>>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <ServeDir<F> as Service<Request<ReqBody>>>::poll_ready(&mut self.0, cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // this removes the scheme and authority, but it's ok since ServeDir doesn't care
        if let Ok(uri) = format!("{}.html", req.uri().path()).parse() {
            *req.uri_mut() = uri;
        }
        self.0.call(req)
    }
}
