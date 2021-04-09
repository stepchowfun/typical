use crate::{
    error::{listing, throw, Error},
    format::CodeStr,
    token::{
        Token, Variant, AS_KEYWORD, CHOICE_KEYWORD, IMPORT_KEYWORD, RESTRICTED_KEYWORD,
        STRUCT_KEYWORD,
    },
};
use std::path::Path;
use unicode_segmentation::GraphemeCursor;

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
                    source_range: (i, i + 1),
                    variant: Variant::Colon,
                });
            }
            '.' => {
                tokens.push(Token {
                    source_range: (i, i + 1),
                    variant: Variant::Dot,
                });
            }
            '=' => {
                tokens.push(Token {
                    source_range: (i, i + 1),
                    variant: Variant::Equals,
                });
            }
            '{' => {
                tokens.push(Token {
                    source_range: (i, i + 1),
                    variant: Variant::LeftCurly,
                });
            }
            '}' => {
                tokens.push(Token {
                    source_range: (i, i + 1),
                    variant: Variant::RightCurly,
                });
            }

            // If the first code point is alphabetic according to the Unicode derived property,
            // keep reading subsequent alphanumeric code points and underscores to build up an
            // identifier or keyword.
            _ if c.is_alphabetic() || c == '_' => {
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
                        source_range: (i, end),
                        variant: Variant::As,
                    });
                } else if &schema_contents[i..end] == CHOICE_KEYWORD {
                    tokens.push(Token {
                        source_range: (i, end),
                        variant: Variant::Choice,
                    });
                } else if &schema_contents[i..end] == IMPORT_KEYWORD {
                    tokens.push(Token {
                        source_range: (i, end),
                        variant: Variant::Import,
                    });
                } else if &schema_contents[i..end] == RESTRICTED_KEYWORD {
                    tokens.push(Token {
                        source_range: (i, end),
                        variant: Variant::Restricted,
                    });
                } else if &schema_contents[i..end] == STRUCT_KEYWORD {
                    tokens.push(Token {
                        source_range: (i, end),
                        variant: Variant::Struct,
                    });
                } else {
                    tokens.push(Token {
                        source_range: (i, end),
                        variant: Variant::Identifier(schema_contents[i..end].to_owned()),
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
                            source_range: (i, end),
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
                            Some(&listing(schema_contents, (i, end))),
                            None,
                        ));
                    }
                }
            }

            // If the first code point is a single quote, keep reading subsequent code points until
            // a second single quote is reached to build up a path.
            '\'' => {
                let mut end = i;

                while let Some((j, d)) = iter.next() {
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
                        Some(&listing(schema_contents, (i, i + 1))),
                        None,
                    ));
                } else {
                    tokens.push(Token {
                        source_range: (i, end + 1),
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
                    Some(&listing(schema_contents, (i, end))),
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
        token::{
            Token, Variant, AS_KEYWORD, CHOICE_KEYWORD, IMPORT_KEYWORD, RESTRICTED_KEYWORD,
            STRUCT_KEYWORD,
        },
        tokenizer::tokenize,
    };
    use std::path::Path;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn tokenize_example() {
        let source = "
            import 'bar.t' as bar

            # This is a struct.
            struct plugh {
              qux: bar.Foo = 0
              corge: restricted int = 1
            }

            # This is a choice.
            choice zyzzy {
              grault: bar.Bar = 0
              garply: restricted int = 1
            }
        ";

        assert_same!(
            tokenize(Path::new("foo.t"), source).unwrap(),
            vec![
                Token {
                    source_range: (13, 19),
                    variant: Variant::Import,
                },
                Token {
                    source_range: (20, 27),
                    variant: Variant::Path(Path::new("bar.t").to_owned()),
                },
                Token {
                    source_range: (28, 30),
                    variant: Variant::As,
                },
                Token {
                    source_range: (31, 34),
                    variant: Variant::Identifier("bar".to_owned()),
                },
                Token {
                    source_range: (80, 86),
                    variant: Variant::Struct,
                },
                Token {
                    source_range: (87, 92),
                    variant: Variant::Identifier("plugh".to_owned()),
                },
                Token {
                    source_range: (93, 94),
                    variant: Variant::LeftCurly,
                },
                Token {
                    source_range: (109, 112),
                    variant: Variant::Identifier("qux".to_owned()),
                },
                Token {
                    source_range: (112, 113),
                    variant: Variant::Colon,
                },
                Token {
                    source_range: (114, 117),
                    variant: Variant::Identifier("bar".to_owned()),
                },
                Token {
                    source_range: (117, 118),
                    variant: Variant::Dot,
                },
                Token {
                    source_range: (118, 121),
                    variant: Variant::Identifier("Foo".to_owned()),
                },
                Token {
                    source_range: (122, 123),
                    variant: Variant::Equals,
                },
                Token {
                    source_range: (124, 125),
                    variant: Variant::Integer(0),
                },
                Token {
                    source_range: (140, 145),
                    variant: Variant::Identifier("corge".to_owned()),
                },
                Token {
                    source_range: (145, 146),
                    variant: Variant::Colon,
                },
                Token {
                    source_range: (147, 157),
                    variant: Variant::Restricted,
                },
                Token {
                    source_range: (158, 161),
                    variant: Variant::Identifier("int".to_owned()),
                },
                Token {
                    source_range: (162, 163),
                    variant: Variant::Equals,
                },
                Token {
                    source_range: (164, 165),
                    variant: Variant::Integer(1),
                },
                Token {
                    source_range: (178, 179),
                    variant: Variant::RightCurly,
                },
                Token {
                    source_range: (225, 231),
                    variant: Variant::Choice,
                },
                Token {
                    source_range: (232, 237),
                    variant: Variant::Identifier("zyzzy".to_owned()),
                },
                Token {
                    source_range: (238, 239),
                    variant: Variant::LeftCurly,
                },
                Token {
                    source_range: (254, 260),
                    variant: Variant::Identifier("grault".to_owned()),
                },
                Token {
                    source_range: (260, 261),
                    variant: Variant::Colon,
                },
                Token {
                    source_range: (262, 265),
                    variant: Variant::Identifier("bar".to_owned()),
                },
                Token {
                    source_range: (265, 266),
                    variant: Variant::Dot,
                },
                Token {
                    source_range: (266, 269),
                    variant: Variant::Identifier("Bar".to_owned()),
                },
                Token {
                    source_range: (270, 271),
                    variant: Variant::Equals,
                },
                Token {
                    source_range: (272, 273),
                    variant: Variant::Integer(0),
                },
                Token {
                    source_range: (288, 294),
                    variant: Variant::Identifier("garply".to_owned()),
                },
                Token {
                    source_range: (294, 295),
                    variant: Variant::Colon,
                },
                Token {
                    source_range: (296, 306),
                    variant: Variant::Restricted,
                },
                Token {
                    source_range: (307, 310),
                    variant: Variant::Identifier("int".to_owned()),
                },
                Token {
                    source_range: (311, 312),
                    variant: Variant::Equals,
                },
                Token {
                    source_range: (313, 314),
                    variant: Variant::Integer(1),
                },
                Token {
                    source_range: (327, 328),
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
                source_range: (0, AS_KEYWORD.len()),
                variant: Variant::As,
            }],
        );
    }

    #[test]
    fn tokenize_choice() {
        assert_same!(
            tokenize(Path::new("foo.t"), CHOICE_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, CHOICE_KEYWORD.len()),
                variant: Variant::Choice,
            }],
        );
    }

    #[test]
    fn tokenize_colon() {
        assert_same!(
            tokenize(Path::new("foo.t"), ":").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::Colon,
            }],
        );
    }

    #[test]
    fn tokenize_dot() {
        assert_same!(
            tokenize(Path::new("foo.t"), ".").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::Dot,
            }],
        );
    }

    #[test]
    fn tokenize_equals() {
        assert_same!(
            tokenize(Path::new("foo.t"), "=").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::Equals,
            }],
        );
    }

    #[test]
    fn tokenize_identifier() {
        assert_same!(
            tokenize(Path::new("foo.t"), "\u{5e78}\u{798f}").unwrap(),
            vec![Token {
                source_range: (0, 6),
                variant: Variant::Identifier("\u{5e78}\u{798f}".to_owned()),
            }],
        );
    }

    #[test]
    fn tokenize_import() {
        assert_same!(
            tokenize(Path::new("foo.t"), IMPORT_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, IMPORT_KEYWORD.len()),
                variant: Variant::Import,
            }],
        );
    }

    #[test]
    fn tokenize_integer_literal_valid() {
        assert_same!(
            tokenize(Path::new("foo.t"), "42").unwrap(),
            vec![Token {
                source_range: (0, 2),
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
                source_range: (0, 1),
                variant: Variant::LeftCurly,
            }],
        );
    }

    #[test]
    fn tokenize_path_non_empty() {
        assert_same!(
            tokenize(Path::new("foo.t"), "'bar.t'").unwrap(),
            vec![Token {
                source_range: (0, 7),
                variant: Variant::Path(Path::new("bar.t").to_owned()),
            }],
        );
    }

    #[test]
    fn tokenize_path_empty() {
        assert_same!(
            tokenize(Path::new("foo.t"), "''").unwrap(),
            vec![Token {
                source_range: (0, 2),
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
    fn tokenize_restricted() {
        assert_same!(
            tokenize(Path::new("foo.t"), RESTRICTED_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, RESTRICTED_KEYWORD.len()),
                variant: Variant::Restricted,
            }],
        );
    }

    #[test]
    fn tokenize_right_curly() {
        assert_same!(
            tokenize(Path::new("foo.t"), "}").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::RightCurly,
            }],
        );
    }

    #[test]
    fn tokenize_struct() {
        assert_same!(
            tokenize(Path::new("foo.t"), STRUCT_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, STRUCT_KEYWORD.len()),
                variant: Variant::Struct,
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
