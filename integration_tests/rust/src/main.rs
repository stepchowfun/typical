#![deny(clippy::all, clippy::pedantic, warnings)]

mod circular_dependency;
mod comprehensive;
mod degenerate;
mod round_trip;
mod schema_evolution;
mod types;

use std::io;

fn main() -> io::Result<()> {
    println!("Running circular dependency test\u{2026}\n");
    circular_dependency::run()?;

    println!("Running comprehensive integration test\u{2026}\n");
    comprehensive::run()?;

    println!("Running degenerate integration test\u{2026}\n");
    degenerate::run()?;

    println!("\nRunning schema evolution integration test\u{2026}\n");
    schema_evolution::run()
}
