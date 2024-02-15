//! Build wasm package and copy it to the web-client directory.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

/// Temporary output directory
const TEMP_OUTPUT_DIR: &str = "celerc";

const BINDING_TS: &str = "binding.g.ts";
const CELERCWASM_BG_WASM: &str = "celercwasm_bg.wasm";
const WORKER_JS: &str = "worker.js";

/// Output directory
const OUTPUT_PUBLIC_DIR: &str = "../../web-client/public/celerc";
const OUTPUT_LOW_DIR: &str = "../../web-client/src/low/celerc";

pub fn main() {
    match main_internal() {
        Ok(_) => println!("done!"),
        Err(e) => eprintln!("error: {}", e),
    }
}

fn main_internal() -> io::Result<()> {
    wasm_pack_build()?;
    process_typescript()?;

    println!("copying files");
    fs::create_dir_all(OUTPUT_PUBLIC_DIR)?;
    // there is some problem with wasm-bindgen (or tsify), where it doesn't generate
    // the correct bindings in debug mode
    if cfg!(debug_assertions) {
        println!("not copying generated bindings in debug mode. Please run in release mode if bindings are updated!");
    } else {
        fs::copy(
            Path::new(TEMP_OUTPUT_DIR).join(BINDING_TS),
            Path::new(OUTPUT_LOW_DIR).join(BINDING_TS),
        )?;
    }

    fs::copy(
        Path::new(TEMP_OUTPUT_DIR).join(WORKER_JS),
        Path::new(OUTPUT_PUBLIC_DIR).join(WORKER_JS),
    )?;

    fs::copy(
        Path::new(TEMP_OUTPUT_DIR).join(CELERCWASM_BG_WASM),
        Path::new(OUTPUT_PUBLIC_DIR).join(CELERCWASM_BG_WASM),
    )?;

    Ok(())
}

fn wasm_pack_build() -> io::Result<()> {
    if let Err(e) = fs::remove_dir_all(TEMP_OUTPUT_DIR) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e);
        }
    }

    let mut command = if cfg!(debug_assertions) {
        println!("building in debug mode");
        build_wasm_pack_command(&["--dev"])
    } else {
        println!("building in release mode");
        build_wasm_pack_command(&["--release"])
    };

    let result = command.spawn()?.wait_with_output()?;

    if !result.status.success() {
        eprintln!("wasm-pack build finished with error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "wasm-pack build failed",
        ));
    }
    println!("{}", String::from_utf8_lossy(&result.stdout));

    Ok(())
}

fn build_wasm_pack_command(extra_args: &[&str]) -> Command {
    let mut command = Command::new("wasm-pack");
    let out_dir = Path::new("build")
        .join(TEMP_OUTPUT_DIR)
        .display()
        .to_string();
    let mut args = vec!["build", "--out-dir", &out_dir, "--target", "no-modules"];
    args.extend(extra_args);
    command.args(args);
    command.current_dir("..");
    command
}

