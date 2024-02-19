use axum::{Router, routing};
use tracing::info;

use crate::env;

mod compile;

pub fn init_api(router: Router) -> Router {
    info!("initializing api routes");
    router.nest("/api/v1", init_api_v1())
}

pub fn init_api_v1() -> Router {
    Router::new()
    .route("/version", routing::get(|| async { env::version() }))
        .nest("/compile", compile::init_compile_api())
}
