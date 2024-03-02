use std::io;

use axum::{routing, Router};
use tracing::info;

use crate::env;

mod compile;
mod export;
mod view;
mod header;

pub fn init_api(router: Router, app_dir: &str) -> Result<Router, io::Error> {
    info!("initializing api routes");
    let router = router
        .nest("/api/v1", init_api_v1()?)
        .nest("/view", view::init_api(app_dir)?);

    Ok(router)
}

pub fn init_api_v1() -> Result<Router, io::Error> {
    let router = Router::new()
        .route("/version", routing::get(|| async { env::version() }))
        .nest("/compile", compile::init_api())
        .nest("/export", export::init_api());

    Ok(router)
}
