use std::{
    env,
    io::{stderr, stdout, Write},
    path::Path,
    process::Command,
};

const SCHEMA_PATH: &str = "types.t";

fn main() {
    println!("cargo:rerun-if-changed={}", SCHEMA_PATH);

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let output = Command::new("typical")
        .arg("generate")
        .arg(SCHEMA_PATH)
        .arg("--rust-out")
        .arg(Path::new(&out_dir).join("types.rs"))
        .output()
        .expect("Failed to run Typical. Is it installed?");

    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}
