use crate::format::CodePath;
use colored::{Colorize, control::SHOULD_COLORIZE};
use std::{
    cmp::{max, min},
    error, fmt,
    path::{Path, PathBuf},
    rc::Rc,
};

// For extra type safety, we introduce a dedicated type for source ranges. Tokens and syntax trees
// can use this type instead of tuples.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceRange {
    pub start: usize, // Inclusive
    pub end: usize,   // Exclusive
}

// This is the primary error type we'll be using everywhere.
#[derive(Clone, Debug)]
pub struct Error {
    message: String,
    source_range: Option<SourceRange>,
    source_path: Option<PathBuf>,
    listing: Option<String>,
    reason: Option<Rc<dyn error::Error>>,
}

impl Error {
    // Construct an error and render its source context for terminal output when available.
    pub fn new(
        message: &str,
        source_path: Option<&Path>,
        source_context: Option<(&str, SourceRange)>,
        reason: Option<Rc<dyn error::Error>>,
    ) -> Self {
        let (source_range, source_listing) =
            source_context.map_or((None, None), |(source_contents, source_range)| {
                (
                    Some(source_range),
                    Some(listing(source_contents, source_range)),
                )
            });

        Self {
            message: message.to_owned(),
            source_range,
            source_path: source_path.map(Path::to_owned),
            listing: source_listing,
            reason,
        }
    }

    // Expose the structured parts of the error without allowing them to become inconsistent.
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn source_range(&self) -> Option<SourceRange> {
        self.source_range
    }
    pub fn source_path(&self) -> Option<&Path> {
        self.source_path.as_deref()
    }
    pub fn listing(&self) -> Option<&str> {
        self.listing.as_deref()
    }
    pub fn reason(&self) -> Option<&(dyn error::Error + 'static)> {
        self.reason.as_deref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Render the error header from its structured fields.
        write!(f, "{}", "[Error]".red().bold())?;
        if let Some(path) = self.source_path() {
            write!(f, " {}", format!("[{}]", path.code_path()).magenta())?;
        }
        write!(f, " {}", self.message())?;

        // Include the source listing when one is available and nonempty.
        if let Some(listing) = self.listing()
            && !listing.is_empty()
        {
            write!(f, "\n\n{listing}")?;
        }

        // Include the underlying failure when one is available.
        if let Some(reason) = self.reason() {
            write!(f, "\n\n{} {}", "Reason:".blue().bold(), reason)?;
        }

        Ok(())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.reason()
    }
}

