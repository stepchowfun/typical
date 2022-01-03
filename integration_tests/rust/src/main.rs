#![deny(clippy::all, clippy::pedantic, warnings)]

mod assertions;
mod circular_dependency;
mod comprehensive;
mod degenerate;
mod schema_evolution;
mod types;

use std::io;

fn main() -> io::Result<()> {
    println!("Running circular dependency integration test\u{2026}\n");
    circular_dependency::run()?;

    println!("\nRunning comprehensive integration test\u{2026}\n");
    comprehensive::run()?;

    println!("\nRunning degenerate integration test\u{2026}\n");
    degenerate::run()?;

    println!("\nRunning schema evolution integration test\u{2026}\n");
    schema_evolution::run()
}
