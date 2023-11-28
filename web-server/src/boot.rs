//! Things to do on server boot

use celerc::macros::async_recursion;
use celerc::util;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures::future;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::{io, join};
use tracing::{debug, info};

/// Setup site origin in static html files
pub async fn setup_site_origin(
    docs_dir: PathBuf,
    app_dir: PathBuf,
    origin: &str,
) -> io::Result<()> {
    debug!("setting up site origin to {origin}");
    util::init_site_origin(origin.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let origin = util::get_site_origin();
    let domain = util::get_site_domain();
    let (r1, r2) = join!(
        process_site_origin_for_path(docs_dir, origin, domain),
        process_site_origin_for_path(app_dir, origin, domain)
    );
    r1?;
    r2?;
    Ok(())
}

#[async_recursion]
async fn process_site_origin_for_path(
    path: PathBuf,
    origin: &'static str,
    domain: &'static str,
) -> io::Result<()> {
    if path.is_dir() {
        let mut dir = tokio::fs::read_dir(path).await?;
        let mut futures = vec![];
        while let Some(entry) = dir.next_entry().await? {
            futures.push(process_site_origin_for_path(entry.path(), origin, domain));
        }
        for result in future::join_all(futures).await {
            result?;
        }
    } else if path
        .extension()
        .map(|ext| ext == "html" || ext == "js" || ext == "css")
        .unwrap_or(false)
    {
        process_site_origin_in_file(path.as_ref(), origin, domain).await?;
    }
    Ok(())
}

async fn process_site_origin_in_file(path: &Path, origin: &str, domain: &str) -> io::Result<()> {
    debug!("processing site origin for {}", path.display());
    let s = tokio::fs::read_to_string(path).await?;
    let new_s = s
        .replace("scheme://celer.placeholder.domain", origin)
        .replace("celer.placeholder.domain", domain);
    if s != new_s {
        info!("updating site origin in {}", path.display());
        tokio::fs::write(path, new_s).await?;
    }
    Ok(())
}

static GZIP_EXTS: &[&str] = &["html", "js", "css", "wasm"];

/// Gzip static assets whose extension is in GZIP_EXTS
pub async fn gzip_static_assets(docs_dir: PathBuf, app_dir: PathBuf) -> io::Result<()> {
    let (r1, r2) = join!(
        gzip_static_assets_for_path(docs_dir, GZIP_EXTS),
        gzip_static_assets_for_path(app_dir, GZIP_EXTS)
    );
    r1?;
    r2?;
    Ok(())
}

#[async_recursion]
async fn gzip_static_assets_for_path(path: PathBuf, exts: &'static [&str]) -> io::Result<()> {
    if path.is_dir() {
        let mut dir = tokio::fs::read_dir(path).await?;
        let mut futures = vec![];
        while let Some(entry) = dir.next_entry().await? {
            futures.push(gzip_static_assets_for_path(entry.path(), exts));
        }
        for result in future::join_all(futures).await {
            result?;
        }
    } else if path
        .extension()
        .map(|ext| exts.iter().any(|e| e == &ext))
        .unwrap_or(false)
    {
        gzip_file(path).await?;
    }
    Ok(())
}

async fn gzip_file(path: PathBuf) -> io::Result<()> {
    debug!("compressing: {}", path.display());
    let mut path_gz = path.clone();
    if let Some(ext) = path_gz.extension().and_then(|ext| ext.to_str()) {
        path_gz.set_extension(&format!("{ext}.gz"));
        if path_gz.exists() {
            debug!("skipping: {}", path.display());
            return Ok(());
        }
    }
    let bytes = tokio::fs::read(&path).await?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&bytes)?;
    let compressed_bytes = encoder.finish()?;
    tokio::fs::write(path_gz, compressed_bytes).await?;
    info!("compressed: {}", path.display());
    Ok(())
}
