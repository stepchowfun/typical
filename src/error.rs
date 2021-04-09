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

// This function constructs a nicely formatted error.
pub fn throw<T: error::Error + 'static>(
    message: &str,
    source_path: Option<&Path>,
    listing: Option<&str>,
    reason: Option<T>,
) -> Error {
    #[allow(clippy::option_map_or_none)]
    Error {
        message: if let Some(path) = source_path {
            if let Some(listing) = listing {
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
            } else {
                format!(
                    "{} {} {}",
                    "[Error]".red().bold(),
                    format!("[{}]", path.to_string_lossy().code_str()).magenta(),
                    message,
                )
            }
        } else if let Some(listing) = listing {
            if listing.is_empty() {
                format!("{} {}", "[Error]".red().bold(), message)
            } else {
                format!("{} {}\n\n{}", "[Error]".red().bold(), message, listing)
            }
        } else {
            format!("{} {}", "[Error]".red().bold(), message)
        },

        reason: reason.map_or(None, |reason| Some(Rc::new(reason))),
    }
}

// This function renders the relevant lines of a source file given the source file contents and a
// range. The range is inclusive on the left and exclusive on the right.
pub fn listing(source_contents: &str, source_range: (usize, usize)) -> String {
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
        if line_start >= source_range.1 {
            break;
        }

        // If we haven't reached the lines of interest yet, skip to the next line.
        if pos <= source_range.0 {
            continue;
        }

        // We trim the end of the line to remove any carriage return (or any other whitespace) that
        // might have been present before the line feed.
        let trimmed_line = line.trim_end();

        // Highlight the relevant part of the line.
        let (section_start, section_end) = if source_range.0 > line_start {
            (
                min(source_range.0 - line_start, trimmed_line.len()),
                min(source_range.1 - line_start, trimmed_line.len()),
            )
        } else {
            let end = min(source_range.1 - line_start, trimmed_line.len());
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
    use crate::{
        assert_same,
        error::{listing, throw, Error},
    };
    use std::{path::Path, rc::Rc};

    #[test]
    fn error_no_reason_display() {
        assert_eq!(
            format!(
                "{}",
                Error {
                    message: "Something went wrong.".to_owned(),
                    reason: None,
                },
            ),
            "Something went wrong.",
        );
    }

    #[test]
    fn error_with_reason_display() {
        assert_eq!(
            format!(
                "{}",
                Error {
                    message: "Something went wrong.".to_owned(),
                    reason: Some(Rc::new(Error {
                        message: "Something deeper went wrong.".to_owned(),
                        reason: None,
                    })),
                },
            ),
            "\
            Something went wrong.\n\
            \n\
            Reason: Something deeper went wrong.\
            ",
        );
    }

    #[test]
    fn throw_no_source_path_listing_reason() {
        assert_same!(
            throw::<Error>("An error occurred.", None, None, None),
            Error {
                message: "[Error] An error occurred.".to_owned(),
                reason: None,
            },
        );
    }

    #[test]
    fn throw_with_source_path_no_listing_reason() {
        assert_same!(
            throw::<Error>("An error occurred.", Some(Path::new("foo")), None, None),
            Error {
                message: "[Error] [`foo`] An error occurred.".to_owned(),
                reason: None,
            },
        );
    }

    #[test]
    fn throw_with_listing_no_source_path_reason() {
        assert_same!(
            throw::<Error>("An error occurred.", None, Some("It happened here."), None),
            Error {
                message: "\
                    [Error] An error occurred.\n\
                    \n\
                    It happened here.\
                "
                .to_owned(),
                reason: None,
            },
        );
    }

    #[test]
    fn throw_with_reason_no_source_path_listing() {
        let reason = throw::<Error>("An deeper error occurred.", None, None, None);

        assert_same!(
            throw::<Error>("An error occurred.", None, None, Some(reason.clone())),
            Error {
                message: "[Error] An error occurred.".to_owned(),
                reason: Some(Rc::new(reason)),
            },
        );
    }

    #[test]
    fn throw_with_source_path_listing_no_reason() {
        assert_same!(
            throw::<Error>(
                "An error occurred.",
                Some(Path::new("foo")),
                Some("It happened here."),
                None,
            ),
            Error {
                message: "\
                    [Error] [`foo`] An error occurred.\n\
                    \n\
                    It happened here.\
                "
                .to_owned(),
                reason: None,
            },
        );
    }

    #[test]
    fn throw_with_listing_reason_no_source_path() {
        let reason = throw::<Error>("An deeper error occurred.", None, None, None);

        assert_same!(
            throw::<Error>(
                "An error occurred.",
                None,
                Some("It happened here."),
                Some(reason.clone()),
            ),
            Error {
                message: "\
                    [Error] An error occurred.\n\
                    \n\
                    It happened here.\
                "
                .to_owned(),
                reason: Some(Rc::new(reason)),
            },
        );
    }

    #[test]
    fn throw_with_source_path_reason_no_listing() {
        let reason = throw::<Error>("An deeper error occurred.", None, None, None);

        assert_same!(
            throw::<Error>(
                "An error occurred.",
                Some(Path::new("foo")),
                None,
                Some(reason.clone()),
            ),
            Error {
                message: "[Error] [`foo`] An error occurred.".to_owned(),
                reason: Some(Rc::new(reason)),
            },
        );
    }

    #[test]
    fn throw_with_source_path_listing_reason() {
        let reason = throw::<Error>("An deeper error occurred.", None, None, None);

        assert_same!(
            throw::<Error>(
                "An error occurred.",
                Some(Path::new("foo")),
                Some("It happened here."),
                Some(reason.clone()),
            ),
            Error {
                message: "\
                    [Error] [`foo`] An error occurred.\n\
                    \n\
                    It happened here.\
                "
                .to_owned(),
                reason: Some(Rc::new(reason)),
            },
        );
    }

    #[test]
    fn listing_empty() {
        assert_eq!(listing("", (0, 0)), "");
    }

    #[test]
    fn listing_single_line_full_range() {
        assert_eq!(
            listing("foo bar", (0, 7)),
            "1 \u{2502} foo bar\n    \u{203e}\u{203e}\u{203e}\u{203e}\u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_single_line_partial_range() {
        assert_eq!(
            listing("foo bar", (1, 6)),
            "1 \u{2502} foo bar\n     \u{203e}\u{203e}\u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_multiple_lines_full_range() {
        assert_eq!(
            listing("foo\nbar\nbaz\nqux", (0, 15)),
            "1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} bar\n  \u{250a} \
                \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \u{250a} \u{203e}\u{203e}\u{203e}\n4 \
                \u{2502} qux\n    \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_multiple_lines_partial_range() {
        assert_eq!(
            listing("foo\nbar\nbaz\nqux", (5, 9)),
            "2 \u{2502} bar\n  \u{250a}  \u{203e}\u{203e}\n3 \u{2502} baz\n    \u{203e}",
        );
    }

    #[test]
    fn listing_many_lines_partial_range() {
        assert_eq!(
            listing(
                "foo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux",
                (33, 42),
            ),
            " 9 \u{2502} foo\n   \u{250a}  \u{203e}\u{203e}\n10 \u{2502} bar\n   \u{250a} \
                \u{203e}\u{203e}\u{203e}\n11 \u{2502} baz\n     \u{203e}\u{203e}",
        );
    }
}
