use {
    crate::{
        error::{listing, throw, Error, SourceRange},
        format::CodeStr,
        identifier::Identifier,
        schema, token,
    },
    std::{char::REPLACEMENT_CHARACTER, collections::BTreeMap, path::Path},
};

// This function computes the source range for a token, or the empty range at the end of the source
// file in the case where the given position is at the end of the token stream.
fn token_source_range(tokens: &[token::Token], position: usize) -> SourceRange {
    if position == tokens.len() {
        tokens
            .last()
            .map_or(SourceRange { start: 0, end: 0 }, |token| SourceRange {
                start: token.source_range.end,
                end: token.source_range.end,
            })
    } else {
        tokens[position].source_range
    }
}

// This function computes the source range for a span of tokens. `start` is inclusive, and `end` is
// exclusive. If `start` is equal to `end`, the empty range for the token at `start` is returned
// (or the empty range at the end of the source file if `start` is at the end of the stream).
fn span_tokens(tokens: &[token::Token], start: usize, end: usize) -> SourceRange {
    if start == tokens.len() {
        token_source_range(tokens, start)
    } else if start == end {
        let pos = tokens[start].source_range.start;

        SourceRange {
            start: pos,
            end: pos,
        }
    } else {
        SourceRange {
            start: token_source_range(tokens, start).start,
            end: token_source_range(tokens, end - 1).end,
        }
    }
}

// This function constructs a generic error that just complains about a particular token not being
// found where expected.
fn unexpected_token(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: usize,
    expectation: &str,
) -> Error {
    let source_range = token_source_range(tokens, position);

    if tokens.is_empty() {
        throw::<Error>(
            &format!("Expected {}, but the file is empty.", expectation),
            Some(source_path),
            Some(&listing(source_contents, source_range)),
            None,
        )
    } else if position == tokens.len() {
        throw::<Error>(
            &format!("Expected {} at the end of the file.", expectation),
            Some(source_path),
            Some(&listing(source_contents, source_range)),
            None,
        )
    } else {
        throw::<Error>(
            &format!(
                "Expected {}, but encountered {}.",
                expectation,
                tokens[position].to_string().code_str(),
            ),
            Some(source_path),
            Some(&listing(source_contents, source_range)),
            None,
        )
    }
}

// This macro consumes a single token (with no arguments). On a successful parse, this macro
// evaluates to the next token position (i.e., `$next + 1`). If the parse fails, this macro returns
// the given `$error_value`.
macro_rules! consume_token_0 {
    (
        $source_path:expr,
        $source_contents:expr,
        $tokens:expr,
        $position:expr,
        $errors:ident,
        $variant:ident,
        $error_value:expr $(,)?
    ) => {{
        // Macros are call-by-name, but we want call-by-value (or at least call-by-need) to avoid
        // accidentally evaluating arguments multiple times. Here we force eager evaluation.
        let source_path = $source_path;
        let source_contents = $source_contents;
        let tokens = $tokens;
        let position = $position;

        // Make sure we have a token to parse.
        if *position == tokens.len() {
            $errors.push(unexpected_token(
                source_path,
                source_contents,
                tokens,
                *position,
                &format!("{}", token::Variant::$variant.to_string().code_str()),
            ));

            return $error_value;
        }

        // Check if the token matches what we expect.
        if let token::Variant::$variant = tokens[*position].variant {
            *position += 1;
        } else {
            $errors.push(unexpected_token(
                source_path,
                source_contents,
                tokens,
                *position,
                &format!("{}", token::Variant::$variant.to_string().code_str()),
            ));

            return $error_value;
        }
    }};
}

// This macro consumes a single token (with one argument). On a successful parse, this macro
// evaluates to the argument paired with the next token position (i.e., `$next + 1`). If the parse
// fails, this macro returns the given `$error_value`.
macro_rules! consume_token_1 {
    (
        $source_path:expr,
        $source_contents:expr,
        $tokens:expr,
        $position:expr,
        $errors:ident,
        $variant:ident,
        $expectation:expr,
        $error_value:expr $(,)?
    ) => {{
        // Macros are call-by-name, but we want call-by-value (or at least call-by-need) to avoid
        // accidentally evaluating arguments multiple times. Here we force eager evaluation.
        let source_path = $source_path;
        let source_contents = $source_contents;
        let tokens = $tokens;
        let position = $position;
        let expectation = $expectation;

        // Make sure we have a token to parse.
        if *position == tokens.len() {
            $errors.push(unexpected_token(
                source_path,
                source_contents,
                tokens,
                *position,
                expectation,
            ));

            return $error_value;
        }

        // Check if the token matches what we expect.
        if let token::Variant::$variant(argument) = &tokens[*position].variant {
            *position += 1;

            argument.clone()
        } else {
            $errors.push(unexpected_token(
                source_path,
                source_contents,
                tokens,
                *position,
                expectation,
            ));

            return $error_value;
        }
    }};
}

