use crate::{format::CodeStr, line_index::LineIndex};
use colored::{Colorize, control::SHOULD_COLORIZE};
use std::{
    cmp::{max, min},
    error, fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

// For extra type safety, we introduce a dedicated type for source ranges. Tokens and syntax trees
// can use this type instead of tuples.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceRange {
    pub start: usize, // Inclusive
    pub end: usize,   // Exclusive
}

// This is the primary error type we'll be using everywhere. Its parts are exposed through accessors
// rather than public fields so they can't become inconsistent.
#[derive(Clone, Debug)]
pub struct Error {
    message: String,
    source_range: Option<SourceRange>,
    source_path: Option<PathBuf>,
    listing: Option<String>,
    reason: Option<Arc<dyn error::Error + Send + Sync>>,
}

impl Error {
    // Construct an error and render its source context for terminal output when available.
    pub fn new(
        message: &str,
        source_path: Option<&Path>,
        source_context: Option<(&str, &LineIndex, SourceRange)>,
        reason: Option<Arc<dyn error::Error + Send + Sync>>,
    ) -> Self {
        let (source_range, source_listing) = source_context.map_or(
            (None, None),
            |(source_contents, line_index, source_range)| {
                (
                    Some(source_range),
                    Some(listing(source_contents, line_index, source_range)),
                )
            },
        );

        Self {
            message: message.to_owned(),
            source_range,
            source_path: source_path.map(Path::to_owned),
            listing: source_listing,
            reason,
        }
    }

    // Describe what went wrong.
    pub fn message(&self) -> &str {
        &self.message
    }

    // Locate the error in its source, if it has a location.
    pub fn source_range(&self) -> Option<SourceRange> {
        self.source_range
    }

    // Identify the file the error is about, if any.
    pub fn source_path(&self) -> Option<&Path> {
        self.source_path.as_deref()
    }

    // Show the source lines the error refers to, as rendered for the terminal, if it has a
    // location.
    pub fn listing(&self) -> Option<&str> {
        self.listing.as_deref()
    }

