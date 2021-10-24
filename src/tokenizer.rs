use {
    crate::{
        error::{listing, throw, Error, SourceRange},
        format::CodeStr,
        token::{
            Token, Variant, ASYMMETRIC_KEYWORD, AS_KEYWORD, BOOL_KEYWORD, BYTES_KEYWORD,
            CHOICE_KEYWORD, DELETED_KEYWORD, F64_KEYWORD, IMPORT_KEYWORD, OPTIONAL_KEYWORD,
            S64_KEYWORD, STRING_KEYWORD, STRUCT_KEYWORD, U64_KEYWORD, UNIT_KEYWORD,
        },
    },
    std::path::Path,
    unicode_segmentation::GraphemeCursor,
};

// An identifier can be prefixed with this character to avoid being parsed as a keyword.
const RAW_IDENTIFIER_SIGIL: char = '$';

// Tokenize the contents of a schema file.
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
pub fn tokenize(schema_path: &Path, schema_contents: &str) -> Result<Vec<Token>, Vec<Error>> {
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
                        errors.push(throw::<Error>(
                            "Identifiers cannot be empty.",
                            Some(schema_path),
                            Some(&listing(schema_contents, SourceRange { start: i, end })),
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
                    if ('0'..='9').contains(d) {
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
                        errors.push(throw::<Error>(
                            &format!(
                                "Integer {} must be less than 2^64.",
                                &schema_contents[i..end].code_str(),
                            ),
                            Some(schema_path),
                            Some(&listing(schema_contents, SourceRange { start: i, end })),
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
                    errors.push(throw::<Error>(
                        &format!(
                            "Path starting here must be terminated by a {}.",
                            "'".code_str(),
                        ),
                        Some(schema_path),
                        Some(&listing(
                            schema_contents,
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

            // Skip whitespace.
            _ if c.is_whitespace() => continue,

            // Skip comments.
            '#' => {
                for (_, d) in &mut iter {
                    if d == '\n' {
                        break;
                    }
                }
            }

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
                errors.push(throw::<Error>(
                    &format!("Unexpected symbol {}.", &schema_contents[i..end].code_str()),
                    Some(schema_path),
                    Some(&listing(schema_contents, SourceRange { start: i, end: i })),
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
    use {
        crate::{
            assert_fails, assert_same,
            error::SourceRange,
            token::{
                Token, Variant, ASYMMETRIC_KEYWORD, AS_KEYWORD, BOOL_KEYWORD, BYTES_KEYWORD,
                CHOICE_KEYWORD, DELETED_KEYWORD, F64_KEYWORD, IMPORT_KEYWORD, OPTIONAL_KEYWORD,
                S64_KEYWORD, STRING_KEYWORD, STRUCT_KEYWORD, U64_KEYWORD, UNIT_KEYWORD,
            },
            tokenizer::{tokenize, RAW_IDENTIFIER_SIGIL},
        },
        std::path::Path,
    };

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
            tokenize(Path::new("foo.t"), source).unwrap(),
            vec![
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
        assert_same!(tokenize(Path::new("foo.t"), "").unwrap(), vec![]);
    }

    #[test]
    fn tokenize_whitespace() {
        assert_same!(tokenize(Path::new("foo.t"), " \t\n").unwrap(), vec![]);
    }

    #[test]
    fn tokenize_comment() {
        assert_same!(
            tokenize(Path::new("foo.t"), "# Hello, World!").unwrap(),
            vec![],
        );
    }

    #[test]
    fn tokenize_as() {
        assert_same!(
            tokenize(Path::new("foo.t"), AS_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), ASYMMETRIC_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), BOOL_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), BYTES_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), CHOICE_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), ":").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::Colon,
            }],
        );
    }

    #[test]
    fn tokenize_deleted() {
        assert_same!(
            tokenize(Path::new("foo.t"), DELETED_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), ".").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::Dot,
            }],
        );
    }

    #[test]
    fn tokenize_equals() {
        assert_same!(
            tokenize(Path::new("foo.t"), "=").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::Equals,
            }],
        );
    }

    #[test]
    fn tokenize_f64() {
        assert_same!(
            tokenize(Path::new("foo.t"), F64_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), "\u{5e78}\u{798f}").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 6 },
                variant: Variant::Identifier("\u{5e78}\u{798f}".into()),
            }],
        );
    }

    #[test]
    fn tokenize_raw_identifier() {
        assert_same!(
            tokenize(
                Path::new("foo.t"),
                &format!("{}{}", RAW_IDENTIFIER_SIGIL, STRUCT_KEYWORD),
            )
            .unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 7 },
                variant: Variant::Identifier(STRUCT_KEYWORD.into()),
            }],
        );
    }

    #[test]
    fn tokenize_import() {
        assert_same!(
            tokenize(Path::new("foo.t"), IMPORT_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), "42").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 2 },
                variant: Variant::Integer(42),
            }],
        );
    }

    #[test]
    fn tokenize_integer_literal_out_of_range() {
        assert_fails!(
            tokenize(Path::new("foo.t"), "18446744073709551616"),
            "Integer `18446744073709551616` must be less than 2^64.",
        );
    }

    #[test]
    fn tokenize_left_curly() {
        assert_same!(
            tokenize(Path::new("foo.t"), "{").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::LeftCurly,
            }],
        );
    }

    #[test]
    fn tokenize_left_square() {
        assert_same!(
            tokenize(Path::new("foo.t"), "[").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::LeftSquare,
            }],
        );
    }

    #[test]
    fn tokenize_optional() {
        assert_same!(
            tokenize(Path::new("foo.t"), OPTIONAL_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), "'bar.t'").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 7 },
                variant: Variant::Path(Path::new("bar.t").to_owned()),
            }],
        );
    }

    #[test]
    fn tokenize_path_empty() {
        assert_same!(
            tokenize(Path::new("foo.t"), "''").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 2 },
                variant: Variant::Path(Path::new("").to_owned()),
            }],
        );
    }

    #[test]
    fn tokenize_path_non_terminated() {
        assert_fails!(
            tokenize(Path::new("foo.t"), "'bar.t"),
            "Path starting here must be terminated by a `\'`.",
        );
    }

    #[test]
    fn tokenize_right_curly() {
        assert_same!(
            tokenize(Path::new("foo.t"), "}").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::RightCurly,
            }],
        );
    }

    #[test]
    fn tokenize_right_square() {
        assert_same!(
            tokenize(Path::new("foo.t"), "]").unwrap(),
            vec![Token {
                source_range: SourceRange { start: 0, end: 1 },
                variant: Variant::RightSquare,
            }],
        );
    }

    #[test]
    fn tokenize_s64() {
        assert_same!(
            tokenize(Path::new("foo.t"), S64_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), STRING_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), STRUCT_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), U64_KEYWORD).unwrap(),
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
        assert_same!(
            tokenize(Path::new("foo.t"), UNIT_KEYWORD).unwrap(),
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
        assert_fails!(
            tokenize(Path::new("foo.t"), "\u{1f610}\u{fe0f}"),
            "Unexpected symbol `\u{1f610}\u{fe0f}`.",
        );
    }
}
