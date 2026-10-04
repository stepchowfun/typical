use crate::{
    error::{Error, SourceRange},
    format::CodeStr,
    line_index::LineIndex,
    token::{
        AS_KEYWORD, ASYMMETRIC_KEYWORD, BOOL_KEYWORD, BYTES_KEYWORD, CHOICE_KEYWORD,
        DELETED_KEYWORD, F64_KEYWORD, IMPORT_KEYWORD, OPTIONAL_KEYWORD, S64_KEYWORD,
        STRING_KEYWORD, STRUCT_KEYWORD, Token, U64_KEYWORD, UNIT_KEYWORD, Variant,
    },
};
use std::path::Path;
use unicode_segmentation::GraphemeCursor;

// An identifier can be prefixed with this character to avoid being parsed as a keyword.
const RAW_IDENTIFIER_SIGIL: char = '$';

// Tokenize the contents of a schema file.
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
pub fn tokenize(
    schema_path: &Path,
    schema_contents: &str,
    line_index: &LineIndex,
) -> Result<Vec<Token>, Vec<Error>> {
    // We'll be building up this vector of tokens.
    let mut tokens = vec![];

    // Construct a vector to hold any errors that might be detected below.
    let mut errors = vec![];

    // We want to iterate one code point at a time, but we also want the byte indices so we can
    // capture slices.
    let mut iter = schema_contents.char_indices().peekable();

    // Consume the input one code point at a time.
    while let Some((i, c)) = iter.next() {
        // Match on the first code point of the token.
        match c {
            // Match tokens corresponding to symbols.
            ':' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::Colon,
                });
            }
            '.' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::Dot,
                });
            }
            '=' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::Equals,
                });
            }
            '{' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::LeftCurly,
                });
            }
            '[' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::LeftSquare,
                });
            }
            '}' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::RightCurly,
                });
            }
            ']' => {
                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: i + 1,
                    },
                    variant: Variant::RightSquare,
                });
            }

            // If the first code point is alphabetic according to the Unicode derived property,
            // keep reading subsequent alphanumeric code points and underscores to build up an
            // identifier or keyword.
            _ if c.is_alphabetic() || c == '_' || c == RAW_IDENTIFIER_SIGIL => {
                let mut end = schema_contents.len();

                while let Some((j, d)) = iter.peek() {
                    if d.is_alphanumeric() || *d == '_' {
                        iter.next();
                    } else {
                        end = *j;
                        break;
                    }
                }

                if &schema_contents[i..end] == AS_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::As,
                    });
                } else if &schema_contents[i..end] == ASYMMETRIC_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Asymmetric,
                    });
                } else if &schema_contents[i..end] == BOOL_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Bool,
                    });
                } else if &schema_contents[i..end] == BYTES_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Bytes,
                    });
                } else if &schema_contents[i..end] == CHOICE_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Choice,
                    });
                } else if &schema_contents[i..end] == DELETED_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Deleted,
                    });
                } else if &schema_contents[i..end] == F64_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::F64,
                    });
                } else if &schema_contents[i..end] == IMPORT_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Import,
                    });
                } else if &schema_contents[i..end] == OPTIONAL_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Optional,
                    });
                } else if &schema_contents[i..end] == S64_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::S64,
                    });
                } else if &schema_contents[i..end] == STRING_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::String,
                    });
                } else if &schema_contents[i..end] == STRUCT_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Struct,
                    });
                } else if &schema_contents[i..end] == U64_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::U64,
                    });
                } else if &schema_contents[i..end] == UNIT_KEYWORD {
                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Unit,
                    });
                } else {
                    let start = if c == RAW_IDENTIFIER_SIGIL { i + 1 } else { i };

                    if start == end {
                        errors.push(Error::new(
                            "Identifiers cannot be empty.",
                            Some(schema_path),
                            Some((schema_contents, line_index, SourceRange { start: i, end })),
                            None,
                        ));
                    }

                    if schema_contents[start..end].starts_with('_') {
                        errors.push(Error::new(
                            "Identifiers cannot begin with `_`.",
                            Some(schema_path),
                            Some((schema_contents, line_index, SourceRange { start: i, end })),
                            None,
                        ));
                    }

                    tokens.push(Token {
                        source_range: SourceRange { start: i, end },
                        variant: Variant::Identifier(schema_contents[start..end].into()),
                    });
                }
            }

            // If the first code point is a digit, keep reading subsequent digits to build up an
            // integer literal.
            '0'..='9' => {
                let mut end = schema_contents.len();

                while let Some((j, d)) = iter.peek() {
                    if d.is_ascii_digit() {
                        iter.next();
                    } else {
                        end = *j;
                        break;
                    }
                }

                // Try to parse the integer.
                match schema_contents[i..end].parse::<usize>() {
                    Ok(integer) => {
                        tokens.push(Token {
                            source_range: SourceRange { start: i, end },
                            variant: Variant::Integer(integer),
                        });
                    }
                    Err(_) => {
                        errors.push(Error::new(
                            &format!(
                                "Integer {} must be less than 2^64.",
                                schema_contents[i..end].code_str(),
                            ),
                            Some(schema_path),
                            Some((schema_contents, line_index, SourceRange { start: i, end })),
                            None,
                        ));
                    }
                }
            }

            // If the first code point is a single quote, keep reading subsequent code points until
            // a second single quote is reached to build up a path.
            '\'' => {
                let mut end = i;

                for (j, d) in &mut iter {
                    if d == '\'' {
                        end = j;
                        break;
                    }
                }

                if end == i {
                    errors.push(Error::new(
                        &format!(
                            "Path starting here must be terminated by a {}.",
                            "'".code_str(),
                        ),
                        Some(schema_path),
                        Some((
                            schema_contents,
                            line_index,
                            SourceRange {
                                start: i,
                                end: i + 1,
                            },
                        )),
                        None,
                    ));
                } else {
                    tokens.push(Token {
                        source_range: SourceRange {
                            start: i,
                            end: end + 1,
                        },
                        variant: Variant::Path(Path::new(&schema_contents[i + 1..end]).to_owned()),
                    });
                }
            }

            // Read comments.
            '#' => {
                let mut line_start = i + 1;
                let mut line_end = schema_contents.len();
                let mut lines = vec![];

                loop {
                    for (j, d) in &mut iter {
                        if d == '\n' {
                            line_end = j;
                            break;
                        }
                    }

                    lines.push(schema_contents[line_start..line_end].trim().to_owned());

                    while let Some((_, d)) = iter.peek() {
                        if d.is_whitespace() && *d != '\n' {
                            iter.next();
                        } else {
                            break;
                        }
                    }

                    if let Some((j, '#')) = iter.peek() {
                        line_start = j + 1;
                        line_end = schema_contents.len();
                    } else {
                        break;
                    }
                }

                let mut paragraphs = vec![];
                let mut paragraph = String::new();

                for line in lines {
                    if line.is_empty() && !paragraph.is_empty() {
                        paragraphs.push(paragraph.clone());
                        paragraph.clear();
                    } else if !line.is_empty() {
                        if !paragraph.is_empty() {
                            paragraph.push(' ');
                        }

                        paragraph.push_str(&line);
                    }
                }

                if !paragraph.is_empty() {
                    paragraphs.push(paragraph.clone());
                }

                tokens.push(Token {
                    source_range: SourceRange {
                        start: i,
                        end: line_end,
                    },
                    variant: Variant::Comment(paragraphs),
                });
            }

            // Skip whitespace.
            _ if c.is_whitespace() => {}

            // If we made it this far, the input contains something unexpected.
            _ => {
                // We are going to attempt to compute the problematic grapheme cluster. Note that we
                // might already be in the middle of a grapheme cluster, in which case this logic
                // will compute the remainder of it. To start, we create a cursor that represents
                // the current position within the source.
                let mut cursor = GraphemeCursor::new(i, schema_contents.len(), true);

                // Now we find the next grapheme cluster boundary. The first `unwrap` is
                // justified because the docs indicate the only two errors that can be returned are
                // `GraphemeIncomplete::PreContext` and `GraphemeIncomplete::NextChunk`, but the
                // docs also state that these two conditions are impossible since the chunk we
                // provide is the whole source string. The second `unwrap` is justified because we
                // only get `None` in the case where we're at the end of the string, and we know
                // we're not at the end of the string since otherwise we would have exited the loop
                // already.
                let end = cursor.next_boundary(schema_contents, 0).unwrap().unwrap();

                // Now that we've computed the grapheme cluster, construct and report the error.
                errors.push(Error::new(
                    &format!("Unexpected symbol {}.", schema_contents[i..end].code_str()),
                    Some(schema_path),
                    Some((
                        schema_contents,
                        line_index,
                        SourceRange { start: i, end: i },
                    )),
                    None,
                ));
            }
        }
    }

    // If there are any errors at this point, return them.
    if !errors.is_empty() {
        return Err(errors);
    }

    // If we made it this far, we've successfully tokenized the input.
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_fails, assert_same,
        error::SourceRange,
        line_index::LineIndex,
        token::{
            AS_KEYWORD, ASYMMETRIC_KEYWORD, BOOL_KEYWORD, BYTES_KEYWORD, CHOICE_KEYWORD,
            DELETED_KEYWORD, F64_KEYWORD, IMPORT_KEYWORD, OPTIONAL_KEYWORD, S64_KEYWORD,
            STRING_KEYWORD, STRUCT_KEYWORD, Token, U64_KEYWORD, UNIT_KEYWORD, Variant,
        },
        tokenizer::{RAW_IDENTIFIER_SIGIL, tokenize},
    };
    use std::{fmt::Write, path::Path};

    #[test]
    fn tokenize_example() {
        let source = "
            # This is a struct.
            struct Foo {
            }

            # This is a choice.
            choice Bar {
            }
        ";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![
                Token {
                    source_range: SourceRange { start: 13, end: 32 },
                    variant: Variant::Comment(vec!["This is a struct.".to_owned()]),
                },
                Token {
                    source_range: SourceRange { start: 45, end: 51 },
                    variant: Variant::Struct,
                },
                Token {
                    source_range: SourceRange { start: 52, end: 55 },
                    variant: Variant::Identifier("Foo".into()),
                },
                Token {
                    source_range: SourceRange { start: 56, end: 57 },
                    variant: Variant::LeftCurly,
                },
                Token {
                    source_range: SourceRange { start: 70, end: 71 },
                    variant: Variant::RightCurly,
                },
                Token {
                    source_range: SourceRange {
                        start: 85,
                        end: 104,
                    },
                    variant: Variant::Comment(vec!["This is a choice.".to_owned()]),
                },
                Token {
                    source_range: SourceRange {
                        start: 117,
                        end: 123,
                    },
                    variant: Variant::Choice,
                },
                Token {
                    source_range: SourceRange {
                        start: 124,
                        end: 127,
                    },
                    variant: Variant::Identifier("Bar".into()),
                },
                Token {
                    source_range: SourceRange {
                        start: 128,
                        end: 129,
                    },
                    variant: Variant::LeftCurly,
                },
                Token {
                    source_range: SourceRange {
                        start: 142,
                        end: 143,
                    },
                    variant: Variant::RightCurly,
                },
            ],
        );
    }

    #[test]
    fn tokenize_empty() {
        let source = "";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![],
        );
    }

    #[test]
    fn tokenize_whitespace() {
        let source = " \t\n";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![],
        );
    }

    #[test]
    fn tokenize_as() {
        let source = AS_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: AS_KEYWORD.len(),
                },
                variant: Variant::As,
            }],
        );
    }

    #[test]
    fn tokenize_asymmetric() {
        let source = ASYMMETRIC_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: ASYMMETRIC_KEYWORD.len(),
                },
                variant: Variant::Asymmetric,
            }],
        );
    }

    #[test]
    fn tokenize_bool() {
        let source = BOOL_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: BOOL_KEYWORD.len(),
                },
                variant: Variant::Bool,
            }],
        );
    }

    #[test]
    fn tokenize_bytes() {
        let source = BYTES_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: BYTES_KEYWORD.len(),
                },
                variant: Variant::Bytes,
            }],
        );
    }

    #[test]
    fn tokenize_choice() {
        let source = CHOICE_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: CHOICE_KEYWORD.len(),
                },
                variant: Variant::Choice,
            }],
        );
    }

    #[test]
    fn tokenize_colon() {
        let source = ":";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::Colon,
            }],
        );
    }

    #[test]
    fn tokenize_comment_simple() {
        let source = "# Hello, World!";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 15 },
                variant: Variant::Comment(vec!["Hello, World!".to_owned()]),
            }],
        );
    }

    #[test]
    fn tokenize_comment_complex() {
        let source = " # \n # Hello, \n # World! \n # \n # Hello, \n # Earth! \n # ";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 1, end: 55 },
                variant: Variant::Comment(vec![
                    "Hello, World!".to_owned(),
                    "Hello, Earth!".to_owned(),
                ]),
            }],
        );
    }

    #[test]
    fn tokenize_deleted() {
        let source = DELETED_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: DELETED_KEYWORD.len(),
                },
                variant: Variant::Deleted,
            }],
        );
    }

    #[test]
    fn tokenize_dot() {
        let source = ".";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::Dot,
            }],
        );
    }

    #[test]
    fn tokenize_equals() {
        let source = "=";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::Equals,
            }],
        );
    }

    #[test]
    fn tokenize_f64() {
        let source = F64_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: F64_KEYWORD.len(),
                },
                variant: Variant::F64,
            }],
        );
    }

    #[test]
    fn tokenize_bare_identifier() {
        let source = "\u{5e78}\u{798f}";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 6 },
                variant: Variant::Identifier("\u{5e78}\u{798f}".into()),
            }],
        );
    }

    #[test]
    fn tokenize_raw_identifier() {
        let source = format!("{RAW_IDENTIFIER_SIGIL}{STRUCT_KEYWORD}");

        assert_same!(
            tokenize(Path::new("foo.t"), &source, &LineIndex::new(&source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 7 },
                variant: Variant::Identifier(STRUCT_KEYWORD.into()),
            }],
        );
    }

    #[test]
    fn tokenize_bare_identifier_underscore_prefix() {
        let source = "_foo";

        assert_fails!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)),
            "Identifiers cannot begin with `_`.",
        );
    }

    #[test]
    fn tokenize_raw_identifier_underscore_prefix() {
        let source = format!("{}{}", RAW_IDENTIFIER_SIGIL, "_foo");

        assert_fails!(
            tokenize(Path::new("foo.t"), &source, &LineIndex::new(&source)),
            "Identifiers cannot begin with `_`.",
        );
    }

    #[test]
    fn tokenize_import() {
        let source = IMPORT_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: IMPORT_KEYWORD.len(),
                },
                variant: Variant::Import,
            }],
        );
    }

    #[test]
    fn tokenize_integer_literal_valid() {
        let source = "42";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 2 },
                variant: Variant::Integer(42),
            }],
        );
    }

    #[test]
    fn tokenize_integer_literal_out_of_range() {
        let source = "18446744073709551616";

        assert_fails!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)),
            "Integer `18446744073709551616` must be less than 2^64.",
        );
    }

    #[test]
    fn tokenize_left_curly() {
        let source = "{";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::LeftCurly,
            }],
        );
    }

    #[test]
    fn tokenize_left_square() {
        let source = "[";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::LeftSquare,
            }],
        );
    }

    #[test]
    fn tokenize_optional() {
        let source = OPTIONAL_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: OPTIONAL_KEYWORD.len(),
                },
                variant: Variant::Optional,
            }],
        );
    }

    #[test]
    fn tokenize_path_non_empty() {
        let source = "'bar.t'";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 7 },
                variant: Variant::Path(Path::new("bar.t").to_owned()),
            }],
        );
    }

    #[test]
    fn tokenize_path_empty() {
        let source = "''";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 2 },
                variant: Variant::Path(Path::new("").to_owned()),
            }],
        );
    }

    #[test]
    fn tokenize_path_non_terminated() {
        let source = "'bar.t";

        assert_fails!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)),
            "Path starting here must be terminated by a `\'`.",
        );
    }

    #[test]
    fn tokenize_right_curly() {
        let source = "}";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::RightCurly,
            }],
        );
    }

    #[test]
    fn tokenize_right_square() {
        let source = "]";

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::RightSquare,
            }],
        );
    }

    #[test]
    fn tokenize_s64() {
        let source = S64_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: S64_KEYWORD.len(),
                },
                variant: Variant::S64,
            }],
        );
    }

    #[test]
    fn tokenize_string() {
        let source = STRING_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: STRING_KEYWORD.len(),
                },
                variant: Variant::String,
            }],
        );
    }

    #[test]
    fn tokenize_struct() {
        let source = STRUCT_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: STRUCT_KEYWORD.len(),
                },
                variant: Variant::Struct,
            }],
        );
    }

    #[test]
    fn tokenize_u64() {
        let source = U64_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: U64_KEYWORD.len(),
                },
                variant: Variant::U64,
            }],
        );
    }

    #[test]
    fn tokenize_unit() {
        let source = UNIT_KEYWORD;

        assert_same!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)).unwrap(),
            vec![Token {
                source_range: SourceRange {
                    start: 0,
                    end: UNIT_KEYWORD.len(),
                },
                variant: Variant::Unit,
            }],
        );
    }

    #[test]
    fn tokenize_unexpected_symbol() {
        let source = "\u{1f610}\u{fe0f}";

        assert_fails!(
            tokenize(Path::new("foo.t"), source, &LineIndex::new(source)),
            "Unexpected symbol `\u{1f610}\u{fe0f}`.",
        );
    }
}