// Format a list while avoiding an extra visual gap after a source-range underline.
pub fn format_errors(errors: &[Error]) -> String {
    errors
        .iter()
        .fold(String::new(), |acc, error| {
            format!(
                "{}\n{}{}",
                acc,
                // Only render an empty line between errors here if the previous line doesn't
                // already visually look like an empty line. See [ref:overline_u203e].
                if acc
                    .split('\n')
                    .next_back()
                    .unwrap() // Safe since `split` always results in at least one item
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
        .to_owned()
}

// This function renders the relevant lines of a source file given the source file contents and a
// range. The range is inclusive on the left and exclusive on the right.
fn listing(source_contents: &str, source_range: SourceRange) -> String {
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
        if line_start >= source_range.end {
            break;
        }

        // If we haven't reached the lines of interest yet, skip to the next line.
        if pos <= source_range.start {
            continue;
        }

        // We trim the end of the line to remove any carriage return (or any other whitespace) that
        // might have been present before the line feed.
        let trimmed_line = line.trim_end();

        // Highlight the relevant part of the line.
        let (section_start, section_end) = if source_range.start > line_start {
            (
                min(source_range.start - line_start, trimmed_line.len()),
                min(source_range.end - line_start, trimmed_line.len()),
            )
        } else {
            let end = min(source_range.end - line_start, trimmed_line.len());
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
                "{}{}{}{}{}",
                format!("{line_number:>gutter_width$} \u{2502} ")
                    .blue()
                    .bold(),
                &line[..*section_start],
                line[*section_start..*section_end].red(),
                &line[*section_end..],
                if colorized {
                    String::new()
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
    use crate::error::{Error, SourceRange, format_errors, listing};
    use std::{path::Path, rc::Rc};

    // Reuse one source context across the constructor tests.
    const SOURCE_CONTENTS: &str = "abcd";
    const SOURCE_RANGE: SourceRange = SourceRange { start: 1, end: 3 };
    const SOURCE_LISTING: &str = "1 \u{2502} abcd\n     \u{203e}\u{203e}";

    #[test]
    fn new_no_source_path_context_reason() {
        let error = Error::new("An error occurred.", None, None, None);

        assert_eq!(error.message(), "An error occurred.");
        assert!(error.source_path().is_none());
        assert!(error.source_range().is_none());
        assert!(error.listing().is_none());
        assert!(error.reason().is_none());
        assert_eq!(error.to_string(), "[Error] An error occurred.");
    }

    #[test]
    fn new_with_source_path_no_context_reason() {
        let error = Error::new("An error occurred.", Some(Path::new("foo")), None, None);

        assert_eq!(error.message(), "An error occurred.");
        assert_eq!(error.source_path(), Some(Path::new("foo")));
        assert!(error.source_range().is_none());
        assert!(error.listing().is_none());
        assert!(error.reason().is_none());
        assert_eq!(error.to_string(), "[Error] [`foo`] An error occurred.");
    }

    #[test]
    fn new_with_source_context_no_source_path_reason() {
        let error = Error::new(
            "An error occurred.",
            None,
            Some((SOURCE_CONTENTS, SOURCE_RANGE)),
            None,
        );

        assert_eq!(error.message(), "An error occurred.");
        assert!(error.source_path().is_none());
        assert_eq!(error.source_range(), Some(SOURCE_RANGE));
        assert_eq!(error.listing(), Some(SOURCE_LISTING));
        assert!(error.reason().is_none());
        assert_eq!(
            error.to_string(),
            format!("[Error] An error occurred.\n\n{SOURCE_LISTING}"),
        );
    }

    #[test]
    fn new_with_reason_no_source_path_context() {
        let reason = Error::new("A deeper error occurred.", None, None, None);
        let error = Error::new("An error occurred.", None, None, Some(Rc::new(reason)));

        assert_eq!(error.message(), "An error occurred.");
        assert!(error.source_path().is_none());
        assert!(error.source_range().is_none());
        assert!(error.listing().is_none());
        assert_eq!(
            error.reason().unwrap().to_string(),
            "[Error] A deeper error occurred.",
        );
        assert_eq!(
            error.to_string(),
            "[Error] An error occurred.\n\nReason: [Error] A deeper error occurred.",
        );
    }

    #[test]
    fn new_with_source_path_context_no_reason() {
        let error = Error::new(
            "An error occurred.",
            Some(Path::new("foo")),
            Some((SOURCE_CONTENTS, SOURCE_RANGE)),
            None,
        );

        assert_eq!(error.message(), "An error occurred.");
        assert_eq!(error.source_path(), Some(Path::new("foo")));
        assert_eq!(error.source_range(), Some(SOURCE_RANGE));
        assert_eq!(error.listing(), Some(SOURCE_LISTING));
        assert!(error.reason().is_none());
        assert_eq!(
            error.to_string(),
            format!("[Error] [`foo`] An error occurred.\n\n{SOURCE_LISTING}"),
        );
    }

    #[test]
    fn new_with_source_context_reason_no_source_path() {
        let reason = Error::new("A deeper error occurred.", None, None, None);
        let error = Error::new(
            "An error occurred.",
            None,
            Some((SOURCE_CONTENTS, SOURCE_RANGE)),
            Some(Rc::new(reason)),
        );

        assert_eq!(error.message(), "An error occurred.");
        assert!(error.source_path().is_none());
        assert_eq!(error.source_range(), Some(SOURCE_RANGE));
        assert_eq!(error.listing(), Some(SOURCE_LISTING));
        assert_eq!(
            error.reason().unwrap().to_string(),
            "[Error] A deeper error occurred.",
        );
        assert_eq!(
            error.to_string(),
            format!(
                "[Error] An error occurred.\n\n{SOURCE_LISTING}\n\nReason: [Error] A deeper \
                    error occurred.",
            ),
        );
    }

    #[test]
    fn new_with_source_path_reason_no_context() {
        let reason = Error::new("A deeper error occurred.", None, None, None);
        let error = Error::new(
            "An error occurred.",
            Some(Path::new("foo")),
            None,
            Some(Rc::new(reason)),
        );

        assert_eq!(error.message(), "An error occurred.");
        assert_eq!(error.source_path(), Some(Path::new("foo")));
        assert!(error.source_range().is_none());
        assert!(error.listing().is_none());
        assert_eq!(
            error.reason().unwrap().to_string(),
            "[Error] A deeper error occurred.",
        );
        assert_eq!(
            error.to_string(),
            "[Error] [`foo`] An error occurred.\n\nReason: [Error] A deeper error occurred.",
        );
    }

    #[test]
    fn new_with_source_path_context_reason() {
        let reason = Error::new("A deeper error occurred.", None, None, None);
        let error = Error::new(
            "An error occurred.",
            Some(Path::new("foo")),
            Some((SOURCE_CONTENTS, SOURCE_RANGE)),
            Some(Rc::new(reason)),
        );

        assert_eq!(error.message(), "An error occurred.");
        assert_eq!(error.source_path(), Some(Path::new("foo")));
        assert_eq!(error.source_range(), Some(SOURCE_RANGE));
        assert_eq!(error.listing(), Some(SOURCE_LISTING));
        assert_eq!(
            error.reason().unwrap().to_string(),
            "[Error] A deeper error occurred.",
        );
        assert_eq!(
            error.to_string(),
            format!(
                "[Error] [`foo`] An error occurred.\n\n{SOURCE_LISTING}\n\nReason: [Error] A \
                    deeper error occurred.",
            ),
        );
    }

    #[test]
    fn listing_empty() {
        assert_eq!(listing("", SourceRange { start: 0, end: 0 }), "");
    }

    #[test]
    fn listing_single_line_full_range() {
        assert_eq!(
            listing("foo bar", SourceRange { start: 0, end: 7 }),
            "1 \u{2502} foo bar\n    \u{203e}\u{203e}\u{203e}\u{203e}\u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_single_line_partial_range() {
        assert_eq!(
            listing("foo bar", SourceRange { start: 1, end: 6 }),
            "1 \u{2502} foo bar\n     \u{203e}\u{203e}\u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_multiple_lines_full_range() {
        assert_eq!(
            listing("foo\nbar\nbaz\nqux", SourceRange { start: 0, end: 15 }),
            "1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} bar\n  \u{250a} \
                \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \u{250a} \u{203e}\u{203e}\u{203e}\n4 \
                \u{2502} qux\n    \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_multiple_lines_partial_range() {
        assert_eq!(
            listing("foo\nbar\nbaz\nqux", SourceRange { start: 5, end: 9 }),
            "2 \u{2502} bar\n  \u{250a}  \u{203e}\u{203e}\n3 \u{2502} baz\n    \u{203e}",
        );
    }

    #[test]
    fn listing_many_lines_partial_range() {
        assert_eq!(
            listing(
                "foo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux",
                SourceRange { start: 33, end: 42 },
            ),
            " 9 \u{2502} foo\n   \u{250a}  \u{203e}\u{203e}\n10 \u{2502} bar\n   \u{250a} \
                \u{203e}\u{203e}\u{203e}\n11 \u{2502} baz\n     \u{203e}\u{203e}",
        );
    }

    #[test]
    fn format_errors_empty() {
        assert_eq!(format_errors(&[]), "");
    }

    #[test]
    fn format_errors_single() {
        assert_eq!(
            format_errors(&[Error::new("Something went wrong.", None, None, None)]),
            "[Error] Something went wrong.",
        );
    }

    #[test]
    fn format_errors_double() {
        assert_eq!(
            format_errors(&[
                Error::new("Something went kinda wrong.", None, None, None),
                Error::new("Something went sorta wrong.", None, None, None),
                Error::new("Something went very wrong.", None, None, None),
            ]),
            "\
[Error] Something went kinda wrong.

[Error] Something went sorta wrong.

[Error] Something went very wrong.\
",
        );
    }

    #[test]
    fn format_errors_visually_empty_line() {
        assert_eq!(
            format_errors(&[
                Error::new(
                    "1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} \
                                bar\n  \u{250a} \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \
                                \u{250a} \u{203e}\u{203e}\u{203e}\n4 \u{2502} qux\n    \u{203e}\
                                \u{203e}\u{203e}",
                    None,
                    None,
                    None,
                ),
                Error::new("Something went sorta wrong.", None, None, None),
            ]),
            "\
[Error] 1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} \
bar\n  \u{250a} \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \
\u{250a} \u{203e}\u{203e}\u{203e}\n4 \u{2502} qux\n    \u{203e}\
\u{203e}\u{203e}
[Error] Something went sorta wrong.\
",
        );
    }
}