// This is the top-level parsing function.
pub fn parse(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
) -> Result<schema::Schema, Vec<Error>> {
    // Try to parse the tokens into a schema.
    let mut position = 0;
    let mut errors = vec![];
    let schema = parse_schema(
        source_path,
        source_contents,
        tokens,
        &mut position,
        &mut errors,
    );

    // Check if the parse was successful but we didn't consume all the tokens.
    if errors.is_empty() && position != tokens.len() {
        // Complain about the first unparsed token.
        errors.push(throw::<Error>(
            &format!("Unexpected {}.", tokens[position].to_string().code_str()),
            Some(source_path),
            Some(&listing(
                source_contents,
                token_source_range(tokens, position),
            )),
            None,
        ));
    }

    // If there are no errors, return the schema. Otherwise, report the errors.
    if errors.is_empty() {
        Ok(schema)
    } else {
        Err(errors)
    }
}

// Parse a schema.
#[allow(clippy::too_many_lines)]
fn parse_schema(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: &mut usize,
    errors: &mut Vec<Error>,
) -> schema::Schema {
    let mut imports = BTreeMap::new();
    let mut declarations = BTreeMap::new();

    // Parse the imports.
    while *position < tokens.len() {
        match tokens[*position].variant {
            token::Variant::Import => {
                // This is guaranteed to advance the token position due to
                // [ref:parse_import_keyword_chomp].
                if let Some((name, import)) =
                    parse_import(source_path, source_contents, tokens, position, errors)
                {
                    if imports.insert(name.clone(), import.clone()).is_some() {
                        errors.push(throw::<Error>(
                            &format!(
                                "An import named {} already exists in this file.",
                                name.code_str(),
                            ),
                            Some(source_path),
                            Some(&listing(source_contents, import.source_range)),
                            None,
                        ));
                    }
                }
            }
            _ => {
                break;
            }
        }
    }

    // Parse the declarations.
    while *position < tokens.len() {
        match tokens[*position].variant {
            token::Variant::Struct => {
                let declaration_start = *position;
                *position += 1;

                // Parse the name and fields. This is guaranteed to advance the token position due
                // to [ref:parse_fields_some_advance].
                if let Some((name, fields)) = parse_fields(
                    source_path,
                    source_contents,
                    tokens,
                    position,
                    "a name for the struct",
                    errors,
                ) {
                    let source_range = span_tokens(tokens, declaration_start, *position);

                    if declarations
                        .insert(
                            name.clone(),
                            schema::Declaration {
                                source_range,
                                variant: schema::DeclarationVariant::Struct(fields),
                            },
                        )
                        .is_some()
                    {
                        errors.push(throw::<Error>(
                            &format!(
                                "A declaration named {} already exists in this file.",
                                name.code_str(),
                            ),
                            Some(source_path),
                            Some(&listing(source_contents, source_range)),
                            None,
                        ));
                    }
                }
            }
            token::Variant::Choice => {
                let declaration_start = *position;
                *position += 1;

                // Parse the name and fields. This is guaranteed to advance the token position due
                // to [ref:parse_fields_some_advance].
                if let Some((name, fields)) = parse_fields(
                    source_path,
                    source_contents,
                    tokens,
                    position,
                    "a name for the choice",
                    errors,
                ) {
                    let source_range = span_tokens(tokens, declaration_start, *position);

                    if declarations
                        .insert(
                            name.clone(),
                            schema::Declaration {
                                source_range,
                                variant: schema::DeclarationVariant::Choice(fields),
                            },
                        )
                        .is_some()
                    {
                        errors.push(throw::<Error>(
                            &format!(
                                "A declaration named {} already exists in this file.",
                                name.code_str(),
                            ),
                            Some(source_path),
                            Some(&listing(source_contents, source_range)),
                            None,
                        ));
                    }
                }
            }
            _ => {
                break;
            }
        }
    }

    // Construct and return the schema.
    schema::Schema {
        imports,
        declarations,
    }
}

