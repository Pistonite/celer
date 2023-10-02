//! Things to do on server boot

use std::{path::Path};

use tokio::{fs::File, io};
use tracing::{info, debug};

/// Setup site origin in static html files
pub async fn setup_site_origin(docs_dir: &Path, app_dir: &Path, origin: &str) -> io::Result<()> {
    debug!("setting up site origin to {origin}");
    let domain = match origin.strip_prefix("https://") {
        Some(domain) => domain,
        None => origin.strip_prefix("http://").unwrap_or(origin),
    };
    process_site_origin_for_path(docs_dir, origin, domain).await?;
    process_site_origin_for_path(app_dir, origin, domain).await?;
    Ok(())
}

#[async_recursion::async_recursion]
async fn process_site_origin_for_path(path: &Path, origin: &str, domain: &str) -> io::Result<()> {
    if path.is_dir() {
        let mut dir = tokio::fs::read_dir(path).await?;
        while let Some(entry) = dir.next_entry().await? {
            process_site_origin_for_path(entry.path().as_ref(), origin, domain).await?;
        }
    } else if path.extension().map(|ext| ext == "html").unwrap_or(false) {
        process_site_origin_in_file(path, origin, domain).await?;
    }
    Ok(())
}

async fn process_site_origin_in_file(path: &Path, origin: &str, domain: &str) -> io::Result<()> {
    debug!("processing site origin for {}", path.display());
    let s = tokio::fs::read_to_string(path).await?;
    let new_s = s.replace("scheme://celer.placeholder.domain", origin).replace("celer.placeholder.domain", domain);
    if s != new_s {
        info!("updating site origin in {}", path.display());
        tokio::fs::write(path, new_s).await?;
    }
    Ok(())
}
