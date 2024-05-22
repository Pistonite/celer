use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

const VERSION_FILE: &str = "./VERSION";

fn main() {
    let version = std::fs::read_to_string(VERSION_FILE)
        .map(|x| x.trim().to_string())
        .unwrap_or_else(|_| "0.0.0-dev unknown".to_string());
    println!("[boot] server version is {version}");

    let server_executable = std::env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(|x| x.parent())
        .map(|x| x.join("celery"))
        .expect("fail to get server executable path");

    if !server_executable.exists() {
        println!("[boot] server executable not found!");
        return;
    }

    let mut status: ExitStatus;
    println!("[boot] starting server...");

    loop {
        status = Command::new(&server_executable)
            .env("CELERSERVER_VERSION", version.clone())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("fail to spawn server process");

        if !status.success() {
            break;
        }

        // on success status (graceful normal shutdown), reboot the server
        println!("Restarting server...")
    }

    println!("Server exited with status: {}", status);
}
