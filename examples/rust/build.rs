use std::{
    env,
    io::{stderr, BufRead, Write},
    path::Path,
    process::Command,
};

const SCHEMA_PATH: &str = "types.t";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let output = Command::new("typical")
        .arg("generate")
        .arg(SCHEMA_PATH)
        .arg("--list-schemas")
        .arg("--rust")
        .arg(Path::new(&out_dir).join("types.rs"))
        .output()
        .expect("Failed to run Typical. Is it installed?");

    stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    for line in output.stdout.lines().map(Result::unwrap) {
        if !line.is_empty() {
            println!("cargo:rerun-if-changed={line}");
        }
    }
}
