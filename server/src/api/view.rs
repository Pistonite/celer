//! The `/view` API endpoint to serve the celer viewer HTML with meta tags injected.

use std::fs;
use std::io;
use std::sync::OnceLock;

use axum::http::header;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::routing;
use axum::extract::Path;
use axum::Router;
use cached::proc_macro::cached;
use celerc::env;
use celerc::util;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::error;

use crate::compiler;

pub fn init_api(app_dir: &str) -> Result<Router, io::Error> {
    init_view_html(app_dir)?;

    let router = Router::new()
        .route("/:owner/:repo", routing::get(view_owner_repo))
        .route("/:owner/:repo/*path", routing::get(view_owner_repo_path))
        .layer(ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .layer(SetResponseHeaderLayer::overriding(
                header::CONTENT_TYPE,  HeaderValue::from_static("text/html;charset=utf-8")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                header::CACHE_CONTROL,  HeaderValue::from_static("public,max-age=600")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                header::EXPIRES,  HeaderValue::from_static("600")
            ))
        );

    Ok(router)
}

const SERVER_INJECTED_TAGS: &str = "<!-- SERVER_INJECTED_TAGS -->";

static VIEW_HTML_HEAD: OnceLock<String> = OnceLock::new();
fn get_head() -> Result<&'static str, StatusCode> {
    match VIEW_HTML_HEAD.get() {
        Some(x) => Ok(x),
        None => {
            error!("View html head not initialized!");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

static VIEW_HTML_TAIL: OnceLock<String> = OnceLock::new();
fn get_tail() -> Result<&'static str, StatusCode> {
    match VIEW_HTML_TAIL.get() {
        Some(x) => Ok(x),
        None => {
            error!("View html tail not initialized!");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn init_view_html(app_dir: &str) -> Result<(), io::Error> {
    let view_path = std::path::Path::new(app_dir).join("view.html").canonicalize()?;
    let view_html = fs::read_to_string(view_path)?;

    let mut split = view_html.split(SERVER_INJECTED_TAGS);
    match split.next() {
        Some(head) => {
            VIEW_HTML_HEAD.get_or_init(|| head.to_string());
        }
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "view.html missing head".to_string()));
        }
    };
    match split.next() {
        Some(tail) => {
            VIEW_HTML_TAIL.get_or_init(|| tail.to_string());
        }
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "view.html missing tail".to_string()));
        }
    };
    if split.next().is_some() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "view.html has multiple inject tags".to_string()));
    }

    Ok(())
}

async fn view_owner_repo(
    Path((owner, repo)): Path<(String, String)>,
) -> Result<String, StatusCode> {
    // try to separate repo:reference:remaining
    let mut repo_parts = repo.splitn(3, ':');
    let repo = match repo_parts.next() {
        Some(repo) => repo,
        None => return view_fallback(),
    };
    let reference = match repo_parts.next() {
        Some(reference) => reference,
        None => "main",
    };
    view_internal(&owner, repo, reference, "").await
}

async fn view_owner_repo_path(
    Path((owner, repo, path)): Path<(String, String, String)>,
) -> Result<String, StatusCode> {
    // try to separate path:reference:remaining
    let mut path_parts = path.splitn(3, ':');
    let path = match path_parts.next() {
        Some(path) => path,
        None => "",
    };
    let reference = match path_parts.next() {
        Some(reference) => reference,
        None => "main",
    };
    view_internal(&owner, &repo, reference, path).await
}

#[cached(
    size=128, 
    time=600,
    key= "String",
    convert=r#"{ format!("{owner}/{repo}/{reference}/{path}") }"#,
    result = true
)]
async fn view_internal(owner: &str, repo: &str, reference: &str, path: &str) -> Result<String, StatusCode> {
    let mut builder = compiler::new_context_builder(owner, repo, Some(reference));
    if !path.is_empty() {
        builder = builder.entry_point(Some(path.to_string()));
    }

    let metadata = match builder.get_metadata().await {
        Err(e) => {
            error!("Error getting metadata for project {owner}/{repo}/{reference}/{path}: {e}");
            return view_fallback();
        },
        Ok(metadata) => metadata,
    };
    let title = &metadata.title;
    let version = &metadata.version;

    let title = if title.is_empty() {
        "Celer Viewer".to_string()
    } else if version.is_empty() {
        title.to_string()
    } else {
        format!("{title} - {version}")
    };

    let description = {
        let mut repo_desc = format!("{owner}/{repo}");
        if !path.is_empty() {
            repo_desc.push('/');
            repo_desc.push_str(path);
        }
        if !reference.is_empty() {
            repo_desc.push_str(" (");
            repo_desc.push_str(reference);
            repo_desc.push(')');
        }
        format!("View {repo_desc} on Celer")
    };

    let view_url = {
        let mut url = format!("{}/view/{owner}/{repo}", env::get_site_origin());
        if !path.is_empty() {
            url.push('/');
            url.push_str(path);
        }
        if !reference.is_empty() {
            url.push(':');
            url.push_str(reference);
        }
        url
    };
    let title_tag = format!("<meta name=\"og:title\" content=\"{}\">", util::html_attr_escape(&title));
    let description_tag = format!("<meta name=\"og:description\" content=\"{}\">", util::html_attr_escape(&description));
    let url_tag = format!("<meta name=\"og:url\" content=\"{}\">", util::html_attr_escape(&view_url));

    let head = get_head()?;
    let tail = get_tail()?;

    let html = format!(
        "{head}
         {title_tag}
         {description_tag}
         {url_tag}
         {tail}"
    );

    Ok(html)
    
}

fn view_fallback() -> Result<String, StatusCode> {
    let head = get_head()?;
    let tail = get_tail()?;
    Ok(format!("{head}{tail}"))
}
