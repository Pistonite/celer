//! Main entry point for the web server.
//!
//! Currently, the web server does not support HTTPS.
//! It is recommended to use a reverse proxy such as nginx to handle HTTPS.
//! Alternatively, you can use a CDN such as Cloudflare to proxy the website.

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::future::Future;
use std::io::{self, Bytes, Empty};
use std::path::{PathBuf, Path};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use axum::body::Body;
use axum::response::{Response, IntoResponse};
use axum::{Server, Router};
use axum::http::{Request, Uri, StatusCode};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultOnResponse, DefaultOnRequest, DefaultMakeSpan};
use tracing::{Level, debug, info};

mod env;
use env::Environment;
mod services;
use services::{NestedRouteRedirectService, AddHtmlExtService};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::parse();
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(env.logging_level.clone())
        .init();
    info!("configuring routes...");

    let router = Router::new();
    let router = init_docs(router, &env.docs_dir)?;


    let router = router.layer(tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))

    );

    let address = format!("0.0.0.0:{}", env.port).parse()?;
    info!("starting server on {address}");
    Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

/// Setup the /docs route
fn init_docs(router: Router, docs_dir: &str) -> Result<Router, io::Error> {
    let docs_dir = Path::new(docs_dir).canonicalize()?;
    debug!("/docs -> {}", docs_dir.display());
    let docs_404_path = docs_dir.join("404.html").canonicalize()?;
    let service = NestedRouteRedirectService::new("/docs", 
        ServeDir::new(&docs_dir).fallback(
            AddHtmlExtService(ServeDir::new(&docs_dir).fallback(
                ServeFile::new(&docs_404_path)
            ))
        )
    );
    Ok(router.nest_service("/docs", service))
}

// fn create_serve_dir(fs_path: &str, not_found_path: &str) -> ServeDirWrapper {
//     ServeDirWrapper::new(ServeDir::new(fs_path), ServeFile::new(not_found_path))
// }

// #[derive(Debug, Clone)]
// struct ServeDirWrapper {
//     // location_cache: BTreeMap<String, Uri>,
//     serve_dir: ServeDir,
//     serve_404: ServeFile,
// }
//
// impl ServeDirWrapper {
//     fn new(serve_dir: ServeDir, serve_404: ServeFile) -> Self {
//         Self {
//             // location_cache: BTreeMap::new(),
//             serve_dir,
//             serve_404,
//         }
//     }
// }
//
// use tower::Service;
// use tower_http::trace;
// use tracing::Level;
//
// impl Service<Request<Body>> for ServeDirWrapper {
//     type Response = Response;
//     type Error = Infallible;
//     type Future =BoxFuture<'static, Result<Response, Infallible>>;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         <ServeDir as Service<Request<Body>>>::poll_ready(&mut self.serve_dir, cx)
//     }
//
//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         let path = req.uri().path().to_string();
//         // if path ends with / or .html, there's no guessing needed
//         if path.ends_with('/')  || path.ends_with(".html") {
//             let future = self.serve_dir.call(req);
//             return Box::pin(async move {
//                 Ok(future.await?.into_response())
//             });
//         }
//        
//         // try adding .html extension
//         let html_uri: Uri = match format!("{}.html", path).parse() {
//             Ok(uri) => uri,
//             _ => return Box::pin( async { Ok(StatusCode::NOT_FOUND.into_response()) })
//         };
//         let req_html = replace_uri(&req, html_uri);
//         // try adding /index.html
//         let index_html_uri: Uri = match format!("{}/index.html", path).parse() {
//             Ok(uri) => uri,
//             _ => return Box::pin( async { Ok(StatusCode::NOT_FOUND.into_response()) })
//         };
//         let req_index_html = replace_uri(&req, index_html_uri);
//         let future_original = self.serve_dir.call(req);
//         let future_html = self.serve_dir.call(req_html);
//         let future_index_html = self.serve_dir.call(req_index_html);
//
//         Box::pin(async move {
//             if let Ok(res) = future_original.await {
//                 let status = res.status();
//                 match status {
//                     StatusCode::NOT_FOUND => {
//                         // try adding .html
//                         if let Ok(res) = future_html.await {
//                             return Ok(res.into_response());
//                         }
//                     }
//                     StatusCode::TEMPORARY_REDIRECT => {
//                         // try adding /index.html
//                         if let Ok(res) = future_index_html.await {
//                             return Ok(res.into_response());
//                         }
//                     }
//                     _ => {
//                         return Ok(res.into_response());
//                     }
//                 }
//             }
//             return Ok(StatusCode::NOT_FOUND.into_response())
//         })
//     }
//
// }

// /// Clone the request without the body and extensions
// fn clone_request<ReqBody>(base: &Request<ReqBody>) -> Request<()> {
//     let mut new_req = Request::new(());
//     *new_req.method_mut() = base.method().clone();
//     *new_req.headers_mut() = base.headers().clone();
//     *new_req.uri_mut() = base.uri().clone();
//     new_req
// }
//
// fn replace_uri<ReqBody>(base: &Request<ReqBody>, uri: Uri) -> Request<()> {
//     let mut new_req = Request::new(());
//     *new_req.method_mut() = base.method().clone();
//     *new_req.headers_mut() = base.headers().clone();
//     *new_req.uri_mut() = uri;
//     new_req
// }
//
    // TODO: state
    //app.with(security::CorsMiddleware::new());
    // app.with(driftwood::ApacheCombinedLogger);
    // let not_found_middleware = NotFoundMiddleware {
    //     path: AsyncPathBuf::from(&args.docs_dir).join("404.html"),
    // };
    // if let Some(docs_dir) = args.docs_dir {
    //     app.at("/docs/*").get(handle_docs_proxy);
    //     app.at("/docs").get(handle_docs_proxy);
    // } else {
    //     todo!("docs")
    // }
    // app.register(
    //     root()
    //         .at("docs", |r| {
    //             r.serve_file(PathBuf::from(&args.docs_dir).join("index.html"))
    //                 .expect("Failed to setup /docs route")
    //         })
    //         .at("docs/", |r| {
    //             r.with(not_found_middleware, |r| {
    //                 r.serve_dir(&args.docs_dir)
    //                     .expect("Failed to setup /docs route")
    //             })
    //             .serve_file(PathBuf::from(&args.docs_dir).join("index.html"))
    //             .expect("Failed to setup /docs route")
    //         }),
    // );

    // app.at("/docs").with(NotFoundMiddleware {
    //     path: AsyncPathBuf::from(&args.docs_dir).join("404.html"),
    // }).serve_dir(&args.docs_dir)?;
    // app.at("/docs").serve_file(PathBuf::from(&args.docs_dir).join("index.html"))?;
    // log::info!("Serving {} at /docs", args.docs_dir);

    // // log::info!("Setting up /icon");
    // // app.at("/icon/:icon").get(get_icon);
    // // log::info!("Setting up /icons");
    // // app.at("/icons").get(get_icons);
    // // log::info!("Setting up static files");
    // // app.at("/").serve_dir(static_dir)?;
    // // log::info!("Setting up index.heml");
    // // app.at("/").serve_file(format!("{}index.html", static_dir))?;
    //
