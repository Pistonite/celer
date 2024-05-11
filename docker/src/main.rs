use std::{io::Cursor, path::PathBuf, process::Command, sync::Arc};

use clap::Parser;
use reqwest::Client;
use serde_json::Value;
use tokio::task::JoinSet;



#[derive(Debug, Parser)]
struct Args {

    /// The branch to download the artifact from
    ///
    /// Defaults to `git rev-parse --abbrev-ref HEAD`
    branch: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set! Need a PAT with repo read permission");

    let args: Args = Args::parse();
    let branch = match args.branch { 
        Some(branch) => branch.trim().to_string(),
        None => {
            println!("No branch provided, defaulting to `git rev-parse --abbrev-ref HEAD`");
            let branch = Command::new("git").args(["rev-parse", "--abbrev-ref", "HEAD"]).output()?;
            String::from_utf8_lossy(&branch.stdout).trim().to_string()
        }
    };

    println!("Using branch {}", branch);
    let rev = Command::new("git").args(["rev-parse", &branch]).output()?;
    let rev = String::from_utf8_lossy(&rev.stdout).trim().to_string();
    
    println!("Using rev {}", rev);

    let client = Client::new();
    let res = client.get("https://api.github.com/repos/Pistonite/celer/actions/artifacts")
        .header("User-Agent", "reqwest").send().await?;
    let value: Value = serde_json::from_slice(&res.bytes().await?)?;
    let artifacts = value.as_object().ok_or("expected object")?
        .get("artifacts").ok_or("expected `artifacts` in object")?
        .as_array().ok_or("expected `artifacts` to be array")?
        .iter().filter(|artifact| {
            match filter_artifact(artifact, &branch, &rev) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    false
                }
            }
        }).collect::<Vec<_>>();

    if artifacts.is_empty() {
        eprintln!("No matching artifacts found");
        return Ok(());
    } 
    if artifacts.len() > 3 {
        eprintln!("Need 3 artifacts, found {}", artifacts.len());
        return Ok(());
    }

    println!("Found {} artifacts", artifacts.len());

    let target_dir = PathBuf::from("dist");
    if target_dir.exists() {
        println!("Removing existing dist directory");
        std::fs::remove_dir_all(&target_dir)?;
    }
    std::fs::create_dir_all(&target_dir)?;

    let client = Arc::new(client);

    let mut join_set = JoinSet::new();

    for artifact in artifacts {
        let client = Arc::clone(&client);
        let token = token.clone();
        let artifact = artifact.clone();
        let target_dir = target_dir.clone();
        join_set.spawn( async move {
            let r = download_artifact(artifact, client, token, target_dir).await;
            r.is_ok()
        });
    }

    while let Some(result) = join_set.join_next().await {
        let r = result?;
        if !r {
            eprintln!("Error downloading artifact");
            return Ok(());
        }
    }
    
    println!("Done");

    Ok(())
}

fn filter_artifact(artifact: &Value, branch: &str, rev: &str) -> Result<bool, &'static str> {
    let workflow_run = artifact.as_object().ok_or("expected artifact object")?
        .get("workflow_run").ok_or("expected `workflow_run` in object")?
        .as_object().ok_or("expected `workflow_run` to be object")?;

    let head_branch = workflow_run.get("head_branch").ok_or("expected `head_branch` in object")?
        .as_str().ok_or("expected `head_branch` to be string")?;

    if head_branch != branch {
        return Ok(false);
    }

    let head_sha = workflow_run.get("head_sha").ok_or("expected `head_sha` in object")?
        .as_str().ok_or("expected `head_sha` to be string")?;

    if head_sha != rev {
        return Ok(false);
    }

    Ok(true)
}

async fn download_artifact(artifact: Value, client: Arc<Client>, token: String, mut target_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let name = artifact
        .get("name").ok_or("expected `name` in object")?
        .as_str().ok_or("expected `name` to be string")?;

    println!("Downloading artifact {}", name);

    let download_url = artifact
        .get("archive_download_url").ok_or("expected `archive_download_url` in object")?
        .as_str().ok_or("expected `archive_download_url` to be string")?;

    println!("Downloading {}", download_url);

    let res = client.get(download_url)
        .header("User-Agent", "reqwest")
        .header("Authorization", format!("Bearer {}", token))
        .send().await?;
    if res.status() == 410 {
        return Err("Artifact has expired".into());
    }

    let bytes = res.bytes().await?;

    target_dir.push(name);
    
    println!("Extracting to {}", target_dir.display());
    zip_extract::extract(Cursor::new(bytes), &target_dir, false)?;

    Ok(())
}
