use std::{
    env,
    io::{BufRead, Write, stderr},
    path::{Path, PathBuf},
    process::Command,
};

const SCHEMA_PATH: &str = "../types/types.t";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let typical_manifest_path = manifest_dir.join("../../Cargo.toml");
    let typical_binary_path = manifest_dir.join(format!(
        "../../target/debug/typical{}",
        env::consts::EXE_SUFFIX
    ));

    let mut command = if typical_binary_path.is_file() {
        Command::new(&typical_binary_path)
    } else {
        let mut command = Command::new("cargo");
        command
            .arg("run")
            .arg("--quiet")
            .arg("--manifest-path")
            .arg(&typical_manifest_path)
            .arg("--");
        command
    };
    let output = command
        .arg("generate")
        .arg(SCHEMA_PATH)
        .arg("--list-schemas")
        .arg("--rust")
        .arg(Path::new(&out_dir).join("types.rs"))
        .output()
        .expect("Failed to run the local Typical generator.");

    stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    for line in output.stdout.lines().map(Result::unwrap) {
        if !line.is_empty() {
            println!("cargo:rerun-if-changed={line}");
        }
    }
}
