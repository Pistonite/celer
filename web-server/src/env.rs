//! Server environment settings
use axum_server::tls_rustls::RustlsConfig;
use envconfig::Envconfig;
use once_cell::sync::Lazy;
use std::env;
use std::path::Path;
use tracing::{error, info, Level};

#[derive(Envconfig)]
pub struct Environment {
    /// Logging level
    #[envconfig(from = "CELERSERVER_LOG", default = "INFO")]
    pub logging_level: Level,

    #[envconfig(from = "CELERSERVER_PORT", default = "8173")]
    /// Port to listen on
    pub port: u16,

    /// If ANSI formatting is enabled in logs
    #[envconfig(from = "CELERSERVER_ANSI", default = "true")]
    pub ansi: bool,

    #[envconfig(from = "CELERSERVER_DOCS_DIR")]
    /// Directory to serve docs
    pub docs_dir: String,

    #[envconfig(from = "CELERSERVER_APP_DIR")]
    /// Directory to serve web client
    pub app_dir: String,

    /// Site origin of the server (e.g. https://celer.example.com)
    #[envconfig(from = "CELERSERVER_SITE_ORIGIN")]
    pub site_origin: String,

    /// Enable gzip compression for static assets
    #[envconfig(from = "CELERSERVER_GZIP", default = "false")]
    pub gzip: bool,

    #[envconfig(from = "CELERSERVER_HTTPS_CERT")]
    cert_path: Option<String>,

    #[envconfig(from = "CELERSERVER_HTTPS_KEY")]
    key_path: Option<String>,
}

impl Environment {
    /// Parse environment from command line arguments and environment variables
    pub fn parse() -> Self {
        let mut env = match Environment::init_from_env() {
            Ok(env) => env,
            Err(envconfig::Error::EnvVarMissing { name }) => {
                panic!("Server cannot start due to missing environment variable: {name}");
            }
            Err(envconfig::Error::ParseError { name }) => {
                panic!(
                    "Server cannot start due to failure when parsing environment variable: {name}"
                );
            }
        };

        for arg in env::args() {
            if arg == "--debug" {
                env.logging_level = Level::DEBUG;
            }
        }

        env
    }

    pub async fn get_https_config(&self) -> Option<RustlsConfig> {
        let (cert_path, key_path) = match (&self.cert_path, &self.key_path) {
            (Some(cert_path), Some(key_path)) => (cert_path, key_path),
            _ => {
                info!("No certificate path and key path provided, starting server in http mode. Set the environment variable CELERSERVER_HTTPS_CERT and CELERSERVER_HTTPS_KEY to the cert and key pem file to enable https");
                return None;
            }
        };

        let cert_path = Path::new(cert_path)
            .canonicalize()
            .map_err(|e| {
                error!("failed to access certificate path for https: {e}");
                error!("falling back to http");
            })
            .ok()?;

        let key_path = Path::new(key_path)
            .canonicalize()
            .map_err(|e| {
                error!("failed to access certificate key path for https: {e}");
                error!("falling back to http");
            })
            .ok()?;

        RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .map_err(|e| {
                error!("failed to load certificate for https: {e}");
                error!("falling back to http");
            })
            .ok()
    }
}

static VERSION: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string("VERSION")
        .map(|x| x.trim().to_string())
        .unwrap_or_else(|_| "0.0.0-dev unknown".to_string())
});

/// Get the version of the server
///
/// The server version is read from the file `VERSION` in the current working directory
pub fn version() -> &'static str {
    &VERSION
}
