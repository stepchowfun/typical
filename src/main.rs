mod assertions;
mod count;
mod error;
mod error_merger;
mod format;
mod generate_rust;
mod generate_typescript;
mod identifier;
mod parser;
mod schema;
mod schema_loader;
mod token;
mod tokenizer;
mod validator;

use crate::{
    count::count,
    error::{Error, throw},
    error_merger::merge_errors,
    format::CodeStr,
    schema_loader::load_schemas,
    validator::validate,
};
use clap::{ArgAction, Args, CommandFactory, Parser, Subcommand as ClapSubcommand};
use clap_complete::{Shell, generate};
use std::{
    fs::{create_dir_all, write},
    io::stdout,
    path::Path,
    process::exit,
};

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// The name of the program binary
const BIN_NAME: &str = "typical";

// This struct represents the command-line arguments.
#[derive(Parser)]
#[command(
    about = concat!(
        env!("CARGO_PKG_DESCRIPTION"),
        "\n\n",
        "More information can be found at: ",
        env!("CARGO_PKG_HOMEPAGE")
    ),
    version,
    disable_version_flag = true,
)]
struct Cli {
    #[arg(short, long, help = "Print version", action = ArgAction::Version)]
    _version: Option<bool>,

    #[command(subcommand)]
    command: TypicalCommand,
}

#[derive(Args)]
struct GenerateArgs {
    #[arg(value_name = "SCHEMA_PATH", help = "Set the path to the schema")]
    path: String,

    #[arg(
        long,
        help = "List the schemas imported by the given schema (and the given schema itself)"
    )]
    list_schemas: bool,

    #[arg(
        long,
        value_name = "PATH",
        help = "Set the path to the Rust file to emit"
    )]
    rust: Option<String>,

    #[arg(
        long,
        value_name = "PATH",
        help = "Set the path to the TypeScript file to emit"
    )]
    typescript: Option<String>,
}

#[derive(Args)]
struct FormatArgs {
    #[arg(value_name = "SCHEMA_PATH", help = "Set the path to the schema")]
    path: String,

    #[arg(long, help = "Check the formatting rather than actually doing it")]
    check: bool,
}

#[derive(Args)]
struct ShellCompletionArgs {
    #[arg(help = "Bash, Fish, Zsh, PowerShell, or Elvish")]
    shell: String,
}

#[derive(ClapSubcommand)]
enum TypicalCommand {
    #[command(about = "Generate code for a schema and its transitive dependencies")]
    Generate(GenerateArgs),

    #[command(about = "Format a schema and its transitive dependencies")]
    Format(FormatArgs),

    #[command(
        name = "shell-completion",
        about = "Print a shell completion script. Supports Bash, Fish, Zsh, PowerShell, and Elvish."
    )]
    ShellCompletion(ShellCompletionArgs),
}

// Generate code for a schema and its transitive dependencies.
fn generate_code(
    path: &Path,
    list_schemas: bool,
    rust: Option<&Path>,
    typescript: Option<&Path>,
) -> Result<(), Error> {
    // Load the schema and its transitive dependencies.
    eprintln!("Loading schemas\u{2026}");
    let schemas = load_schemas(path).map_err(|errors| merge_errors(&errors))?;
    eprintln!("{} loaded.", count(schemas.len(), "schema"));

    // Validate the schemas.
    eprintln!("Validating schemas\u{2026}");
    validate(&schemas).map_err(|errors| merge_errors(&errors))?;

    // Print the schema paths, if applicable.
    if list_schemas {
        eprintln!("Listing schemas\u{2026}");

        // The `unwrap` is safe since otherwise the schema would've failed to load above.
        let directory = path.parent().unwrap();

        for (_, source_path, _) in schemas.values() {
            println!("{}", directory.join(source_path).to_string_lossy());
        }
    }

    // Generate Rust code, if applicable.
    if let Some(rust) = rust {
        eprintln!("Generating Rust\u{2026}");

        // Create any missing intermediate directories as needed.
        if let Some(parent) = rust.parent() {
            create_dir_all(parent).map_err(|error| {
                throw(
                    &format!("Unable to create {}.", parent.to_string_lossy().code_str()),
                    None,
                    None,
                    Some(error),
                )
            })?;
        }

        // Generate the code and write it to the file.
        eprintln!("Writing {}\u{2026}", rust.to_string_lossy().code_str());
        write(rust, generate_rust::generate(VERSION, &schemas)).map_err(|error| {
            throw(
                &format!("Unable to write {}.", rust.to_string_lossy().code_str()),
                None,
                None,
                Some(error),
            )
        })?;
    }

    // Generate TypeScript code, if applicable.
    if let Some(typescript) = typescript {
        eprintln!("Generating TypeScript\u{2026}");

        // Create any missing intermediate directories as needed.
        if let Some(parent) = typescript.parent() {
            create_dir_all(parent).map_err(|error| {
                throw(
                    &format!("Unable to create {}.", parent.to_string_lossy().code_str()),
                    None,
                    None,
                    Some(error),
                )
            })?;
        }

        // Generate the code and write it to the file.
        eprintln!(
            "Writing {}\u{2026}",
            typescript.to_string_lossy().code_str(),
        );
        write(typescript, generate_typescript::generate(VERSION, &schemas)).map_err(|error| {
            throw(
                &format!(
                    "Unable to write {}.",
                    typescript.to_string_lossy().code_str(),
                ),
                None,
                None,
                Some(error),
            )
        })?;
    }

    eprintln!("Done.");
    Ok(())
}

