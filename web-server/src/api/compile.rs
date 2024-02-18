use axum::extract::Path;
use cached::proc_macro::cached;
// use celerc::types::ExecDoc;

pub async fn compile_owner_repo(Path((_owner, _repo)): Path<(String, String)>) -> String {
    // TODO #25: implement this
    "".to_string()
}

pub async fn compile_owner_repo_path(
    Path((_owner, _repo, _path)): Path<(String, String, String)>,
) -> String {
    // TODO #25: implement this
    "".to_string()
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
    time=301,
    key="String",
    convert=r#"{ cache_key!(_owner, _repo, _path, _reference) }"#,
    sync_writes=true
)]
async fn get_context(owner: &str, repo: &str, path: Option<&str>, reference: Option<&str>) -> PreparedContext<$0> {
    // create root resource (root of repo)
    // load project entry points
    //
    // compile and return
    // TODO #25: implement this
    "".to_string()
}



