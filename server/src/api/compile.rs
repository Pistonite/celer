//! The `/compile` API endpoint.

use axum::{Json, Router};
use axum::extract::Path;
use axum::routing;
use instant::Instant;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

use crate::compiler;

pub fn init_api() -> Router {
    Router::new()
        .route("/:owner/:repo/:reference", routing::get(compile_owner_repo_ref))
        .route(
            "/:owner/:repo/:reference/*path",
            routing::get(compile_owner_repo_ref_path),
        )
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum CompileResponse {
    Success(Value),
    Failure(String),
}

async fn compile_owner_repo_ref(
    Path((owner, repo, reference)): Path<(String, String, String)>,
) -> Json<CompileResponse> {
    let response = compile_internal(&owner, &repo, None, &reference).await;
    Json(response)
}

async fn compile_owner_repo_ref_path(
    Path((owner, repo, reference, path)): Path<(String, String, String, String)>,
) -> Json<CompileResponse> {
    let response = compile_internal(&owner, &repo, Some(&path), &reference).await;
    Json(response)
}

async fn compile_internal(
    owner: &str,
    repo: &str,
    path: Option<&str>,
    reference: &str,
) -> CompileResponse {
    // TODO #192: plugin options
    let start_time = Instant::now();
    let prep_ctx = match compiler::get_context(owner, repo, path, reference).await {
        Ok(ctx) => ctx,
        Err(e) => return CompileResponse::Failure(e.to_string()),
    };

    let expo_ctx = compiler::compile(&prep_ctx, Some(start_time), None).await;
    let expo_ctx_json = match serde_json::to_value(expo_ctx) {
        Ok(v) => v,
        Err(e) => return CompileResponse::Failure(e.to_string()),
    };
    CompileResponse::Success(expo_ctx_json)
}
