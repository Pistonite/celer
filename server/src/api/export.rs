
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::routing;
use axum::{Json, Router};
use celerc::{ExpoDoc, ExportRequest};
use instant::Instant;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

use crate::compiler;

use super::header;

pub fn init_api() -> Router {
    Router::new()
        .route(
            "/:owner/:repo/:reference",
            routing::get(export_owner_repo_ref),
        )
        .route(
            "/:owner/:repo/:reference/*path",
            routing::get(export_owner_repo_ref_path),
        )
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
}

async fn export_owner_repo_ref(
    Path((owner, repo, reference)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> Json<ExpoDoc> {
    let plugin_options = match header::get_plugin_options(&headers) {
        Ok(v) => v,
        Err(e) => return Json(ExpoDoc::Error(e)),
    };
    let req = match header::get_export_request(&headers) {
        Ok(v) => v,
        Err(e) => return Json(ExpoDoc::Error(e)),
    };
    let response = export_internal(&owner, &repo, None, &reference, &plugin_options, req).await;
    Json(response)
}

async fn export_owner_repo_ref_path(
    Path((owner, repo, reference, path)): Path<(String, String, String, String)>,
    headers: HeaderMap,
) -> Json<ExpoDoc> {
    let plugin_options = match header::get_plugin_options(&headers) {
        Ok(v) => v,
        Err(e) => return Json(ExpoDoc::Error(e)),
    };
    let req = match header::get_export_request(&headers) {
        Ok(v) => v,
        Err(e) => return Json(ExpoDoc::Error(e)),
    };
    let response = export_internal(&owner, &repo, Some(&path), &reference, &plugin_options, req).await;
    Json(response)
}
async fn export_internal(
    owner: &str,
    repo: &str,
    path: Option<&str>,
    reference: &str,
    plugin_options_json: &str,
    req: ExportRequest
) -> ExpoDoc {
    let start_time = Instant::now();
    let prep_ctx = match compiler::get_context(owner, repo, path, reference).await {
        Ok(ctx) => ctx,
        Err(e) => return ExpoDoc::Error(e.to_string()),
    };

    let plugin_options = if plugin_options_json.is_empty() {
        None
    } else {
        match compiler::parse_plugin_options(plugin_options_json, &prep_ctx.project_res).await {
            Ok(options) => Some(options),
            Err(e) => return ExpoDoc::Error(e),
        }
    };

    compiler::export(&prep_ctx, Some(start_time), plugin_options, req).await
}
