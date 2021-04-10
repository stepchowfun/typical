#![deny(warnings)]

mod assertions;
mod count;
mod error;
mod format;
mod generate_rust;
mod parser;
mod schema;
mod token;
mod tokenizer;
mod validator;

use crate::{
    count::count,
    error::{listing, throw, Error},
    format::CodeStr,
    parser::parse,
    tokenizer::tokenize,
    validator::validate,
};
use clap::{App, AppSettings, Arg, Shell, SubCommand};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io::stdout,
    path::{Path, PathBuf},
    process::exit,
};

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// The name of the program binary
const BIN_NAME: &str = "typical";

// Command-line option and subcommand names
const GENERATE_SUBCOMMAND: &str = "generate";
const GENERATE_SUBCOMMAND_PATH_OPTION: &str = "generate-path";
const GENERATE_SUBCOMMAND_RUST_OUT_DIR_OPTION: &str = "rust-out-dir";
const SHELL_COMPLETION_SUBCOMMAND: &str = "shell-completion";
const SHELL_COMPLETION_SUBCOMMAND_SHELL_OPTION: &str = "shell-completion-shell";

// Set up the command-line interface.
fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Typical")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about("Typical is an interface definition language.")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::NextLineHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp) // [tag:subcommand_required_else_help],
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name(GENERATE_SUBCOMMAND)
                .about("Generate code for a schema and its transitive dependencies")
                .arg(
                    Arg::with_name(GENERATE_SUBCOMMAND_PATH_OPTION)
                        .value_name("SCHEMA_PATH")
                        .help("Sets the path of the schema")
                        .required(true), // [tag:generate_subcommand_path_required],
                )
                .arg(
                    Arg::with_name(GENERATE_SUBCOMMAND_RUST_OUT_DIR_OPTION)
                        .value_name("PATH")
                        .long(GENERATE_SUBCOMMAND_RUST_OUT_DIR_OPTION)
                        .help("Sets the path of the directory to emit Rust code"),
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
                        .required(true), // [tag:shell_completion_subcommand_shell_required],
                ),
        )
}

// Load a schema and its transitive dependencies. If this function succeeds, the imports in the
// returned schemas are guaranteed to have valid paths which are relative to and contained within
// the directory containing `schema_path` [tag:valid_based_paths]. No other validation is performed
// by this function.
#[allow(clippy::too_many_lines)]
fn load_schemas(
    schema_path: &Path,
) -> Result<HashMap<PathBuf, (schema::Schema, String)>, Vec<Error>> {
    // The schema and all its transitive dependencies will end up here.
    let mut schemas = HashMap::new();

    // Any errors will end up here.
    let mut errors = vec![];

    // Canonicalize the path.
    let canonical_schema_path = match schema_path.canonicalize() {
        Ok(canonical_schema_path) => canonical_schema_path,
        Err(error) => {
            errors.push(throw(
                &format!(
                    "Unable to load {}.",
                    schema_path.to_string_lossy().code_str(),
                ),
                None,
                None,
                Some(error),
            ));

            return Err(errors);
        }
    };

    // Compute the base directory for this schema's dependencies.
    let base_path = if let Some(base_path) = canonical_schema_path.parent() {
        base_path
    } else {
        errors.push(throw::<Error>(
            &format!(
                "{} is not a file.",
                schema_path.to_string_lossy().code_str(),
            ),
            None,
            None,
            None,
        ));

        return Err(errors);
    };

    // Strip the schema parent path from the schema path, i.e., compute the schema file name. The
    // `unwrap` is safe because we know `base_path` is the parent of `canonical_schema_path`.
    let based_schema_path = canonical_schema_path.strip_prefix(base_path).unwrap();

    // Initialize the "frontier" with the given path. Paths in the frontier are relative to
    // `base_path` [tag:frontier_paths_based].
    let mut paths_to_load: Vec<(PathBuf, Option<(PathBuf, String)>)> =
        vec![(based_schema_path.to_owned(), None)];
    let mut visited_paths = HashSet::new();
    visited_paths.insert(based_schema_path.to_owned());

    // Perform a depth-first traversal of the transitive dependencies.
    while let Some((path, origin)) = paths_to_load.pop() {
        // Read the file.
        let contents = match read_to_string(&base_path.join(&path)) {
            Ok(contents) => contents,
            Err(error) => {
                let message = format!("Unable to load {}.", path.to_string_lossy().code_str());

                if let Some((origin_path, origin_listing)) = origin {
                    errors.push(throw(
                        &message,
                        Some(&origin_path),
                        Some(&origin_listing),
                        Some(error),
                    ));
                } else {
                    errors.push(throw(&message, None, None, Some(error)));
                }

                continue;
            }
        };

        // Tokenize the contents.
        let tokens = match tokenize(&path, &contents) {
            Ok(tokens) => tokens,
            Err(error) => {
                errors.extend_from_slice(&error);

                continue;
            }
        };

        // Parse the tokens.
        let mut schema = match parse(&path, &contents, &tokens) {
            Ok(schema) => schema,
            Err(error) => {
                errors.extend_from_slice(&error);

                continue;
            }
        };

        // Compute the base directory for this schema's dependencies. The `unwrap` is safe due to
        // [ref:frontier_paths_based].
        let parent_path = path.parent().unwrap();

        // Add the dependencies to the frontier.
        for import in &mut schema.imports {
            // Compute the source listing for this import for error reporting.
            let origin_listing = listing(&contents, import.source_range);

            // Compute the import path.
            let non_canonical_import_path = base_path.join(parent_path.join(&import.original_path));

            // Canonicalize the path.
            let canonical_import_path = match non_canonical_import_path.canonicalize() {
                Ok(canonical_import_path) => canonical_import_path,
                Err(error) => {
                    errors.push(throw(
                        &format!(
                            "Unable to load {}.",
                            import.original_path.to_string_lossy().code_str(),
                        ),
                        Some(&path),
                        Some(&origin_listing),
                        Some(error),
                    ));

                    continue;
                }
            };

            // Populate the based path in the schema by stripping the base path from the canonical
            // import path[tag:populate_based_path].
            import.based_path =
                if let Ok(based_import_path) = canonical_import_path.strip_prefix(base_path) {
                    based_import_path.to_owned()
                } else {
                    errors.push(throw::<Error>(
                        &format!(
                            "{} is not a descendant of {}, which is the base directory for this \
                                run.",
                            canonical_import_path.to_string_lossy().code_str(),
                            base_path.to_string_lossy().code_str(),
                        ),
                        Some(&path),
                        Some(&origin_listing),
                        None,
                    ));

                    continue;
                };

            // Visit this import if it hasn't been visited already.
            if !visited_paths.contains(&import.based_path) {
                visited_paths.insert(import.based_path.clone());
                paths_to_load.push((
                    import.based_path.clone(),
                    Some((path.clone(), origin_listing)),
                ));
            }
        }

        // Store the schema.
        schemas.insert(path.clone(), (schema, contents));
    }

    // Return a success or report any errors.
    if errors.is_empty() {
        Ok(schemas)
    } else {
        Err(errors)
    }
}

