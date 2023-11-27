use axum::extract::Path;
use cached::proc_macro::cached;
use celerc::types::ExecDoc;

pub async fn compile_owner_repo(Path((owner, repo)): Path<(String, String)>) -> String {
    todo!()
}

pub async fn compile_owner_repo_path(
    Path((owner, repo, path)): Path<(String, String, String)>,
) -> String {
    todo!()
}

/// Separate string into before `:` and after
fn separate_ref(path: &str) -> (&str, Option<&str>) {
    let mut parts = path.splitn(2, ':');
    let path = parts.next().unwrap_or("");
    (path, parts.next())
}

macro_rules! cache_key {
    ($owner:ident, $repo:ident, $path:ident, $reference:ident) => {
        {
            let r = $reference.clone().unwrap_or("main");
            let owner = $owner;
            let repo = $repo;
            match $path.as_ref() {
                Some(x) => format!("{owner}/{repo}/{r}/{x}"),
                None => format!("{owner}/{repo}/{r}"),
            }
        }
    }
}

#[cached(
    size=32,
    time=600,
    key="String",
    convert=r#"{ cache_key!(owner, repo, path, reference) }"#
)]
async fn compile(owner: &str, repo: &str, path: Option<&str>, reference: Option<&str>) -> String {
}