fn process_typescript() -> io::Result<()> {
    println!("generating typescript definitions");
    let path = Path::new(TEMP_OUTPUT_DIR).join("celercwasm.d.ts");
    let d_ts = fs::read_to_string(path)?;
    let d_ts = patch_d_ts(d_ts);
    let mut lines = d_ts.lines();
    let mut gen_lines = vec![];
    gen_lines.push(Cow::from(include_str!("./prelude.ts")));
    let mut funcs = vec![];
    let mut export_blocks = BTreeMap::new();
    let mut last_block: Option<&mut Vec<Cow<str>>> = None;

    while let Some(line) = lines.next() {
        if line == "declare namespace wasm_bindgen {" {
            for line in lines.by_ref() {
                if line == "}" {
                    break;
                }
                let line = line.strip_prefix('\t').expect("expecting namespace indent");
                match line.strip_prefix("export function ") {
                    None => {
                        if line.starts_with("export ") {
                            let key = match line.strip_prefix("export interface ") {
                                Some(key) => key,
                                None => match line.strip_prefix("export type ") {
                                    Some(key) => key,
                                    None => line,
                                },
                            };
                            if export_blocks
                                .insert(key.to_string(), vec![Cow::from(line)])
                                .is_some()
                            {
                                panic!("duplicate export block");
                            }
                            last_block = export_blocks.get_mut(key);
                        } else {
                            match last_block.as_mut() {
                                Some(x) => x.push(Cow::from(line)),
                                None => gen_lines.push(Cow::from(line)),
                            }
                        }
                    }
                    Some(fn_def) => {
                        let mut fn_def = fn_def.splitn(2, '(');
                        let fn_name = fn_def.next().expect("cannot get function name");
                        let fn_def = fn_def
                            .next()
                            .expect("cannot get function args and return type");
                        let fn_id = funcs.len();
                        funcs.push(fn_name);
                        let (args, return_type) = parse_function(fn_def);
                        let mut fn_gen = String::new();
                        fn_gen.push_str("export function ");
                        fn_gen.push_str(fn_name);
                        fn_gen.push('(');
                        if !args.is_empty() {
                            for arg in &args {
                                fn_gen.push_str(&arg.ident);
                                fn_gen.push_str(": ");
                                fn_gen.push_str(&arg.arg_type);
                                fn_gen.push_str(", ");
                            }
                            fn_gen.pop();
                            fn_gen.pop();
                        }
                        fn_gen.push_str("): ");
                        fn_gen.push_str(&return_type);
                        fn_gen.push_str(" {");
                        gen_lines.push(Cow::from(fn_gen));
                        let mut fn_gen = String::new();
                        fn_gen.push_str("    ");
                        if return_type != "void" {
                            fn_gen.push_str("return ");
                        }
                        fn_gen.push_str("callWorker(");
                        fn_gen.push_str(&fn_id.to_string());
                        fn_gen.push_str(", [");
                        if !args.is_empty() {
                            for arg in &args {
                                match arg.ident.strip_suffix('?') {
                                    Some(ident) => {
                                        fn_gen.push_str(ident);
                                    }
                                    None => {
                                        fn_gen.push_str(&arg.ident);
                                    }
                                }
                                fn_gen.push_str(", ");
                            }
                            fn_gen.pop();
                            fn_gen.pop();
                        }
                        fn_gen.push_str("]);");
                        gen_lines.push(Cow::from(fn_gen));
                        gen_lines.push(Cow::from("}"));
                    }
                }
            }
            break;
        }
    }

    for lines in export_blocks.into_values() {
        for line in lines {
            gen_lines.push(line);
        }
    }

    fs::write(
        Path::new(TEMP_OUTPUT_DIR).join(BINDING_TS),
        gen_lines.join("\n"),
    )?;

    build_worker(&funcs)?;
    Ok(())
}

fn patch_d_ts(d_ts: String) -> String {
    // https://github.com/madonoharu/tsify/issues/37
    // tsify doesn't quote properties starting with number correctly
    let d_ts = d_ts.replace("    2d: ", "    \"2d\": ");
    d_ts.replace("    3d: ", "    \"3d\": ")
}

fn parse_function(mut s: &str) -> (Vec<TsArg>, String) {
    let mut args = vec![];

    loop {
        if let Some(rest) = s.strip_prefix("): ") {
            let return_type = rest
                .strip_suffix(';')
                .expect("function definition must end with ;");
            return (args, return_type.to_string());
        }
        let mut rest = s.splitn(2, ':');
        let ident = rest.next().expect("cannot get identifier name");
        let rest = rest.next().expect("cannot get arg type");
        let rest = rest.strip_prefix(' ').expect("malformed function type");
        // wasm_bindgen output has no generic or functions, which makes parsing simpler
        let i_comma = rest.find(',');
        match i_comma {
            Some(i) => {
                let arg_type = rest[..i].to_string();
                args.push(TsArg {
                    ident: ident.to_string(),
                    arg_type,
                });
                s = &rest[i + 2..];
            }
            None => {
                let i_paren = rest.find(')').expect("malformed function type");
                let arg_type = rest[..i_paren].to_string();
                args.push(TsArg {
                    ident: ident.to_string(),
                    arg_type,
                });
                s = &rest[i_paren..];
            }
        }
    }
}

struct TsArg {
    ident: String,
    arg_type: String,
}

/// Build worker.js
fn build_worker(funcs: &[&str]) -> io::Result<()> {
    println!("generating worker.js");
    let mut worker_js = fs::read_to_string(Path::new(TEMP_OUTPUT_DIR).join("celercwasm.js"))?;
    worker_js.push_str("// worker_init.js\n");
    worker_js.push_str(include_str!("./worker_init.js"));
    worker_js.push_str("\n__initWorker([\n");
    for func in funcs {
        worker_js.push_str("    wasm_bindgen.");
        worker_js.push_str(func);
        worker_js.push_str(",\n");
    }
    worker_js.push_str("]);\n");
    fs::write(Path::new(TEMP_OUTPUT_DIR).join(WORKER_JS), worker_js)?;
    Ok(())
}