// Parse an import. If this function returns `None`, then at least one error was added to `errors`.
// If the starting token is the `import` keyword, then this function is guaranteed to advance the
// `position` [tag:parse_import_keyword_chomp].
fn parse_import(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: &mut usize,
    errors: &mut Vec<Error>,
) -> Option<(Identifier, schema::Import)> {
    let start = *position;

    // Consume the `import` keyword.
    consume_token_0!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Import,
        None,
    );

    // Parse the path.
    let path = consume_token_1!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Path,
        "the path of a file to import",
        None,
    );

    // Determine if the import name was given explicitly.
    let explicit_import_name = if *position == tokens.len() {
        false
    } else {
        matches!(tokens[*position].variant, token::Variant::As)
    };

    // Determine the import name.
    let name = if explicit_import_name {
        // Consume the `as` keyword.
        consume_token_0!(
            source_path,
            source_contents,
            tokens,
            &mut *position,
            errors,
            As,
            None,
        );

        // Parse the name.
        consume_token_1!(
            source_path,
            source_contents,
            tokens,
            &mut *position,
            errors,
            Identifier,
            "a name for the import",
            None,
        )
    } else if let Some(file_stem) = path.file_stem() {
        // The `to_string_lossy` is semantically a no-op because the path was parsed from a
        // file which is guaranteed to be valid UTF-8.
        file_stem.to_string_lossy().as_ref().into()
    } else {
        errors.push(throw::<Error>(
            "Unable to infer a name for this import.",
            Some(source_path),
            Some(&listing(
                source_contents,
                span_tokens(tokens, start, *position),
            )),
            None,
        ));

        REPLACEMENT_CHARACTER.to_string().as_str().into()
    };

    // Construct and return the import.
    Some((
        name,
        schema::Import {
            source_range: span_tokens(tokens, start, *position),
            path,
            namespace: None,
        },
    ))
}

// Parse a series of fields enclosed in curly braces and preceded by a name. If this function
// returns `None` in the first component, then at least one error was added to `errors`. Otherwise,
// the `position` is guaranteed to have advanced [tag:parse_fields_some_advance].
fn parse_fields(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: &mut usize,
    name_expectation: &str,
    errors: &mut Vec<Error>,
) -> Option<(Identifier, Vec<schema::Field>)> {
    // Parse the name.
    let declaration_name = consume_token_1!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Identifier,
        name_expectation,
        None,
    );

    // Consume the `{`.
    consume_token_0!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        LeftCurly,
        None,
    );

    // Parse the fields.
    let mut fields = vec![];
    while *position < tokens.len() {
        if let token::Variant::RightCurly = tokens[*position].variant {
            break;
        }

        if let Some(field) = parse_field(source_path, source_contents, tokens, position, errors) {
            // In this case, [ref:parse_field_some_advance] guarantees that we will not loop
            // forever.
            fields.push(field.clone());
        } else {
            // Jump past the closing curly brace, if it exists. Otherwise, jump to the end of the
            // source.
            while *position < tokens.len() {
                *position += 1;

                if let token::Variant::RightCurly = tokens[*position - 1].variant {
                    break;
                }
            }

            return Some((declaration_name, fields));
        }
    }

    // Consume the `}`.
    consume_token_0!(
        source_path,
        source_contents,
        tokens,
        position,
        errors,
        RightCurly,
        Some((declaration_name, fields)),
    );

    // Return the name and fields.
    Some((declaration_name, fields))
}

// Parse a field. If this function returns `None`, then at least one error was added to `errors`.
// Otherwise, the `position` is guaranteed to have advanced [tag:parse_field_some_advance].
#[allow(clippy::too_many_lines)]
fn parse_field(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: &mut usize,
    errors: &mut Vec<Error>,
) -> Option<schema::Field> {
    let start = *position;

    // Parse the rule.
    let rule = if *position == tokens.len() {
        schema::Rule::Required
    } else {
        match tokens[*position].variant {
            token::Variant::Optional => {
                *position += 1;

                schema::Rule::Optional
            }
            token::Variant::Unstable => {
                *position += 1;

                schema::Rule::Unstable
            }
            _ => schema::Rule::Required,
        }
    };

    // Parse the name.
    let name = consume_token_1!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Identifier,
        "a field",
        None,
    );

    // Determine if the type was given explicitly.
    let explicit_type = if *position == tokens.len() {
        false
    } else {
        matches!(tokens[*position].variant, token::Variant::Colon)
    };

    // Parse the type, if applicable.
    let r#type = if explicit_type {
        // Consume the colon.
        consume_token_0!(
            source_path,
            source_contents,
            tokens,
            &mut *position,
            errors,
            Colon,
            None,
        );

        // Make sure we have a token to parse next.
        if *position == tokens.len() {
            errors.push(unexpected_token(
                source_path,
                source_contents,
                tokens,
                *position,
                "a type",
            ));

            return None;
        }

        // Parse the type [ref:parse_type_some_advance].
        parse_type(source_path, source_contents, tokens, position, errors)?
    } else {
        schema::Type {
            source_range: span_tokens(tokens, *position, *position),
            variant: schema::TypeVariant::Unit,
        }
    };

    // Consume the equals sign.
    consume_token_0!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Equals,
        None,
    );

    // Parse the index.
    let index = consume_token_1!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Integer,
        "an index for the field",
        None,
    );

    // Return the field.
    Some(schema::Field {
        source_range: span_tokens(tokens, start, *position),
        rule,
        name,
        r#type,
        index,
    })
}

