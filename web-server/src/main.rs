//! Main entry point for the web server.
//!
//! Currently, the web server does not support HTTPS.
//! It is recommended to use a reverse proxy such as nginx to handle HTTPS.
//! Alternatively, you can use a CDN such as Cloudflare to proxy the website.

use axum::{Router, Server};
use std::io;
use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
use tracing::{debug, info, Level};

mod env;
use env::Environment;
mod services;
use services::{AddHtmlExtService, NestedRouteRedirectService};

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
    let router = init_edit(router, &env.app_dir)?;
    let router = init_static(router, &env.app_dir, &["/static", "/assets", "/themes"])?;

    let router = router.layer(
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
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
    let service = NestedRouteRedirectService::new(
        "/docs",
        ServeDir::new(&docs_dir).fallback(AddHtmlExtService(
            ServeDir::new(&docs_dir).fallback(ServeFile::new(&docs_404_path)),
        )),
    );
    Ok(router.nest_service("/docs", service))
}

/// Setup the /edit route
fn init_edit(router: Router, app_dir: &str) -> Result<Router, io::Error> {
    let edit_path = Path::new(app_dir).join("edit.html").canonicalize()?;
    debug!("/edit -> {}", edit_path.display());

    let service = ServeFile::new(&edit_path);
    Ok(router.nest_service("/edit", service))
}

/// Setup static asset routes from web client
fn init_static(
    mut router: Router,
    app_dir: &str,
    routes: &[&'static str],
) -> Result<Router, io::Error> {
    let app_dir = Path::new(app_dir);

    for route in routes {
        // strip the leading slash
        debug_assert_eq!(route.chars().next(), Some('/'));
        let path = app_dir.join(&route[1..]).canonicalize()?;
        debug!("/{route} -> {}", path.display());
        let service = NestedRouteRedirectService::new(route, ServeDir::new(&path));
        router = router.nest_service(route, service);
    }
    Ok(router)
}
