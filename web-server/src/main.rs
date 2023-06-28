//! Main entry point for the web server.
//!
//! Currently, the web server does not support HTTPS.
//! It is recommended to use a reverse proxy such as nginx to handle HTTPS.
//! Alternatively, you can use a CDN such as Cloudflare to proxy the website.

use std::io;
use std::path::PathBuf;
use async_std::path::PathBuf as AsyncPathBuf;

use clap::Parser;
use tide::{Redirect, Request, Response, Middleware, Body, StatusCode};
use tide::security;
//use surf::{Client, Config};

mod state;
use state::State;
use tide_fluent_routes::fs::ServeFs;
use tide_fluent_routes::root;
use tide_fluent_routes::routebuilder::RouteBuilder;
use tide_fluent_routes::router::Router;

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


#[async_std::main]
async fn main() -> Result<(), std::io::Error>{
    let args = Cli::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let address = format!("0.0.0.0:{}", args.port);

    log::info!("Starting web server on {address}");
    // TODO: state
    let mut app = tide::with_state(State {
       // http_client: Client::new(),
        //docs_proxy: args.docs_proxy.clone(),
    });
    //app.with(security::CorsMiddleware::new());
    app.with(driftwood::ApacheCombinedLogger);
    let not_found_middleware = NotFoundMiddleware {
        path: AsyncPathBuf::from(&args.docs_dir).join("404.html"),
    };
    // if let Some(docs_dir) = args.docs_dir {
    //     app.at("/docs/*").get(handle_docs_proxy);
    //     app.at("/docs").get(handle_docs_proxy);
    // } else {
    //     todo!("docs")
    // }
    app.register(
        root()
            .at("docs", |r| r
                .serve_file(PathBuf::from(&args.docs_dir).join("index.html")).expect("Failed to setup /docs route")
            )
            .at("docs/", |r| r
                .with(not_found_middleware, |r| r.serve_dir(&args.docs_dir).expect("Failed to setup /docs route"))
                .serve_file(PathBuf::from(&args.docs_dir).join("index.html")).expect("Failed to setup /docs route")
            )
    );

    // app.at("/docs").with(NotFoundMiddleware {
    //     path: AsyncPathBuf::from(&args.docs_dir).join("404.html"),
    // }).serve_dir(&args.docs_dir)?;
    // app.at("/docs").serve_file(PathBuf::from(&args.docs_dir).join("index.html"))?;
    log::info!("Serving {} at /docs", args.docs_dir);
    
    // // log::info!("Setting up /icon");
    // // app.at("/icon/:icon").get(get_icon);
    // // log::info!("Setting up /icons");
    // // app.at("/icons").get(get_icons);
    // // log::info!("Setting up static files");
    // // app.at("/").serve_dir(static_dir)?;
    // // log::info!("Setting up index.heml");
    // // app.at("/").serve_file(format!("{}index.html", static_dir))?;
    
    app.listen(address).await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct NotFoundMiddleware {
    path: AsyncPathBuf,
}
#[async_trait::async_trait]
impl Middleware<State> for NotFoundMiddleware {
    async fn handle(&self, req: Request<State>, next: tide::Next<'_, State>) -> tide::Result {
        let mut res = next.run(req).await;
        if res.status() != StatusCode::NotFound {
            return Ok(res);
        }
        match Body::from_file(&self.path).await {
            Ok(body) => {
                res.set_body(body);
                //res.set_status(StatusCode::Ok);
                Ok(res)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                log::warn!("File not found: {:?}", &self.path);
                Ok(res)
            }
            Err(e) => Err(e.into()),
        }
    }
}

// async fn handle_docs_proxy(req: Request<State>) -> tide::Result {
//     let location = req.state().docs_proxy.as_ref().expect("docs_proxy must exist, check the routes");
//     let client = &req.state().http_client;
//     state::handle_get_proxy(client, location, &req).await
// }
