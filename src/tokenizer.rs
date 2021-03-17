use crate::{
    error::{throw, Error},
    format::CodeStr,
    token::{Token, Variant, ENUM_KEYWORD, RESTRICTED_KEYWORD, STRUCT_KEYWORD},
};
use std::path::Path;
use unicode_segmentation::GraphemeCursor;

// Tokenize the contents of a schema file.
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
pub fn tokenize<'a>(
    schema_path: Option<&'a Path>,
    schema_contents: &'a str,
) -> Result<Vec<Token<'a>>, Vec<Error>> {
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
            '\n' => {
                // [tag:line_break] [tag:tokens_nonempty]
                if !tokens.is_empty()
                    && match tokens.last().unwrap().variant /* [ref:tokens_nonempty] */ {
                        Variant::Colon
                        | Variant::Enum
                        | Variant::Equals
                        | Variant::Identifier(_)
                        | Variant::LeftCurly
                        | Variant::Restricted
                        | Variant::RightCurly
                        /* [tag:no_consecutive_separators] */
                        | Variant::Separator
                        | Variant::Struct => false,
                        Variant::IntegerLiteral(_) => true,
                    }
                {
                    tokens.push(Token {
                        source_range: (i, i + 1),
                        variant: Variant::Separator,
                    });
                }
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

                if &schema_contents[i..end] == ENUM_KEYWORD {
                    tokens.push(Token {
                        source_range: (i, end),
                        variant: Variant::Enum,
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
                        variant: Variant::Identifier(&schema_contents[i..end]),
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
                match schema_contents[i..end].parse::<u64>() {
                    Ok(integer) => {
                        tokens.push(Token {
                            source_range: (i, end),
                            variant: Variant::IntegerLiteral(integer),
                        });
                    }
                    Err(_) => {
                        errors.push(throw(
                            &format!(
                                "Integer {} must be less than 2^64.",
                                &schema_contents[i..end].code_str(),
                            ),
                            schema_path,
                            Some((schema_contents, (i, end))),
                        ));
                    }
                }
            }

            // Skip whitespace. Note that line breaks are handled above [ref:line_break].
            _ if c.is_whitespace() => continue,

            // Skip comments. Don't skip the terminating line break, if it exists [ref:line_break].
            '#' => {
                while let Some((j, _)) = iter.next() {
                    if iter.peek() == Some(&(j + 1, '\n')) {
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
                errors.push(throw(
                    &format!("Unexpected symbol {}.", &schema_contents[i..end].code_str()),
                    schema_path,
                    Some((schema_contents, (i, end))),
                ));
            }
        }
    }

    // If there are any errors at this point, return them.
    if !errors.is_empty() {
        return Err(errors);
    }

    // Remove trailing line break terminators and line break terminators before certain token types.
    let mut filtered_tokens = vec![];
    let mut tokens_iter = tokens.iter().peekable();
    while let Some(token) = tokens_iter.next() {
        if let Variant::Separator = token.variant {
            if let Some(next_token) = tokens_iter.peek() {
                if match next_token.variant {
                    Variant::Colon
                    | Variant::Enum
                    | Variant::Equals
                    | Variant::IntegerLiteral(_)
                    | Variant::LeftCurly
                    | Variant::Restricted
                    | Variant::RightCurly
                    | Variant::Struct => false,
                    Variant::Identifier(_) => true,
                    Variant::Separator => {
                        // [ref:no_consecutive_separators]
                        panic!("Two consecutive separators were found.");
                    }
                } {
                    filtered_tokens.push(token.clone());
                }
            }
        } else {
            filtered_tokens.push(token.clone());
        }
    }

    // If we made it this far, we've successfully tokenized the input.
    Ok(filtered_tokens)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_fails, assert_same,
        token::{Token, Variant, ENUM_KEYWORD, RESTRICTED_KEYWORD, STRUCT_KEYWORD},
        tokenizer::tokenize,
    };

    #[test]
    fn tokenize_empty() {
        assert_same!(tokenize(None, "").unwrap(), vec![]);
    }

    #[test]
    fn tokenize_whitespace() {
        assert_same!(tokenize(None, " \t\n").unwrap(), vec![]);
    }

    #[test]
    fn tokenize_comment() {
        assert_same!(tokenize(None, "# Hello, World!").unwrap(), vec![]);
    }

    #[test]
    fn tokenize_colon() {
        assert_same!(
            tokenize(None, ":").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::Colon,
            }],
        );
    }

    #[test]
    fn tokenize_enum() {
        assert_same!(
            tokenize(None, ENUM_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, ENUM_KEYWORD.len()),
                variant: Variant::Enum,
            }],
        );
    }

    #[test]
    fn tokenize_equals() {
        assert_same!(
            tokenize(None, "=").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::Equals,
            }],
        );
    }

    #[test]
    fn tokenize_identifier() {
        assert_same!(
            tokenize(None, "\u{5e78}\u{798f}").unwrap(),
            vec![Token {
                source_range: (0, 6),
                variant: Variant::Identifier("\u{5e78}\u{798f}"),
            }],
        );
    }

    #[test]
    fn tokenize_integer_literal() {
        assert_same!(
            tokenize(None, "42").unwrap(),
            vec![Token {
                source_range: (0, 2),
                variant: Variant::IntegerLiteral(42),
            }],
        );
    }

    #[test]
    fn tokenize_left_curly() {
        assert_same!(
            tokenize(None, "{").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::LeftCurly,
            }],
        );
    }

    #[test]
    fn tokenize_restricted() {
        assert_same!(
            tokenize(None, RESTRICTED_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, RESTRICTED_KEYWORD.len()),
                variant: Variant::Restricted,
            }],
        );
    }

    #[test]
    fn tokenize_right_curly() {
        assert_same!(
            tokenize(None, "}").unwrap(),
            vec![Token {
                source_range: (0, 1),
                variant: Variant::RightCurly,
            }],
        );
    }

    #[test]
    fn tokenize_separator() {
        assert_same!(
            tokenize(None, "\n\n42\n\nfoo\n\n").unwrap(),
            vec![
                Token {
                    source_range: (2, 4),
                    variant: Variant::IntegerLiteral(42),
                },
                Token {
                    source_range: (4, 5),
                    variant: Variant::Separator,
                },
                Token {
                    source_range: (6, 9),
                    variant: Variant::Identifier("foo"),
                },
            ],
        );
    }

    #[test]
    fn tokenize_struct() {
        assert_same!(
            tokenize(None, STRUCT_KEYWORD).unwrap(),
            vec![Token {
                source_range: (0, STRUCT_KEYWORD.len()),
                variant: Variant::Struct,
            }],
        );
    }

    #[test]
    fn tokenize_unexpected_code_point() {
        assert_fails!(tokenize(None, "$"), "Unexpected symbol");
    }
}
