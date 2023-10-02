//! Things to do on server boot

use std::sync::Arc;
use std::{path::Path};

use tokio::task::JoinSet;
use tokio::{fs::File, io};
use tokio::join;
use tracing::{info, debug};

/// Setup site origin in static html files
pub async fn setup_site_origin(docs_dir: &Path, app_dir: &Path, origin: &str) -> io::Result<()> {
    debug!("setting up site origin to {origin}");
    let domain = match origin.strip_prefix("https://") {
        Some(domain) => domain,
        None => origin.strip_prefix("http://").unwrap_or(origin),
    };
    let (r1, r2) = join!(
    process_site_origin_for_path(docs_dir, origin, Arc::from(domain)),
    process_site_origin_for_path(app_dir, origin, Arc::from(domain)));
    r1?;
    r2?;
    Ok(())
}

#[async_recursion::async_recursion]
async fn process_site_origin_for_path(path: &Path, origin: &str, domain: Arc<str>) -> io::Result<()> {
    if path.is_dir() {
        let mut dir = tokio::fs::read_dir(path).await?;
        let mut join_set = JoinSet::new();
        while let Some(entry) = dir.next_entry().await? {
            // let origin = Arc::clone(&origin);
            let domain = Arc::clone(&domain);
            let path = entry.path();
            join_set.spawn( async move {
                process_site_origin_for_path(path.as_ref(), origin, domain).await
            });
        }
        while let Some(r) = join_set.join_next().await {
            r?;
        }
    } else if path.extension().map(|ext| ext == "html").unwrap_or(false) {
        process_site_origin_in_file(path, &origin, &domain).await?;
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
