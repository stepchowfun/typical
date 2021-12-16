#![deny(clippy::all, clippy::pedantic, warnings)]

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

use {
    crate::{
        count::count,
        error::{throw, Error},
        error_merger::merge_errors,
        format::CodeStr,
        schema_loader::load_schemas,
        validator::validate,
    },
    clap::{App, AppSettings, Arg, Shell, SubCommand},
    std::{
        fs::{create_dir_all, write},
        io::stdout,
        path::Path,
        process::exit,
    },
};

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// The name of the program binary
const BIN_NAME: &str = "typical";

// Command-line option and subcommand names
const GENERATE_SUBCOMMAND: &str = "generate";
const GENERATE_SUBCOMMAND_PATH_OPTION: &str = "generate-path";
const GENERATE_SUBCOMMAND_RUST_OPTION: &str = "rust";
const GENERATE_SUBCOMMAND_TYPESCRIPT_OPTION: &str = "typescript";
const GENERATE_SUBCOMMAND_LIST_SCHEMAS_OPTION: &str = "list-schemas";
const FORMAT_SUBCOMMAND: &str = "format";
const FORMAT_SUBCOMMAND_PATH_OPTION: &str = "format-path";
const FORMAT_SUBCOMMAND_CHECK_OPTION: &str = "check";
const SHELL_COMPLETION_SUBCOMMAND: &str = "shell-completion";
const SHELL_COMPLETION_SUBCOMMAND_SHELL_OPTION: &str = "shell-completion-shell";

// Set up the command-line interface.
fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Typical")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about("Algebraic data types for data interchange.")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::NextLineHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp) // [tag:subcommand_required_else_help]
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name(GENERATE_SUBCOMMAND)
                .about("Generate code for a schema and its transitive dependencies")
                .arg(
                    Arg::with_name(GENERATE_SUBCOMMAND_PATH_OPTION)
                        .value_name("SCHEMA_PATH")
                        .help("Sets the path of the schema")
                        .required(true), // [tag:generate_subcommand_path_required]
                )
                .arg(
                    Arg::with_name(GENERATE_SUBCOMMAND_LIST_SCHEMAS_OPTION)
                        .long(GENERATE_SUBCOMMAND_LIST_SCHEMAS_OPTION)
                        .help(
                            "Lists the schemas imported by the given schema (and the given schema \
                            itself)",
                        ),
                )
                .arg(
                    Arg::with_name(GENERATE_SUBCOMMAND_RUST_OPTION)
                        .value_name("PATH")
                        .long(GENERATE_SUBCOMMAND_RUST_OPTION)
                        .help("Sets the path of the Rust file to emit"),
                )
                .arg(
                    Arg::with_name(GENERATE_SUBCOMMAND_TYPESCRIPT_OPTION)
                        .value_name("PATH")
                        .long(GENERATE_SUBCOMMAND_TYPESCRIPT_OPTION)
                        .help("Sets the path of the TypeScript file to emit"),
                ),
        )
        .subcommand(
            SubCommand::with_name(FORMAT_SUBCOMMAND)
                .about("Format a schema and its transitive dependencies")
                .arg(
                    Arg::with_name(FORMAT_SUBCOMMAND_PATH_OPTION)
                        .value_name("SCHEMA_PATH")
                        .help("Sets the path of the schema")
                        .required(true), // [tag:format_subcommand_path_required]
                )
                .arg(
                    Arg::with_name(FORMAT_SUBCOMMAND_CHECK_OPTION)
                        .long(FORMAT_SUBCOMMAND_CHECK_OPTION)
                        .help("Check the formatting rather than actually doing it"),
                ),
        )
        .subcommand(
            SubCommand::with_name(SHELL_COMPLETION_SUBCOMMAND)
                .about(
                    " \
                     Prints a shell completion script. Supports Zsh, Fish, Zsh, PowerShell, and \
                     Elvish. \
                     "
                    .trim(),
                )
                .arg(
                    Arg::with_name(SHELL_COMPLETION_SUBCOMMAND_SHELL_OPTION)
                        .value_name("SHELL")
                        .help("Bash, Fish, Zsh, PowerShell, or Elvish")
                        .required(true), // [tag:shell_completion_subcommand_shell_required]
                ),
        )
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
    eprintln!("Formatting schemas\u{2026}");
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
    cli().gen_completions_to(BIN_NAME, shell_variant, &mut stdout());

    // If we made it this far, nothing went wrong.
    Ok(())
}

