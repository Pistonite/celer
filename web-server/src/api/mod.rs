use axum::{Router, routing};
use tracing::info;

use crate::env;

mod compile;
pub fn init_api_v1(router: Router) -> Router {
    info!("initializing api routes");
    router.route("/api/v1/version", routing::get(|| async { env::version() }))
}
