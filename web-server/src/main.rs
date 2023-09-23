//! Main entry point for the web server.
//!
//! Currently, the web server does not support HTTPS.
//! It is recommended to use a reverse proxy such as nginx to handle HTTPS.
//! Alternatively, you can use a CDN such as Cloudflare to proxy the website.

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::future::Future;
use std::io::{self, Bytes, Empty};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use axum::body::Body;
use axum::response::{Response, IntoResponse};
use axum::{Server, Router};
use axum::http::{Request, Uri, StatusCode};
use clap::Parser;
use futures::future::BoxFuture;
use tower_http::services::ServeDir;

mod state;
// use state::State;

#[derive(Debug, Parser)]
#[command(name = "start-server")]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
    /// The port to listen on.
    #[arg(short, long, default_value = "8173")]
    port: u16,
    /// Serve docs from a different location
    #[arg(long, default_value = "docs")]
    docs_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let address = format!("0.0.0.0:{}", args.port);

    log::info!("Starting web server on {address}");

    let router = Router::new().nest_service("/docs", create_serve_dir(&args.docs_dir));

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
    Server::bind(&address.parse().expect("invalid address"))
        .serve(router.into_make_service())
        .await.expect("fail to convert router into service");

    Ok(())
}

fn create_serve_dir(fs_path: &str) -> ServeDirWrapper {
    ServeDirWrapper::new(ServeDir::new(fs_path))
}

#[derive(Debug, Clone)]
struct ServeDirWrapper {
    // location_cache: BTreeMap<String, Uri>,
    serve_dir: ServeDir,
}

impl ServeDirWrapper {
    fn new(serve_dir: ServeDir) -> Self {
        Self {
            // location_cache: BTreeMap::new(),
            serve_dir
        }
    }
}

use tower::Service;
use tower_http::services::fs::ServeFileSystemResponseBody;

impl Service<Request<Body>> for ServeDirWrapper {
    type Response = Response;
    type Error = Infallible;
    type Future =BoxFuture<'static, Result<Response, Infallible>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <ServeDir as Service<Request<Body>>>::poll_ready(&mut self.serve_dir, cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = req.uri().path().to_string();
        // if path ends with / or .html, there's no guessing needed
        if path.ends_with('/')  || path.ends_with(".html") {
            let future = self.serve_dir.call(req);
            return Box::pin(async move {
                // unwrap is safe because call is infallible
                Ok(future.await.unwrap().into_response())
            });
        }
        // if let Some(location) = self.location_cache.get(&path) {
        //     // redirect cache found, replace the uri and call the underlying service
        //     *req.uri_mut() = location.clone();
        //     let future = self.serve_dir.call(req);
        //     return Box::pin(async move {
        //         // unwrap is safe because call is infallible
        //         Ok(future.await.unwrap().into_response())
        //     });
        // };
        // try adding .html extension
        let html_uri: Uri = match format!("{}.html", path).parse() {
            Ok(uri) => uri,
            _ => return Box::pin( async { Ok(StatusCode::NOT_FOUND.into_response()) })
        };
        let req_html = replace_uri(&req, html_uri.clone());
        // try adding /index.html
        let index_html_uri: Uri = match format!("{}/index.html", path).parse() {
            Ok(uri) => uri,
            _ => return Box::pin( async { Ok(StatusCode::NOT_FOUND.into_response()) })
        };
        let req_index_html = replace_uri(&req, index_html_uri.clone());
        let future_original = self.serve_dir.try_call(req);
        let future_html = self.serve_dir.try_call(req_html);
        let future_index_html = self.serve_dir.try_call(req_index_html);

        Box::pin(async move {
            if let Ok(res) = future_original.await {
                return Ok(res.into_response());
            }
            if let Ok(res) = future_html.await {
                // add cache
                return Ok(res.into_response());
            }
            if let Ok(res) = future_index_html.await {
                // add cache
                return Ok(res.into_response());
            }
            return Ok(StatusCode::NOT_FOUND.into_response())
        })
    }

}

/// Clone the request without the body and extensions
fn clone_request<ReqBody>(base: &Request<ReqBody>) -> Request<()> {
    let mut new_req = Request::new(());
    *new_req.method_mut() = base.method().clone();
    *new_req.headers_mut() = base.headers().clone();
    *new_req.uri_mut() = base.uri().clone();
    new_req
}

fn replace_uri<ReqBody>(base: &Request<ReqBody>, uri: Uri) -> Request<()> {
    let mut new_req = Request::new(());
    *new_req.method_mut() = base.method().clone();
    *new_req.headers_mut() = base.headers().clone();
    *new_req.uri_mut() = uri;
    new_req
}

// #[derive(Debug, Clone)]
// struct NotFoundMiddleware {
//     path: AsyncPathBuf,
// }
// #[async_trait::async_trait]
// impl Middleware<State> for NotFoundMiddleware {
//     async fn handle(&self, req: Request<State>, next: tide::Next<'_, State>) -> tide::Result {
//         let mut res = next.run(req).await;
//         if res.status() != StatusCode::NotFound {
//             return Ok(res);
//         }
//         match Body::from_file(&self.path).await {
//             Ok(body) => {
//                 res.set_body(body);
//                 //res.set_status(StatusCode::Ok);
//                 Ok(res)
//             }
//             Err(e) if e.kind() == io::ErrorKind::NotFound => {
//                 log::warn!("File not found: {:?}", &self.path);
//                 Ok(res)
//             }
//             Err(e) => Err(e.into()),
//         }
//     }
// }

// async fn handle_docs_proxy(req: Request<State>) -> tide::Result {
//     let location = req.state().docs_proxy.as_ref().expect("docs_proxy must exist, check the routes");
//     let client = &req.state().http_client;
//     state::handle_get_proxy(client, location, &req).await
// }
