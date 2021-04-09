use crate::format::CodeStr;
use colored::{control::SHOULD_COLORIZE, Colorize};
use pad::{Alignment, PadStr};
use std::{
    cmp::{max, min},
    error, fmt,
    path::Path,
    rc::Rc,
};

// This is the primary error type we'll be using everywhere.
#[derive(Clone, Debug)]
pub struct Error {
    pub message: String,
    pub reason: Option<Rc<dyn error::Error>>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(reason) = &self.reason {
            write!(
                f,
                "{}\n\n{} {}",
                self.message,
                "Reason:".blue().bold(),
                reason,
            )
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl error::Error for Error {
    fn source<'a>(&'a self) -> Option<&(dyn error::Error + 'static)> {
        self.reason.as_deref()
    }
}

// This function constructs an `Error` from a message.
pub fn from_message<T: error::Error + 'static>(
    message: &str,
    source_path: Option<&Path>,
    reason: Option<T>,
) -> Error {
    with_listing(message, source_path, "", reason)
}

// This function constructs an `Error` that may occur at a specific location in a source file.
pub fn with_context<T: error::Error + 'static>(
    message: &str,
    source_path: Option<&Path>,
    source_contents: &str,
    source_range: (usize, usize), // Inclusive on the left and exclusive on the right
    reason: Option<T>,
) -> Error {
    with_listing(
        message,
        source_path,
        &listing(source_contents, source_range.0, source_range.1),
        reason,
    )
}

// This function constructs an `Error` with a given source listing.
pub fn with_listing<T: error::Error + 'static>(
    message: &str,
    source_path: Option<&Path>,
    listing: &str,
    reason: Option<T>,
) -> Error {
    #[allow(clippy::option_map_or_none)]
    Error {
        message: if let Some(path) = source_path {
            if listing.is_empty() {
                format!(
                    "{} {} {}",
                    "[Error]".red().bold(),
                    format!("[{}]", path.to_string_lossy().code_str()).magenta(),
                    message,
                )
            } else {
                format!(
                    "{} {} {}\n\n{}",
                    "[Error]".red().bold(),
                    format!("[{}]", path.to_string_lossy().code_str()).magenta(),
                    message,
                    listing,
                )
            }
        } else if listing.is_empty() {
            format!("{} {}", "[Error]".red().bold(), message)
        } else {
            format!("{} {}\n\n{}", "[Error]".red().bold(), message, listing)
        },

        reason: reason.map_or(None, |reason| Some(Rc::new(reason))),
    }
}