    // Expose the underlying cause of the error, if any, without the thread-safety bounds that only
    // matter for sharing it.
    pub fn reason(&self) -> Option<&(dyn error::Error + 'static)> {
        self.reason
            .as_deref()
            .map(|reason| reason as &(dyn error::Error + 'static))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Render the error header from its structured fields.
        write!(f, "{}", "[Error]".red().bold())?;
        if let Some(path) = self.source_path() {
            write!(f, " {}", format!("[{}]", path.code_str()).magenta())?;
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
    // Append each error to the output, examining only the output's last line so the cost doesn't
    // grow with the output that precedes it.
    let mut formatted = String::new();
    for error in errors {
        // Only render an empty line between errors here if the previous line doesn't already
        // visually look like an empty line. See [ref:overline_u203e].
        let last_line = &formatted[formatted.rfind('\n').map_or(0, |index| index + 1)..];
        formatted.push_str(if last_line.chars().all(|c| c == ' ' || c == '\u{203e}') {
            "\n"
        } else {
            "\n\n"
        });
        formatted.push_str(&error.to_string());
    }
    formatted.trim().to_owned()
}

// This function renders the relevant lines of a source file given the source file contents, an
// index of its lines, and a range. The range is inclusive on the left and exclusive on the right.
fn listing(source_contents: &str, line_index: &LineIndex, source_range: SourceRange) -> String {
    // Remember the relevant lines.
    let mut lines = vec![];

    // Visit the lines which start before the end of the range, beginning with the one containing
    // the start of the range, so the lines before it are never examined. A range starting beyond
    // the source has no lines to show.
    let Some(mut i) = line_index.line(source_range.start) else {
        return String::new();
    };
    while let Some(line_start) = line_index.line_start(i)
        && line_start < source_range.end
    {
        // Extract the line without its line feed.
        let line_end = line_index
            .line_end(i)
            .expect("A line that starts should also end.");
        let line = &source_contents[line_start..line_end];

        // We trim the end of the line to remove any carriage return (or any other whitespace) that
        // might have been present before the line feed.
        let trimmed_line = line.trim_end();

        // Highlight the part of the range within the line's content, excluding its indentation
        // as well as its trailing whitespace, and highlight nothing if the range covers only
        // whitespace on this line.
        let content_start = trimmed_line
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(trimmed_line.len());
        let section_end = min(source_range.end - line_start, trimmed_line.len());
        let section_start = min(
            max(source_range.start.saturating_sub(line_start), content_start),
            section_end,
        );

        // Record the line number and the line contents.
        lines.push((
            (i + 1).to_string(),
            trimmed_line,
            section_start,
            section_end,
        ));
        i += 1;
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
                } else {
                    // Continue the gutter below the line, except below the last one.
                    let gutter = format!(
                        "{} {}",
                        " ".repeat(gutter_width),
                        if i == lines.len() - 1 {
                            " "
                        } else {
                            "\u{250a}"
                        },
                    );
                    if section_start != section_end {
                        format!(
                            "\n{gutter} {}{}",
                            " ".repeat(*section_start),
                            // [tag:overline_u203e]
                            "\u{203e}".repeat(section_end - section_start),
                        )
                    } else if i < lines.len() - 1 {
                        format!("\n{gutter}")
                    } else {
                        String::new()
                    }
                },
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::{
        error::{Error, SourceRange, format_errors, listing},
        line_index::LineIndex,
    };
    use std::{path::Path, sync::Arc};

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
            Some((
                SOURCE_CONTENTS,
                &LineIndex::new(SOURCE_CONTENTS),
                SOURCE_RANGE,
            )),
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
        let error = Error::new("An error occurred.", None, None, Some(Arc::new(reason)));

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
            Some((
                SOURCE_CONTENTS,
                &LineIndex::new(SOURCE_CONTENTS),
                SOURCE_RANGE,
            )),
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
            Some((
                SOURCE_CONTENTS,
                &LineIndex::new(SOURCE_CONTENTS),
                SOURCE_RANGE,
            )),
            Some(Arc::new(reason)),
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
            Some(Arc::new(reason)),
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
            Some((
                SOURCE_CONTENTS,
                &LineIndex::new(SOURCE_CONTENTS),
                SOURCE_RANGE,
            )),
            Some(Arc::new(reason)),
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
        let source = "";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 0, end: 0 },
            ),
            "",
        );
    }

    #[test]
    fn listing_single_line_full_range() {
        let source = "foo bar";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 0, end: 7 },
            ),
            "1 \u{2502} foo bar\n    \u{203e}\u{203e}\u{203e}\u{203e}\u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_single_line_partial_range() {
        let source = "foo bar";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 1, end: 6 },
            ),
            "1 \u{2502} foo bar\n     \u{203e}\u{203e}\u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_multiple_lines_full_range() {
        let source = "foo\nbar\nbaz\nqux";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 0, end: 15 },
            ),
            "1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} bar\n  \u{250a} \
                \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \u{250a} \u{203e}\u{203e}\u{203e}\n4 \
                \u{2502} qux\n    \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_multiple_lines_partial_range() {
        let source = "foo\nbar\nbaz\nqux";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 5, end: 9 },
            ),
            "2 \u{2502} bar\n  \u{250a}  \u{203e}\u{203e}\n3 \u{2502} baz\n    \u{203e}",
        );
    }

    #[test]
    fn listing_many_lines_partial_range() {
        let source = "foo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux\nfoo\nbar\nbaz\nqux";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 33, end: 42 },
            ),
            " 9 \u{2502} foo\n   \u{250a}  \u{203e}\u{203e}\n10 \u{2502} bar\n   \u{250a} \
                \u{203e}\u{203e}\u{203e}\n11 \u{2502} baz\n     \u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_range_starting_in_indentation() {
        let source = "    foo\nbar";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 2, end: 11 },
            ),
            "1 \u{2502}     foo\n  \u{250a}     \u{203e}\u{203e}\u{203e}\n2 \u{2502} bar\n    \
                \u{203e}\u{203e}\u{203e}",
        );
    }

    #[test]
    fn listing_range_ending_in_indentation() {
        let source = "foo\n    bar";

        assert_eq!(
            listing(
                source,
                &LineIndex::new(source),
                SourceRange { start: 0, end: 6 },
            ),
            "1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502}     bar",
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