// Format a schema and its transitive dependencies.
fn format_schema(path: &Path, check: bool) -> Result<(), Error> {
    // Load the schema and its transitive dependencies.
    eprintln!("Loading schemas\u{2026}");
    let schemas = load_schemas(path).map_err(|errors| merge_errors(&errors))?;
    eprintln!("{} loaded.", count(schemas.len(), "schema"));

    // This flag will be set if any changes were made to any of the schemas.
    let mut any_schema_updated = false;

    // Compute the base directory for the schemas. The `unwrap` is safe since otherwise the schema
    // would've failed to load above.
    let directory = path.parent().unwrap();

    // Format the schemas.
    eprintln!(
        "{} schemas\u{2026}",
        if check { "Checking" } else { "Formatting" },
    );
    for (schema, source_path, source_contents) in schemas.values() {
        // Compute the full path and new contents of the schema.
        let full_source_path = directory.join(source_path);
        let new_source_contents = schema.to_string();
        eprintln!("  {}", full_source_path.to_string_lossy().code_str());

        // Check if the contents changed.
        let updated = *source_contents != new_source_contents;
        if updated {
            any_schema_updated = true;
        }

        // Write the updated schema contents, if applicable.
        if updated && !check {
            write(&full_source_path, new_source_contents).map_err(|error| {
                throw(
                    "Unable to write file.",
                    Some(source_path),
                    None,
                    Some(error),
                )
            })?;
        }
    }

    // If the user only wants to check the formatting, fail if any of the schemas need to be
    // formatted.
    if check && any_schema_updated {
        return Err(throw::<Error>(
            &format!(
                "Formatting mismatch. Please run {}.",
                format!("typical format {}", path.to_string_lossy()).code_str(),
            ),
            None,
            None,
            None,
        ));
    }

    eprintln!("Done.");
    Ok(())
}

// Print a shell completion script to STDOUT.
fn shell_completion(shell: &str) -> Result<(), Error> {
    // Determine which shell the user wants the shell completion for.
    let shell_variant = match shell.trim().to_lowercase().as_ref() {
        "bash" => Shell::Bash,
        "fish" => Shell::Fish,
        "zsh" => Shell::Zsh,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            return Err(Error {
                message: format!(
                    "Unknown shell {}. Must be one of Bash, Fish, Zsh, PowerShell, or Elvish.",
                    shell.code_str(),
                ),
                reason: None,
            });
        }
    };

    // Write the script to STDOUT.
    let mut command = Cli::command();
    generate(shell_variant, &mut command, BIN_NAME, &mut stdout());

    // If we made it this far, nothing went wrong.
    Ok(())
}

// Program entrypoint
fn entry() -> Result<(), Error> {
    // Parse command-line arguments.
    let cli = Cli::parse();

    // Decide what to do based on the subcommand.
    match cli.command {
        TypicalCommand::Generate(args) => {
            // Generate code for the schema and its transitive dependencies.
            generate_code(
                Path::new(&args.path),
                args.list_schemas,
                args.rust.as_deref().map(Path::new),
                args.typescript.as_deref().map(Path::new),
            )?;
        }
        TypicalCommand::Format(args) => {
            // Format the schema and its transitive dependencies.
            format_schema(Path::new(&args.path), args.check)?;
        }
        TypicalCommand::ShellCompletion(args) => {
            // Generate the shell completion script.
            shell_completion(&args.shell)?;
        }
    }

    // If we made it this far, nothing went wrong.
    Ok(())
}

// Let the fun begin!
fn main() {
    // Jump to the entrypoint and report any resulting errors.
    if let Err(e) = entry() {
        eprintln!("{e}");
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