// Program entrypoint
fn entry() -> Result<(), Error> {
    // Parse command-line arguments.
    let matches = cli().get_matches();

    // Decide what to do based on the subcommand.
    match matches.subcommand_name() {
        // [tag:generate_subcommand]
        Some(subcommand) if subcommand == GENERATE_SUBCOMMAND => {
            // Get the subcommand matches. The `unwrap` is safe due to [ref:generate_subcommand].
            let subcommand_matches = matches.subcommand_matches(GENERATE_SUBCOMMAND).unwrap();

            // Determine the path to the schema file.
            let path = Path::new(
                subcommand_matches
                    .value_of(GENERATE_SUBCOMMAND_PATH_OPTION)
                    // [ref:generate_subcommand_path_required]
                    .unwrap(),
            );

            // Determine if the user wants to list the schemas.
            let list_schemas =
                subcommand_matches.is_present(GENERATE_SUBCOMMAND_LIST_SCHEMAS_OPTION);

            // Determine the path to the Rust output file, if provided.
            let rust = subcommand_matches
                .value_of(GENERATE_SUBCOMMAND_RUST_OPTION)
                .map(Path::new);

            // Determine the path to the TypeScript output file, if provided.
            let typescript = subcommand_matches
                .value_of(GENERATE_SUBCOMMAND_TYPESCRIPT_OPTION)
                .map(Path::new);

            // Generate code for the schema and its transitive dependencies.
            generate_code(path, list_schemas, rust, typescript)?;
        }

        // [tag:format_subcommand]
        Some(subcommand) if subcommand == FORMAT_SUBCOMMAND => {
            // Get the subcommand matches. The `unwrap` is safe due to [ref:format_subcommand].
            let subcommand_matches = matches.subcommand_matches(FORMAT_SUBCOMMAND).unwrap();

            // Determine the path to the schema file.
            let path = Path::new(
                subcommand_matches
                    .value_of(FORMAT_SUBCOMMAND_PATH_OPTION)
                    // [ref:format_subcommand_path_required]
                    .unwrap(),
            );

            // Determine if the user wants to check the formatting.
            let check = subcommand_matches.is_present(FORMAT_SUBCOMMAND_CHECK_OPTION);

            // Format the schema and its transitive dependencies.
            format_schema(path, check)?;
        }

        // [tag:shell_completion_subcommand]
        Some(subcommand) if subcommand == SHELL_COMPLETION_SUBCOMMAND => {
            shell_completion(
                matches
                    .subcommand_matches(SHELL_COMPLETION_SUBCOMMAND)
                    .unwrap() // [ref:shell_completion_subcommand]
                    .value_of(SHELL_COMPLETION_SUBCOMMAND_SHELL_OPTION)
                    // [ref:shell_completion_subcommand_shell_required]
                    .unwrap(),
            )?;
        }

        // We should never end up in this branch, provided we handled all the subcommands
        // above.
        Some(_) => panic!("Subcommand not implemented."),

        // If no subcommand was provided, the help message should have been printed
        // [ref:subcommand_required_else_help].
        None => panic!("The help message should have been printed."),
    }

    // If we made it this far, nothing went wrong.
    Ok(())
}

// Let the fun begin!
fn main() {
    // Jump to the entrypoint and report any resulting errors.
    if let Err(e) = entry() {
        eprintln!("{}", e);
        exit(1);
    }
}
