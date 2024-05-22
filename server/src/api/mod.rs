use std::io;

use axum::{routing, Router};
use tracing::info;

use crate::env::Environment;

mod compile;
mod export;
mod header;
mod view;

pub fn init_api(router: Router, env: &Environment) -> Result<Router, io::Error> {
    info!("initializing api routes");
    let router = router
        .nest("/api/v1", init_api_v1(env)?)
        .nest("/view", view::init_api(&env.app_dir)?);

    Ok(router)
}

pub fn init_api_v1(env: &Environment) -> Result<Router, io::Error> {
    let version = env.version.clone();
    let router = Router::new()
        .route("/version", routing::get(move || async { version }))
        .nest("/compile", compile::init_api())
        .nest("/export", export::init_api());

    Ok(router)
}
