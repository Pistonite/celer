//! Build wasm package and copy it to the web-client directory.

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

const OUTPUT_DIR: &str = "../web-client/src/low/celerc";

pub fn main() {
    match main_internal() {
        Ok(_) => println!("done!"),
        Err(e) => eprintln!("error: {}", e),
    }
}

fn main_internal() -> io::Result<()> {
    wasm_pack_build()?;
    override_typescript_definitions()?;
    add_console_log()?;
    Ok(())
}

fn wasm_pack_build() -> io::Result<()> {
    if let Err(e) = fs::remove_dir_all(OUTPUT_DIR) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e);
        }
    }

    let mut command = build_wasm_pack_command();
    let result = command
        .spawn()?
        .wait_with_output()?;

    if !result.status.success() {
        eprintln!("wasm-pack build finished with error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "wasm-pack build failed",
        ));
    } else {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }

    Ok(())
}

#[cfg(debug_assertions)]
fn build_wasm_pack_command() -> Command {
    let mut command = Command::new("wasm-pack");
    command.args(&["build", "--out-dir", OUTPUT_DIR, "--dev"]);
    command
}

#[cfg(not(debug_assertions))]
fn build_wasm_pack_command() -> Command {
    let mut command = Command::new("wasm-pack");
    command.args(&["build", "--out-dir", OUTPUT_DIR, "--release"]);
    command
}

fn override_typescript_definitions() -> io::Result<()> {
    println!("generating typescript definitions");
    let mut d_ts = celercwasm::generate_d_ts_imports();
    d_ts.push_str(&celercwasm::generate_d_ts());
    fs::write(Path::new(OUTPUT_DIR).join("celercwasm.d.ts"), d_ts)?;
    Ok(())
}

fn add_console_log() -> io::Result<()> {
    // open file for appending
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(Path::new(OUTPUT_DIR).join("celercwasm.js"))?;

    // write to file
    writeln!(file, "console.log(\"loading compiler module\");")?;

    Ok(())
}