// This function renders the relevant lines of a source file given the source file contents and a
// range. The range is inclusive on the left and exclusive on the right.
pub fn listing(source_contents: &str, range_start: usize, range_end: usize) -> String {
    // Remember the relevant lines and the position of the start of the next line.
    let mut lines = vec![];
    let mut pos = 0_usize;

    // Find the relevant lines.
    for (i, line) in source_contents.split('\n').enumerate() {
        // Record the start of the line before we advance the cursor.
        let line_start = pos;

        // Move the cursor to the start of the next line.
        pos += line.len() + 1;

        // If we're past the lines of interest, we're done.
        if line_start >= range_end {
            break;
        }

        // If we haven't reached the lines of interest yet, skip to the next line.
        if pos <= range_start {
            continue;
        }

        // We trim the end of the line to remove any carriage return (or any other whitespace) that
        // might have been present before the line feed.
        let trimmed_line = line.trim_end();

        // Highlight the relevant part of the line.
        let (section_start, section_end) = if range_start > line_start {
            (
                min(range_start - line_start, trimmed_line.len()),
                min(range_end - line_start, trimmed_line.len()),
            )
        } else {
            let end = min(range_end - line_start, trimmed_line.len());
            let start = trimmed_line
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(end);

            (start, end)
        };

        // Record the line number and the line contents.
        lines.push((
            (i + 1).to_string(),
            trimmed_line,
            section_start,
            section_end,
        ));
    }

    // Compute the width of the string representation of the hugest relevant line number.
    let gutter_width = lines.iter().fold(0_usize, |acc, (line_number, _, _, _)| {
        max(acc, line_number.len())
    });

    // Determine whether the output will be colorized.
    let colorized = SHOULD_COLORIZE.should_colorize();

    // Render the code listing with line numbers.
    lines
        .iter()
        .enumerate()
        .map(|(i, (line_number, line, section_start, section_end))| {
            format!(
                "{}{}{}",
                format!(
                    "{} \u{2502} ",
                    line_number.pad(gutter_width, ' ', Alignment::Right, false),
                )
                .blue()
                .bold(),
                format!(
                    "{}{}{}",
                    &line[..*section_start],
                    &line[*section_start..*section_end].red(),
                    &line[*section_end..],
                ),
                if colorized {
                    "".to_owned()
                } else if section_start == section_end {
                    format!(
                        "\n{} {}",
                        " ".repeat(gutter_width),
                        if i == lines.len() - 1 {
                            " "
                        } else {
                            "\u{250a}"
                        },
                    )
                } else {
                    format!(
                        "\n{} {} {}{}",
                        " ".repeat(gutter_width),
                        if i == lines.len() - 1 {
                            " "
                        } else {
                            "\u{250a}"
                        },
                        " ".repeat(*section_start),
                        // [tag:overline_u203e]
                        "\u{203e}".repeat(section_end - section_start),
                    )
                },
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::error::{from_message, with_context, Error};
    use std::path::Path;

    #[test]
    fn from_message_no_path() {
        let error = from_message::<Error>("An error occurred.", None, None);

        assert_eq!(error.message, "[Error] An error occurred.");
    }

    #[test]
    fn from_message_with_path() {
        let error = from_message::<Error>("An error occurred.", Some(Path::new("foo.g")), None);

        assert_eq!(error.message, "[Error] [`foo.g`] An error occurred.");
    }

    #[test]
    fn with_context_no_path_empty_range() {
        let error = with_context::<Error>("An error occurred.", None, "", (0, 0), None);

        assert_eq!(error.message, "[Error] An error occurred.");
    }

    #[test]
    fn with_context_with_path_empty_range() {
        let error = with_context::<Error>(
            "An error occurred.",
            Some(Path::new("foo.g")),
            "",
            (0, 0),
            None,
        );

        assert_eq!(error.message, "[Error] [`foo.g`] An error occurred.");
    }

    #[test]
    fn with_context_no_path_single_line_full_range() {
        let error = with_context::<Error>("An error occurred.", None, "foo", (0, 3), None);

        assert_eq!(
            error.message,
            "[Error] An error occurred.\n\n1 \u{2502} foo\n    \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn with_context_with_path_single_line_full_range() {
        let error = with_context::<Error>(
            "An error occurred.",
            Some(Path::new("foo.g")),
            "foo",
            (0, 3),
            None,
        );

        assert_eq!(
            error.message,
            "[Error] [`foo.g`] An error occurred.\n\n1 \u{2502} foo\n    \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn with_context_no_path_multiple_lines_full_range() {
        let error =
            with_context::<Error>("An error occurred.", None, "foo\nbar\nbaz", (0, 11), None);

        assert_eq!(
            error.message,
            "[Error] An error occurred.\n\n1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \
                \u{2502} bar\n  \u{250a} \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n    \u{203e}\
                \u{203e}\u{203e}",
        );
    }

    #[test]
    fn with_context_no_path_multiple_lines_partial_range() {
        let error = with_context::<Error>(
            "An error occurred.",
            None,
            "foo\nbar\nbaz\nqux",
            (5, 11),
            None,
        );

        assert_eq!(
            error.message,
            "[Error] An error occurred.\n\n2 \u{2502} bar\n  \u{250a}  \u{203e}\u{203e}\n3 \
                \u{2502} baz\n    \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn with_context_no_path_many_lines_partial_range() {
        let error = with_context::<Error>(
            "An error occurred.",
            None,
            "foo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux",
            (33, 42),
            None,
        );

        assert_eq!(
            error.message,
            "[Error] An error occurred.\n\n 9 \u{2502} foo\n   \u{250a}  \u{203e}\u{203e}\n10 \
                \u{2502} bar\n   \u{250a} \u{203e}\u{203e}\u{203e}\n11 \u{2502} baz\n     \u{203e}\
                \u{203e}",
        );
    }
}