// Parse a type. If this function returns `None`, then at least one error was added to `errors`.
// Otherwise, the `position` is guaranteed to have advanced [tag:parse_type_some_advance].
#[allow(clippy::too_many_lines)]
fn parse_type(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: &mut usize,
    errors: &mut Vec<Error>,
) -> Option<schema::Type> {
    let start = *position;

    // Make sure we have a token to parse next.
    if *position == tokens.len() {
        errors.push(unexpected_token(
            source_path,
            source_contents,
            tokens,
            *position,
            "a type",
        ));

        return None;
    }

    // Parse the type.
    if let token::Variant::LeftSquare = tokens[*position].variant {
        *position += 1;

        if let Some(inner_type) = parse_type(source_path, source_contents, tokens, position, errors)
        {
            // Consume the right square bracket.
            consume_token_0!(
                source_path,
                source_contents,
                tokens,
                &mut *position,
                errors,
                RightSquare,
                None,
            );

            Some(schema::Type {
                source_range: span_tokens(tokens, start, *position),
                variant: schema::TypeVariant::Array(Box::new(inner_type)),
            })
        } else {
            // [ref:parse_type_some_advance]
            None
        }
    } else if let token::Variant::Bool = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::Bool,
        })
    } else if let token::Variant::Bytes = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::Bytes,
        })
    } else if let token::Variant::F64 = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::F64,
        })
    } else if let token::Variant::S64 = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::S64,
        })
    } else if let token::Variant::String = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::String,
        })
    } else if let token::Variant::U64 = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::U64,
        })
    } else if let token::Variant::Unit = tokens[*position].variant {
        *position += 1;

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::Unit,
        })
    } else {
        let (import_name, r#type_name) = if *position < tokens.len() - 2 {
            if let Some(token::Variant::Dot) = tokens.get(*position + 1).map(|token| &token.variant)
            {
                let import = consume_token_1!(
                    source_path,
                    source_contents,
                    tokens,
                    &mut *position,
                    errors,
                    Identifier,
                    "the name of an import",
                    None,
                );

                *position += 1;

                let r#type = consume_token_1!(
                    source_path,
                    source_contents,
                    tokens,
                    &mut *position,
                    errors,
                    Identifier,
                    "a type",
                    None,
                );

                (Some(import), r#type)
            } else {
                let r#type = consume_token_1!(
                    source_path,
                    source_contents,
                    tokens,
                    &mut *position,
                    errors,
                    Identifier,
                    "a type",
                    None,
                );

                (None, r#type)
            }
        } else {
            let r#type = consume_token_1!(
                source_path,
                source_contents,
                tokens,
                &mut *position,
                errors,
                Identifier,
                "a type",
                None,
            );

            (None, r#type)
        };

        Some(schema::Type {
            source_range: span_tokens(tokens, start, *position),
            variant: schema::TypeVariant::Custom(import_name, r#type_name),
        })
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            assert_fails, assert_same, error::SourceRange, parser::parse, schema,
            tokenizer::tokenize,
        },
        std::{collections::BTreeMap, path::Path},
    };

    #[test]
    fn parse_empty() {
        let source_path = Path::new("foo.t");
        let source = "";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]),
            Ok(schema::Schema {
                imports: BTreeMap::new(),
                declarations: BTreeMap::new(),
            }),
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn parse_example() {
        let source_path = Path::new("foo.t");
        let source = "
            import 'baz.t'
            import 'qux.t' as corge

            # This is a struct.
            struct foo {
              w: baz.baz = 0
              optional x: u64 = 1
              y: [bool] = 2
              z = 3
            }

            # This is a choice.
            choice bar {
              w: corge.qux = 0
              unstable x: [bytes] = 1
              y: f64 = 2
              z = 3
            }
        ";
        let tokens = tokenize(source_path, source).unwrap();

        let mut imports = BTreeMap::new();

        imports.insert(
            "baz".into(),
            schema::Import {
                source_range: SourceRange { start: 13, end: 27 },
                path: Path::new("baz.t").to_owned(),
                namespace: None,
            },
        );

        imports.insert(
            "corge".into(),
            schema::Import {
                source_range: SourceRange { start: 40, end: 63 },
                path: Path::new("qux.t").to_owned(),
                namespace: None,
            },
        );

        let foo_fields = vec![
            schema::Field {
                source_range: SourceRange {
                    start: 136,
                    end: 150,
                },
                name: "w".into(),
                rule: schema::Rule::Required,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 139,
                        end: 146,
                    },
                    variant: schema::TypeVariant::Custom(Some("baz".into()), "baz".into()),
                },
                index: 0,
            },
            schema::Field {
                source_range: SourceRange {
                    start: 165,
                    end: 184,
                },
                name: "x".into(),
                rule: schema::Rule::Optional,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 177,
                        end: 180,
                    },
                    variant: schema::TypeVariant::U64,
                },
                index: 1,
            },
            schema::Field {
                source_range: SourceRange {
                    start: 199,
                    end: 212,
                },
                name: "y".into(),
                rule: schema::Rule::Required,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 202,
                        end: 208,
                    },
                    variant: schema::TypeVariant::Array(Box::new(schema::Type {
                        source_range: SourceRange {
                            start: 203,
                            end: 207,
                        },
                        variant: schema::TypeVariant::Bool,
                    })),
                },
                index: 2,
            },
            schema::Field {
                source_range: SourceRange {
                    start: 227,
                    end: 232,
                },
                name: "z".into(),
                rule: schema::Rule::Required,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 229,
                        end: 229,
                    },
                    variant: schema::TypeVariant::Unit,
                },
                index: 3,
            },
        ];

        let bar_fields = vec![
            schema::Field {
                source_range: SourceRange {
                    start: 319,
                    end: 335,
                },
                name: "w".into(),
                rule: schema::Rule::Required,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 322,
                        end: 331,
                    },
                    variant: schema::TypeVariant::Custom(Some("corge".into()), "qux".into()),
                },
                index: 0,
            },
            schema::Field {
                source_range: SourceRange {
                    start: 350,
                    end: 373,
                },
                name: "x".into(),
                rule: schema::Rule::Unstable,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 362,
                        end: 369,
                    },
                    variant: schema::TypeVariant::Array(Box::new(schema::Type {
                        source_range: SourceRange {
                            start: 363,
                            end: 368,
                        },
                        variant: schema::TypeVariant::Bytes,
                    })),
                },
                index: 1,
            },
            schema::Field {
                source_range: SourceRange {
                    start: 388,
                    end: 398,
                },
                name: "y".into(),
                rule: schema::Rule::Required,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 391,
                        end: 394,
                    },
                    variant: schema::TypeVariant::F64,
                },
                index: 2,
            },
            schema::Field {
                source_range: SourceRange {
                    start: 413,
                    end: 418,
                },
                name: "z".into(),
                rule: schema::Rule::Required,
                r#type: schema::Type {
                    source_range: SourceRange {
                        start: 415,
                        end: 415,
                    },
                    variant: schema::TypeVariant::Unit,
                },
                index: 3,
            },
        ];

        let mut declarations = BTreeMap::new();

        declarations.insert(
            "foo".into(),
            schema::Declaration {
                source_range: SourceRange {
                    start: 109,
                    end: 246,
                },
                variant: schema::DeclarationVariant::Struct(foo_fields),
            },
        );

        declarations.insert(
            "bar".into(),
            schema::Declaration {
                source_range: SourceRange {
                    start: 292,
                    end: 432,
                },
                variant: schema::DeclarationVariant::Choice(bar_fields),
            },
        );

        assert_same!(
            parse(source_path, source, &tokens[..]),
            Ok(schema::Schema {
                imports,
                declarations,
            }),
        );
    }

    #[test]
    fn parse_duplicate_import() {
        let source_path = Path::new("foo.t");
        let source = "
            import 'foo.t' as qux
            import 'bar.t' as qux
        ";
        let tokens = tokenize(source_path, source).unwrap();

        assert_fails!(
            parse(source_path, source, &tokens[..]),
            "An import named `qux` already exists in this file.",
        );
    }

    #[test]
    fn parse_duplicate_declaration() {
        let source_path = Path::new("foo.t");
        let source = "
            struct foo {
            }

            choice foo {
            }
        ";
        let tokens = tokenize(source_path, source).unwrap();

        assert_fails!(
            parse(source_path, source, &tokens[..]),
            "A declaration named `foo` already exists in this file.",
        );
    }
}
