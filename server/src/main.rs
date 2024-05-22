//! Main entry point for the web server.
//!
//! Currently, the web server does not support HTTPS.
//! It is recommended to use a reverse proxy such as nginx to handle HTTPS.
//! Alternatively, you can use a CDN such as Cloudflare to proxy the website.

use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{Redirect, Response};
use axum::{middleware, routing, Router};
use axum_server::{Handle, Server};
use tokio::sync::Mutex;
use tokio::time::{self, Duration, Instant};
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{debug, info, Level};

mod api;
mod compiler;
mod env;
use env::Environment;
mod services;
use services::{AddHtmlExtService, NestedRouteRedirectService};
mod boot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::parse();
    tracing_subscriber::fmt()
        .compact()
        .with_ansi(std::io::stdout().is_terminal())
        .with_max_level(env.logging_level)
        .init();
    info!("celer server version: {}", env.version);
    info!("preparing assets...");
    boot::setup_site_origin(
        PathBuf::from(&env.docs_dir),
        PathBuf::from(&env.app_dir),
        &env.site_origin,
    )
    .await?;
    compiler::setup_global_loader();
    if env.gzip {
        info!("compressing assets...");
        boot::gzip_static_assets(PathBuf::from(&env.docs_dir), PathBuf::from(&env.app_dir)).await?;
    } else {
        info!("skipping compression of assets. Specify CELERSERVER_GZIP=true to enable gzip compression.");
    }
    info!("configuring routes...");

    let globals = Globals::new();

    let router = Router::new();
    let router = init_home(router);
    let router = init_docs(router, &env.docs_dir)?;
    let router = init_edit(router, &env.app_dir)?;
    let router = init_static(
        router,
        &env.app_dir,
        &["/celerc", "/static", "/assets", "/themes"],
    )?;
    let router = api::init_api(router, &env)?;

    let shutdown = middleware::from_fn_with_state(globals.clone(), shutdown_middleware);

    let router = router.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(shutdown),
    );

    let address = format!("0.0.0.0:{}", env.port).parse()?;
    let handle = globals.handle;
    if let Some(tls_config) = env.get_https_config().await {
        info!("starting server on https://{address}");
        axum_server::bind_rustls(address, tls_config)
            .handle(handle)
            .serve(router.into_make_service())
            .await?;
    } else {
        info!("starting server on http://{address}");
        Server::bind(address)
            .handle(handle)
            .serve(router.into_make_service())
            .await?;
    }

    info!("server stopped");

    Ok(())
}

/// Redirect / to /docs
fn init_home(router: Router) -> Router {
    debug!("/ -> /docs");
    router.route("/", routing::get(|| async { Redirect::temporary("/docs") }))
}

/// Setup the /docs route
fn init_docs(router: Router, docs_dir: &str) -> Result<Router, io::Error> {
    let docs_dir = Path::new(docs_dir).canonicalize()?;
    debug!("/docs -> {}", docs_dir.display());
    let docs_404_path = docs_dir.join("404.html").canonicalize()?;
    let service = NestedRouteRedirectService::new(
        "/docs",
        ServeDir::new(&docs_dir)
            .precompressed_gzip()
            .fallback(AddHtmlExtService(
                ServeDir::new(&docs_dir)
                    .precompressed_gzip()
                    .fallback(ServeFile::new(docs_404_path).precompressed_gzip()),
            )),
    );
    Ok(router.nest_service("/docs", service))
}

/// Setup the /edit route
fn init_edit(router: Router, app_dir: &str) -> Result<Router, io::Error> {
    let edit_path = Path::new(app_dir).join("edit.html").canonicalize()?;
    debug!("/edit -> {}", edit_path.display());

    let service = ServeFile::new(&edit_path).precompressed_gzip();
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
        debug!("{route} -> {}", path.display());
        let service =
            NestedRouteRedirectService::new(route, ServeDir::new(&path).precompressed_gzip());
        router = router.nest_service(route, service);
    }
    Ok(router)
}

/// Global state for server
#[derive(Clone)]
struct Globals {
    pub handle: Handle,
    pub last_shutdown_check: Arc<Mutex<Instant>>,
}

impl Globals {
    pub fn new() -> Self {
        Self {
            handle: Handle::new(),
            last_shutdown_check: Arc::new(Mutex::new(Instant::now())),
        }
    }
}

async fn shutdown_middleware(State(globals): State<Globals>, req: Request, next: Next) -> Response {
    let restart_interval = Duration::from_secs(15 * 60); // 15 minutes
    let response = next.run(req).await;
    let mut last_shutdown_check = globals.last_shutdown_check.lock().await;
    let now = Instant::now();
    let should_restart = now.duration_since(*last_shutdown_check) > restart_interval;
    *last_shutdown_check = now;
    if should_restart {
        info!("server has idled for too long, queueing restart...");
        let last_shutdown_check = globals.last_shutdown_check.clone();
        let handle = globals.handle.clone();
        tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_secs(5)).await;
                {
                    let last_shutdown_check = last_shutdown_check.lock().await;
                    let now = Instant::now();
                    if now.duration_since(*last_shutdown_check) > Duration::from_secs(5) {
                        break;
                    }
                }
            }
            info!("gracefully shutting down server...");
            handle.graceful_shutdown(Some(Duration::from_secs(15)));
        });
    }

    response
}
