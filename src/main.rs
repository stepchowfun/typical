#![deny(warnings)]

mod assertions;
mod error;
mod format;
mod parser;
mod schema;
mod token;
mod tokenizer;

use crate::{
    error::{from_message, listing, with_listing, Error},
    format::CodeStr,
    parser::parse,
    tokenizer::tokenize,
};
use clap::{
    App,
    AppSettings::{
        ColoredHelp, SubcommandRequiredElseHelp, UnifiedHelpMessage, VersionlessSubcommands,
    },
    Arg, Shell, SubCommand,
};
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
const CHECK_SUBCOMMAND: &str = "check";
const CHECK_SUBCOMMAND_PATH_OPTION: &str = "check-path";
const SHELL_COMPLETION_SUBCOMMAND: &str = "shell-completion";
const SHELL_COMPLETION_SUBCOMMAND_SHELL_OPTION: &str = "shell-completion-shell";

// Set up the command-line interface.
fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Typical")
        .version(VERSION)
        .version_short("v")
        .about("Typical is an interface definition language.")
        .setting(SubcommandRequiredElseHelp) // [tag:subcommand_required_else_help]
        .setting(ColoredHelp)
        .setting(UnifiedHelpMessage)
        .setting(VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name(CHECK_SUBCOMMAND)
                .about("Checks a schema")
                .arg(
                    Arg::with_name(CHECK_SUBCOMMAND_PATH_OPTION)
                        .value_name("PATH")
                        .help("Sets the path of the schema")
                        .required(true) // [tag:check_subcommand_shell_required]
                        .takes_value(true)
                        .number_of_values(1),
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
                        .required(true) // [tag:shell_completion_subcommand_shell_required]
                        .takes_value(true)
                        .number_of_values(1),
                ),
        )
}

// Check a schema.
#[allow(clippy::too_many_lines)]
fn check_schema(schema_path: &Path) -> Result<(), Error> {
    // The schema and all its transitive dependencies will end up here.
    let mut schemas = HashMap::new();

    // Canonicalize the path.
    let canonical_schema_path = match schema_path.canonicalize() {
        Ok(canonical_schema_path) => canonical_schema_path,
        Err(error) => {
            return Err(from_message(
                &format!(
                    "Unable to load {}.",
                    schema_path.to_string_lossy().code_str(),
                ),
                None,
                Some(error),
            ));
        }
    };

    // Compute the base directory for this schema's dependencies.
    let base_path = if let Some(base_path) = canonical_schema_path.parent() {
        base_path
    } else {
        return Err(from_message::<Error>(
            &format!(
                "{} is not a file.",
                schema_path.to_string_lossy().code_str(),
            ),
            None,
            None,
        ));
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
    let mut errors = vec![];
    while let Some((path, origin)) = paths_to_load.pop() {
        // Read the file.
        let contents = match read_to_string(&base_path.join(&path)) {
            Ok(contents) => contents,
            Err(error) => {
                let message = format!("Unable to load {}.", path.to_string_lossy().code_str());

                if let Some((origin_path, origin_listing)) = origin {
                    errors.push(with_listing(
                        &message,
                        Some(&origin_path),
                        &origin_listing,
                        Some(error),
                    ));
                } else {
                    errors.push(from_message(&message, None, Some(error)));
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
        let schema = match parse(&path, &contents, &tokens) {
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
        for import in &schema.imports {
            // Compute the source listing for this import for error reporting.
            let origin_listing = listing(&contents, import.source_range.0, import.source_range.1);

            // Compute the import path.
            let non_canonical_import_path = base_path.join(parent_path.join(&import.path));

            // Canonicalize the path.
            let canonical_import_path = match non_canonical_import_path.canonicalize() {
                Ok(canonical_import_path) => canonical_import_path,
                Err(error) => {
                    errors.push(with_listing(
                        &format!(
                            "Unable to load {}.",
                            import.path.to_string_lossy().code_str(),
                        ),
                        Some(&path),
                        &origin_listing,
                        Some(error),
                    ));

                    continue;
                }
            };

            // Strip the schema parent path from the schema path, i.e., compute the schema file
            // name.
            let based_import_path =
                if let Ok(based_import_path) = canonical_import_path.strip_prefix(base_path) {
                    based_import_path
                } else {
                    return Err(with_listing::<Error>(
                        &format!(
                            "{} is not a descendant of {}, which is the base directory for this \
                                run.",
                            canonical_import_path.to_string_lossy().code_str(),
                            base_path.to_string_lossy().code_str(),
                        ),
                        Some(&path),
                        &origin_listing,
                        None,
                    ));
                };

            // Visit this import if it hasn't been visited already.
            if !visited_paths.contains(based_import_path) {
                visited_paths.insert(based_import_path.to_owned());
                paths_to_load.push((
                    based_import_path.to_owned(),
                    Some((path.clone(), origin_listing)),
                ));
            }
        }

        // Store the schema.
        schemas.insert(path.clone(), schema);
    }

    // Print the schemas.
    let mut skip_blank_line = true;

    for (path, schema) in schemas {
        if skip_blank_line {
            skip_blank_line = false;
        } else {
            println!();
        }

        println!(
            "-- {}\n\n{}",
            path.to_string_lossy().code_str(),
            schema.to_string().code_str(),
        );
    }

    if !skip_blank_line && !errors.is_empty() {
        println!();
    }

    // Return a success or report any errors.
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error {
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
        })
    }
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
        // [tag:check_subcommand]
        Some(subcommand) if subcommand == CHECK_SUBCOMMAND => {
            // Determine the path to the schema file.
            let schema_path = Path::new(
                matches
                    .subcommand_matches(CHECK_SUBCOMMAND)
                    .unwrap() // [ref:check_subcommand]
                    .value_of(CHECK_SUBCOMMAND_PATH_OPTION)
                    // [ref:check_subcommand_shell_required]
                    .unwrap(),
            );

            // Check the schema.
            check_schema(schema_path)?;
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
