#![deny(clippy::all, clippy::pedantic, warnings)]

mod comprehensive;
mod schema_evolution;
mod types;

use std::io;

fn main() -> io::Result<()> {
    println!("Running comprehensive integration test\u{2026}\n");
    comprehensive::run()?;

    println!("\nRunning schema evolution integration test\u{2026}\n");
    schema_evolution::run()
}