// Merge a list of errors into a single one.
fn merge_errors(errors: &[Error]) -> Error {
    Error {
        message: errors
            .iter()
            .fold(String::new(), |acc, error| {
                format!(
                    "{}\n{}{}",
                    acc,
                    // Only render an empty line between errors here if the previous line
                    // doesn't already visually look like an empty line. See
                    // [ref:overline_u203e].
                    if acc
                        .split('\n')
                        .last()
                        .unwrap()
                        .chars()
                        .all(|c| c == ' ' || c == '\u{203e}')
                    {
                        ""
                    } else {
                        "\n"
                    },
                    error,
                )
            })
            .trim()
            .to_owned(),
        reason: None,
    }
}

// Generate code for a schema and its transitive dependencies
fn generate_code(schema_path: &Path, rust_out_dir: Option<&Path>) -> Result<(), Error> {
    // Load the schema and its transitive dependencies.
    eprintln!("Loading schemas\u{2026}");
    let schemas = load_schemas(schema_path).map_err(|errors| merge_errors(&errors))?;
    eprintln!("{} loaded.", count(schemas.len(), "schema"));

    // Validate the schemas.
    eprintln!("Validating schemas\u{2026}");
    validate(&schemas).map_err(|errors| merge_errors(&errors))?;

    // Generate Rust code, if applicable.
    if let Some(rust_out_dir) = rust_out_dir {
        eprintln!("Generating Rust\u{2026}");
        generate_rust::generate(&schemas, rust_out_dir)?;
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
            // Determine the path to the schema file.
            let schema_path = Path::new(
                matches
                    .subcommand_matches(GENERATE_SUBCOMMAND)
                    .unwrap() // [ref:generate_subcommand]
                    .value_of(GENERATE_SUBCOMMAND_PATH_OPTION)
                    // [ref:generate_subcommand_path_required]
                    .unwrap(),
            );

            // Determine the path to the Rust output directory, if provided.
            let rust_out_dir = matches
                .subcommand_matches(GENERATE_SUBCOMMAND)
                .unwrap() // [ref:generate_subcommand]
                .value_of(GENERATE_SUBCOMMAND_RUST_OUT_DIR_OPTION)
                .map(Path::new);

            // Generate code for the schema.
            generate_code(schema_path, rust_out_dir)?;
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
