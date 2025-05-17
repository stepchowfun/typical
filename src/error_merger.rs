use crate::error::Error;

// Merge a list of errors into a single one.
pub fn merge_errors(errors: &[Error]) -> Error {
    Error {
        message: errors
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
            .to_owned(),
        reason: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::Error, error_merger::merge_errors};

    #[test]
    fn merge_errors_empty() {
        assert_eq!(format!("{}", merge_errors(&[])), "");
    }

    #[test]
    fn merge_errors_single() {
        assert_eq!(
            format!(
                "{}",
                merge_errors(&[Error {
                    message: "Something went wrong.".to_owned(),
                    reason: None,
                }]),
            ),
            "Something went wrong.",
        );
    }

    #[test]
    fn merge_errors_double() {
        assert_eq!(
            format!(
                "{}",
                merge_errors(&[
                    Error {
                        message: "Something went kinda wrong.".to_owned(),
                        reason: None,
                    },
                    Error {
                        message: "Something went sorta wrong.".to_owned(),
                        reason: None,
                    },
                    Error {
                        message: "Something went very wrong.".to_owned(),
                        reason: None,
                    },
                ]),
            ),
            "\
Something went kinda wrong.

Something went sorta wrong.

Something went very wrong.\
",
        );
    }

    #[test]
    fn merge_errors_visually_empty_line() {
        assert_eq!(
            format!(
                "{}",
                merge_errors(&[
                    Error {
                        message: "1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} \
                                bar\n  \u{250a} \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \
                                \u{250a} \u{203e}\u{203e}\u{203e}\n4 \u{2502} qux\n    \u{203e}\
                                \u{203e}\u{203e}"
                            .to_owned(),
                        reason: None,
                    },
                    Error {
                        message: "Something went sorta wrong.".to_owned(),
                        reason: None,
                    },
                ]),
            ),
            "\
1 \u{2502} foo\n  \u{250a} \u{203e}\u{203e}\u{203e}\n2 \u{2502} \
bar\n  \u{250a} \u{203e}\u{203e}\u{203e}\n3 \u{2502} baz\n  \
\u{250a} \u{203e}\u{203e}\u{203e}\n4 \u{2502} qux\n    \u{203e}\
\u{203e}\u{203e}
Something went sorta wrong.\
",
        );
    }
}
