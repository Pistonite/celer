//! Generate the bindings and copy the typescript bindings to the web-client folder

use std::fs;
use std::io;
use std::path::Path;
use std::process;

const OUTPUT_FILE: &str = "../web-client/src/low/compiler.g.ts";

#[allow(dead_code)]
pub fn main() {
    match main_internal() {
        Ok(_) => println!("done!"),
        Err(e) => eprintln!("error: {}", e),
    }
}

fn main_internal() -> io::Result<()> {
    // run cargo test
    if let Err(e) = fs::remove_dir_all("bindings") {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e);
        }
    }
    generate_bindings()?;

    // scan the bindings 
    let mut output_chunks = vec![];
    for entry in fs::read_dir("bindings")? {
        let entry = entry?;
        let entry_path = entry.path();

        // copy the file to the web-client folder
        println!("transforming {}", entry_path.display());
        let transformed = transform_file(&entry_path)?;
        output_chunks.push(transformed);
    }

    // create the index file
    println!("emitting {OUTPUT_FILE}");
    output_chunks.sort_unstable();
    let mut output = r#"//! low/compiler
//!
//! Bindings generated from rust types
"#.to_owned();
    output.push_str(&output_chunks.join("\n"));
    fs::write(
        OUTPUT_FILE,
        output)?;

    // run prettier
    run_prettier()
}

fn generate_bindings() -> io::Result<()> {
    let result = process::Command::new("cargo")
        .args(&["test"])
        .spawn()?
        .wait_with_output()?;

    if !result.status.success() {
        eprintln!("cargo test finished with error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        return Err(io::Error::new(io::ErrorKind::Other, "cargo test failed"));
    } else {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }
    Ok(())
}

fn transform_file(file_path: &Path) -> io::Result<String> {
    let content = fs::read_to_string(file_path)?;
    let transformed = content.lines().filter_map(|line| {
        if line.starts_with("//") {
            return None;
        }
        if line.starts_with("import") {
            return None;
        }
        Some(line)
    }).collect::<Vec<_>>().join("\n");
    Ok(transformed)
}

fn run_prettier() -> io::Result<()> {
    println!("formatting with prettier");
    let result = process::Command::new("npm")
        .args(&["run", "fmt:compiler"])
        .current_dir("../web-client")
        .spawn()?
        .wait_with_output()?;

    if !result.status.success() {
        eprintln!("prettier finished with error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        return Err(io::Error::new(io::ErrorKind::Other, "prettier failed"));
    } else {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }
    Ok(())
}

