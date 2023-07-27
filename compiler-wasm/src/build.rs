//! Build wasm package and copy it to the web-client directory.

use std::fs;
use std::io;
use std::path::Path;
use std::process;

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
    Ok(())
}

fn wasm_pack_build() -> io::Result<()> {
    if let Err(e) = fs::remove_dir_all(OUTPUT_DIR) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e);
        }
    }

    let result = process::Command::new("wasm-pack")
        .args(&["build", "--out-dir", OUTPUT_DIR])
        .spawn()?
        .wait_with_output()?;

    if !result.status.success() {
        eprintln!("wasm-pack build finished with error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        return Err(io::Error::new(io::ErrorKind::Other, "wasm-pack build failed"));
    } else {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }

    Ok(())
}

fn override_typescript_definitions() -> io::Result<()>{
    println!("generating typescript definitions");
    let d_ts = celercwasm::generate_d_ts();
    fs::write(Path::new(OUTPUT_DIR).join("celercwasm.d.ts"), d_ts)?;
    Ok(())
}
