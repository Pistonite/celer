//! The `/compile` API endpoint.

use axum::extract::Path;
use axum::http::HeaderMap;
use axum::routing;
use axum::{Json, Router};
use instant::Instant;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

use crate::compiler;

use super::header;

pub fn init_api() -> Router {
    Router::new()
        .route(
            "/:owner/:repo/:reference",
            routing::get(compile_owner_repo_ref),
        )
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
    headers: HeaderMap,
) -> Json<CompileResponse> {
    let plugin_options = match header::get_plugin_options(&headers) {
        Ok(v) => v,
        Err(e) => return Json(CompileResponse::Failure(e)),
    };
    let response = compile_internal(&owner, &repo, None, &reference, &plugin_options).await;
    Json(response)
}

async fn compile_owner_repo_ref_path(
    Path((owner, repo, reference, path)): Path<(String, String, String, String)>,
    headers: HeaderMap,
) -> Json<CompileResponse> {
    let plugin_options = match header::get_plugin_options(&headers) {
        Ok(v) => v,
        Err(e) => return Json(CompileResponse::Failure(e)),
    };
    let response = compile_internal(&owner, &repo, Some(&path), &reference, &plugin_options).await;
    Json(response)
}

async fn compile_internal(
    owner: &str,
    repo: &str,
    path: Option<&str>,
    reference: &str,
    plugin_options_json: &str,
) -> CompileResponse {
    let start_time = Instant::now();
    let prep_ctx = match compiler::get_context(owner, repo, path, reference).await {
        Ok(ctx) => ctx,
        Err(e) => return CompileResponse::Failure(e.to_string()),
    };

    let plugin_options = if plugin_options_json.is_empty() {
        None
    } else {
        match compiler::parse_plugin_options(plugin_options_json, &prep_ctx.project_res).await {
            Ok(options) => Some(options),
            Err(e) => return CompileResponse::Failure(e),
        }
    };

    let expo_ctx = compiler::compile(&prep_ctx, Some(start_time), plugin_options).await;
    let expo_ctx_json = match serde_json::to_value(expo_ctx) {
        Ok(v) => v,
        Err(e) => return CompileResponse::Failure(e.to_string()),
    };

    CompileResponse::Success(expo_ctx_json)
}
