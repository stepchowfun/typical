mod assertions;
mod error;
mod format;
mod token;
mod tokenizer;

use crate::{
    error::{lift, Error},
    format::CodeStr,
    tokenizer::tokenize,
};
use atty::Stream;
use clap::{
    App,
    AppSettings::{
        ColoredHelp, SubcommandRequiredElseHelp, UnifiedHelpMessage, VersionlessSubcommands,
    },
    Arg, Shell, SubCommand,
};
use std::{fs::read_to_string, io::stdout, path::Path, process::exit, thread};

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// The name of the program binary
const BIN_NAME: &str = "typical";

// Command-line option and subcommand names
const CHECK_SUBCOMMAND: &str = "check";
const CHECK_SUBCOMMAND_PATH_OPTION: &str = "check-path";
const SHELL_COMPLETION_SUBCOMMAND: &str = "shell-completion";
const SHELL_COMPLETION_SUBCOMMAND_SHELL_OPTION: &str = "shell-completion-shell";

// The stack size in bytes.
const STACK_SIZE: usize = 16 * 1024 * 1024; // 16 mebibytes (MiB)

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
                .about("Checks a program")
                .arg(
                    Arg::with_name(CHECK_SUBCOMMAND_PATH_OPTION)
                        .value_name("PATH")
                        .help("Sets the path of the program entrypoint")
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

// Process a schema.
fn process_schema(schema_path: &Path) -> Result<(), Error> {
    // Read the schema file.
    let schema_contents = read_to_string(schema_path).map_err(lift(format!(
        "Error when reading file {}.",
        schema_path.to_string_lossy().code_str(),
    )))?;

    // Here is a helper function for mapping a `Vec<Error>` to a single `Error`.
    let collect_errors = |errors: Vec<Error>| Error {
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
    };

    // Tokenize the schema file.
    let tokens = tokenize(Some(schema_path), &schema_contents).map_err(collect_errors)?;

    // Print the tokens.
    for token in tokens {
        println!("{}", token.to_string().code_str());
    }

    // If we made it this far, nothing went wrong.
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
    // Determine whether to print colored output based on whether STDOUT is connected to a
    // terminal.
    colored::control::set_override(atty::is(Stream::Stdout));

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

            // Check the program.
            process_schema(schema_path)?;
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
    // Run everything in a new thread with a big stack.
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            // Jump to the entrypoint and report any resulting errors.
            if let Err(e) = entry() {
                eprintln!("{}", e);
                exit(1);
            }
        })
        .unwrap_or_else(|e| {
            eprintln!("Error spawning thread: {:?}", e);
            exit(1);
        })
        .join()
        .unwrap_or_else(|e| {
            eprintln!("Error joining thread: {:?}", e);
            exit(1);
        });
}
