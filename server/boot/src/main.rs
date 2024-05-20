use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

const VERSION_FILE: &str = "./VERSION";
const CELERY_BIN: &str = "./celery";

fn main() {
    let version = std::fs::read_to_string(VERSION_FILE)
        .map(|x| x.trim().to_string())
        .unwrap_or_else(|_| "0.0.0-dev unknown".to_string());

    if !Path::new(CELERY_BIN).exists() {
        println!("Server executable not found!");
        return;
    }

    let mut status: ExitStatus;
    println!("Starting server...");

    loop {
        status = Command::new(CELERY_BIN)
            .env("CELERSERVER_VERSION", version.clone())
            .env("CELERSERVER_BOOTSTRAP", "true")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Fail to spawn server process");

        if !status.success() {
            break;
        }

        // on success status (graceful normal shutdown), reboot the server
        println!("Restarting server...")
    }

    println!("Server exited with status: {}", status);
}
