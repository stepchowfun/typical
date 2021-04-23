use crate::{
    error::{listing, throw, Error, SourceRange},
    format::CodeStr,
    schema, token,
    token::Identifier,
};
use std::path::Path;

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
        $error_value:expr $(,)? // This comma is needed to satisfy the trailing commas check: ,
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
        $error_value:expr $(,)? // This comma is needed to satisfy the trailing commas check: ,
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
fn parse_schema(
    source_path: &Path,
    source_contents: &str,
    tokens: &[token::Token],
    position: &mut usize,
    errors: &mut Vec<Error>,
) -> schema::Schema {
    let mut imports = vec![];
    let mut declarations = vec![];

    // Parse the imports.
    while *position < tokens.len() {
        match tokens[*position].variant {
            token::Variant::Import => {
                // Parse the path and name. This is guaranteed to advance the token position due to
                // [ref:parse_import_keyword_chomp].

                if let Some(import) =
                    parse_import(source_path, source_contents, tokens, position, errors)
                {
                    imports.push(import);
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
                    declarations.push(schema::Declaration {
                        source_range: span_tokens(tokens, declaration_start, *position),
                        variant: schema::DeclarationVariant::Struct(name, fields),
                    });
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
                    declarations.push(schema::Declaration {
                        source_range: span_tokens(tokens, declaration_start, *position),
                        variant: schema::DeclarationVariant::Choice(name, fields),
                    });
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
) -> Option<schema::Import> {
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
    let name = consume_token_1!(
        source_path,
        source_contents,
        tokens,
        &mut *position,
        errors,
        Identifier,
        "a name for the import",
        None,
    );

    // Construct and return the import.
    Some(schema::Import {
        source_range: span_tokens(tokens, start, *position),
        path,
        namespace: None,
        name,
    })
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
    let name = consume_token_1!(
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
            fields.push(field);
        } else {
            // Jump past the closing curly brace, if it exists. Otherwise, jump to the end of the
            // source.
            while *position < tokens.len() {
                *position += 1;

                if let token::Variant::RightCurly = tokens[*position - 1].variant {
                    break;
                }
            }

            return Some((name, fields));
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
        Some((name, fields)),
    );

    // Return the name and fields.
    Some((name, fields))
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

    // Determine if the field is transitional.
    let transitional = if *position == tokens.len() {
        false
    } else if let token::Variant::Transitional = tokens[*position].variant {
        *position += 1;

        true
    } else {
        false
    };

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
    let type_start = *position;
    let r#type = if let token::Variant::Bool = tokens[*position].variant {
        *position += 1;

        schema::Type {
            source_range: span_tokens(tokens, type_start, *position),
            variant: schema::TypeVariant::Bool,
        }
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
                    "a type for the field",
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
                    "a type for the field",
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
                "a type for the field",
                None,
            );

            (None, r#type)
        };

        schema::Type {
            source_range: span_tokens(tokens, type_start, *position),
            variant: schema::TypeVariant::Custom(import_name, r#type_name),
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
        name,
        transitional,
        r#type,
        index,
    })
}

#[cfg(test)]
mod tests {
    use crate::{assert_same, error::SourceRange, parser::parse, schema, tokenizer::tokenize};
    use std::path::Path;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn parse_example() {
        let source_path = Path::new("foo.t");
        let source = "
            import 'bar.t' as bar

            # This is a struct.
            struct Foo {
              x: bar.Bar = 0
              y: transitional int = 1
              z: Bool = 2
            }

            # This is a choice.
            choice Qux {
              x: bar.Bar = 0
              y: transitional int = 1
              z: Bool = 2
            }
        ";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![schema::Import {
                    source_range: SourceRange { start: 13, end: 34 },
                    path: Path::new("bar.t").to_owned(),
                    namespace: None,
                    name: "bar".into(),
                }],
                declarations: vec![
                    schema::Declaration {
                        source_range: SourceRange {
                            start: 80,
                            end: 199,
                        },
                        variant: schema::DeclarationVariant::Struct(
                            "Foo".into(),
                            vec![
                                schema::Field {
                                    source_range: SourceRange {
                                        start: 107,
                                        end: 121,
                                    },
                                    name: "x".into(),
                                    transitional: false,
                                    r#type: schema::Type {
                                        source_range: SourceRange {
                                            start: 110,
                                            end: 117,
                                        },
                                        variant: schema::TypeVariant::Custom(
                                            Some("bar".into()),
                                            "Bar".into(),
                                        ),
                                    },
                                    index: 0,
                                },
                                schema::Field {
                                    source_range: SourceRange {
                                        start: 136,
                                        end: 159,
                                    },
                                    name: "y".into(),
                                    transitional: true,
                                    r#type: schema::Type {
                                        source_range: SourceRange {
                                            start: 152,
                                            end: 155,
                                        },
                                        variant: schema::TypeVariant::Custom(None, "int".into()),
                                    },
                                    index: 1,
                                },
                                schema::Field {
                                    source_range: SourceRange {
                                        start: 174,
                                        end: 185,
                                    },
                                    name: "z".into(),
                                    transitional: false,
                                    r#type: schema::Type {
                                        source_range: SourceRange {
                                            start: 177,
                                            end: 181,
                                        },
                                        variant: schema::TypeVariant::Bool,
                                    },
                                    index: 2,
                                },
                            ],
                        ),
                    },
                    schema::Declaration {
                        source_range: SourceRange {
                            start: 245,
                            end: 364,
                        },
                        variant: schema::DeclarationVariant::Choice(
                            "Qux".into(),
                            vec![
                                schema::Field {
                                    source_range: SourceRange {
                                        start: 272,
                                        end: 286,
                                    },
                                    name: "x".into(),
                                    transitional: false,
                                    r#type: schema::Type {
                                        source_range: SourceRange {
                                            start: 275,
                                            end: 282,
                                        },
                                        variant: schema::TypeVariant::Custom(
                                            Some("bar".into()),
                                            "Bar".into(),
                                        ),
                                    },
                                    index: 0,
                                },
                                schema::Field {
                                    source_range: SourceRange {
                                        start: 301,
                                        end: 324,
                                    },
                                    name: "y".into(),
                                    transitional: true,
                                    r#type: schema::Type {
                                        source_range: SourceRange {
                                            start: 317,
                                            end: 320,
                                        },
                                        variant: schema::TypeVariant::Custom(None, "int".into()),
                                    },
                                    index: 1,
                                },
                                schema::Field {
                                    source_range: SourceRange {
                                        start: 339,
                                        end: 350,
                                    },
                                    name: "z".into(),
                                    transitional: false,
                                    r#type: schema::Type {
                                        source_range: SourceRange {
                                            start: 342,
                                            end: 346,
                                        },
                                        variant: schema::TypeVariant::Bool,
                                    },
                                    index: 2,
                                },
                            ],
                        ),
                    },
                ],
            },
        );
    }

    #[test]
    fn parse_empty() {
        let source_path = Path::new("foo.t");
        let source = "";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![],
            },
        );
    }

    #[test]
    fn parse_import() {
        let source_path = Path::new("foo.t");
        let source = "import 'bar.t' as bar";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![schema::Import {
                    source_range: SourceRange { start: 0, end: 21 },
                    path: Path::new("bar.t").to_owned(),
                    namespace: None,
                    name: "bar".into(),
                }],
                declarations: vec![],
            },
        );
    }

    #[test]
    fn parse_struct_empty() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 14 },
                    variant: schema::DeclarationVariant::Struct("Foo".into(), vec![]),
                }],
            },
        );
    }

    #[test]
    fn parse_struct_single() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { x: Bool = 0 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 26 },
                    variant: schema::DeclarationVariant::Struct(
                        "Foo".into(),
                        vec![schema::Field {
                            source_range: SourceRange { start: 13, end: 24 },
                            name: "x".into(),
                            transitional: false,
                            r#type: schema::Type {
                                source_range: SourceRange { start: 16, end: 20 },
                                variant: schema::TypeVariant::Bool,
                            },
                            index: 0,
                        }],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_struct_multi() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { x: Bool = 0 y: Bool = 1 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 38 },
                    variant: schema::DeclarationVariant::Struct(
                        "Foo".into(),
                        vec![
                            schema::Field {
                                source_range: SourceRange { start: 13, end: 24 },
                                name: "x".into(),
                                transitional: false,
                                r#type: schema::Type {
                                    source_range: SourceRange { start: 16, end: 20 },
                                    variant: schema::TypeVariant::Bool,
                                },
                                index: 0,
                            },
                            schema::Field {
                                source_range: SourceRange { start: 25, end: 36 },
                                name: "y".into(),
                                transitional: false,
                                r#type: schema::Type {
                                    source_range: SourceRange { start: 28, end: 32 },
                                    variant: schema::TypeVariant::Bool,
                                },
                                index: 1,
                            },
                        ],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_choice_empty() {
        let source_path = Path::new("foo.t");
        let source = "choice Foo { }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 14 },
                    variant: schema::DeclarationVariant::Choice("Foo".into(), vec![]),
                }],
            },
        );
    }

    #[test]
    fn parse_choice_single() {
        let source_path = Path::new("foo.t");
        let source = "choice Foo { x: Bool = 0 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 26 },
                    variant: schema::DeclarationVariant::Choice(
                        "Foo".into(),
                        vec![schema::Field {
                            source_range: SourceRange { start: 13, end: 24 },
                            name: "x".into(),
                            transitional: false,
                            r#type: schema::Type {
                                source_range: SourceRange { start: 16, end: 20 },
                                variant: schema::TypeVariant::Bool,
                            },
                            index: 0,
                        }],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_choice_multi() {
        let source_path = Path::new("foo.t");
        let source = "choice Foo { x: Bool = 0 y: Bool = 1 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 38 },
                    variant: schema::DeclarationVariant::Choice(
                        "Foo".into(),
                        vec![
                            schema::Field {
                                source_range: SourceRange { start: 13, end: 24 },
                                name: "x".into(),
                                transitional: false,
                                r#type: schema::Type {
                                    source_range: SourceRange { start: 16, end: 20 },
                                    variant: schema::TypeVariant::Bool,
                                },
                                index: 0,
                            },
                            schema::Field {
                                source_range: SourceRange { start: 25, end: 36 },
                                name: "y".into(),
                                transitional: false,
                                r#type: schema::Type {
                                    source_range: SourceRange { start: 28, end: 32 },
                                    variant: schema::TypeVariant::Bool,
                                },
                                index: 1,
                            },
                        ],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_field_transitional() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { x: transitional Foo = 0 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 38 },
                    variant: schema::DeclarationVariant::Struct(
                        "Foo".into(),
                        vec![schema::Field {
                            source_range: SourceRange { start: 13, end: 36 },
                            name: "x".into(),
                            transitional: true,
                            r#type: schema::Type {
                                source_range: SourceRange { start: 29, end: 32 },
                                variant: schema::TypeVariant::Custom(None, "Foo".into()),
                            },
                            index: 0,
                        }],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_field_imported_type() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { x: bar.Bar = 0 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                imports: vec![],
                declarations: vec![schema::Declaration {
                    source_range: SourceRange { start: 0, end: 29 },
                    variant: schema::DeclarationVariant::Struct(
                        "Foo".into(),
                        vec![schema::Field {
                            source_range: SourceRange { start: 13, end: 27 },
                            name: "x".into(),
                            transitional: false,
                            r#type: schema::Type {
                                source_range: SourceRange { start: 16, end: 23 },
                                variant: schema::TypeVariant::Custom(
                                    Some("bar".into()),
                                    "Bar".into(),
                                ),
                            },
                            index: 0,
                        }],
                    ),
                }],
            },
        );
    }
}
