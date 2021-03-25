use crate::{
    error::{throw, Error},
    format::CodeStr,
    schema, token,
};
use std::{
    cmp::{max, min},
    path::Path,
    rc::Rc,
};

// An `ErrorFactory` is a closure which takes a source path and contents and produces an `Error`.
// It's cheaper to generate a closure that produces the `Error` than to generate the actual
// `Error`, which may contain a long string message.
type ErrorFactory<'a> = Rc<dyn Fn(&'a Path, &'a str) -> Error + 'a>;

// This function constructs a generic error factory that just complains about a particular token or
// the end of the source file.
fn error_factory<'a>(
    tokens: &'a [token::Token<'a>],
    position: usize,
    expectation: &str,
) -> ErrorFactory<'a> {
    let source_range = token_source_range(tokens, position);
    let expectation = expectation.to_owned();

    Rc::new(move |source_path, source_contents| {
        if tokens.is_empty() {
            throw(
                &format!("Expected {}, but the file is empty.", expectation),
                Some(source_path),
                Some((source_contents, source_range.0)),
            )
        } else if position == tokens.len() {
            throw(
                &format!("Expected {} at the end of the file.", expectation),
                Some(source_path),
                Some((source_contents, source_range.0)),
            )
        } else {
            throw(
                &format!(
                    "Expected {}, but encountered {}.",
                    expectation,
                    tokens[position].to_string().code_str(),
                ),
                Some(source_path),
                Some((source_contents, source_range.0)),
            )
        }
    })
}

// For extra type safety, we use the "newtype" pattern here to introduce a new type for source
// ranges. The goal is to prevent source ranges from accidentally including token indices.
#[derive(Clone, Copy, Debug)]
struct SourceRange((usize, usize)); // Inclusive on the left and exclusive on the right

// This function constructs a `SourceRange` that spans two given `SourceRange`s.
fn span(x: SourceRange, y: SourceRange) -> SourceRange {
    SourceRange {
        0: (min((x.0).0, (y.0).0), max((x.0).1, (y.0).1)),
    }
}

// This function computes the source range for a token, or the empty range at the end of the source
// file in the case where the given position is at the end of the token stream.
fn token_source_range<'a>(tokens: &'a [token::Token<'a>], position: usize) -> SourceRange {
    if position == tokens.len() {
        SourceRange {
            0: tokens
                .last()
                .map_or((0, 0), |token| (token.source_range.1, token.source_range.1)),
        }
    } else {
        SourceRange {
            0: tokens[position].source_range,
        }
    }
}

// This macro consumes a single token (with no arguments). On a successful parse, this macro
// evaluates to the next token position (i.e., `$next + 1`). If the parse fails, this macro returns
// the given `$error_value`.
macro_rules! consume_token_0 {
    (
        $tokens:expr,
        $next:expr,
        $errors:ident,
        $variant:ident,
        $error_value:expr $(,)? // This comma is needed to satisfy the trailing commas check: ,
    ) => {{
        // Macros are call-by-name, but we want call-by-value (or at least call-by-need) to avoid
        // accidentally evaluating arguments multiple times. Here we force eager evaluation.
        let tokens = $tokens;
        let next = $next;

        // Make sure we have a token to parse.
        if next == tokens.len() {
            $errors.push(error_factory(
                tokens,
                next,
                &format!("{}", token::Variant::$variant.to_string().code_str()),
            ));

            return $error_value;
        }

        // Check if the token matches what we expect.
        if let token::Variant::$variant = tokens[next].variant {
            next + 1
        } else {
            $errors.push(error_factory(
                tokens,
                next,
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
        $tokens:expr,
        $next:expr,
        $errors:ident,
        $variant:ident,
        $expectation:expr,
        $error_value:expr $(,)? // This comma is needed to satisfy the trailing commas check: ,
    ) => {{
        // Macros are call-by-name, but we want call-by-value (or at least call-by-need) to avoid
        // accidentally evaluating arguments multiple times. Here we force eager evaluation.
        let tokens = $tokens;
        let next = $next;
        let expectation = $expectation;

        // Make sure we have a token to parse.
        if next == tokens.len() {
            $errors.push(error_factory(tokens, next, expectation));

            return $error_value;
        }

        // Check if the token matches what we expect.
        if let token::Variant::$variant(argument) = tokens[next].variant {
            (argument.clone(), next + 1)
        } else {
            $errors.push(error_factory(tokens, next, expectation));

            return $error_value;
        }
    }};
}

// This is the top-level parsing function.
pub fn parse<'a>(
    source_path: &'a Path,
    source_contents: &'a str,
    tokens: &'a [token::Token<'a>],
) -> Result<schema::Schema<'a>, Vec<Error>> {
    // Try to parse the tokens into a schema.
    let mut error_factories = vec![];
    let (schema, next) = parse_schema(source_path, tokens, 0, &mut error_factories);

    // Check if the parse was successful but we didn't consume all the tokens.
    if error_factories.is_empty() && next != tokens.len() {
        // Complain about the first unparsed token.
        error_factories.push(error_factory(tokens, next, "the end of the file"));
    }

    // If there are no errors, return the schema. Otherwise, report the errors.
    if error_factories.is_empty() {
        Ok(schema)
    } else {
        Err(error_factories
            .into_iter()
            .map(|error_factory| error_factory(source_path, source_contents))
            .collect())
    }
}

// Parse a schema.
fn parse_schema<'a>(
    source_path: &'a Path,
    tokens: &'a [token::Token<'a>],
    start: usize,
    errors: &mut Vec<ErrorFactory<'a>>,
) -> (schema::Schema<'a>, usize) {
    let mut declarations = vec![];
    let mut next = start;

    while next < tokens.len() {
        match tokens[next].variant {
            token::Variant::Struct => {
                let declaration_start = next;
                next += 1;

                // Parse the name and fields.
                let (name, fields, new_next) =
                    parse_fields(tokens, next, "a name for the struct", errors);
                next = new_next;

                // Create the declaration.
                declarations.push(schema::Declaration {
                    source_range: span(
                        token_source_range(tokens, declaration_start),
                        token_source_range(tokens, next - 1),
                    )
                    .0,
                    variant: schema::DeclarationVariant::Struct(name, fields),
                });
            }
            token::Variant::Choice => {
                let declaration_start = next;
                next += 1;

                // Parse the name and fields.
                let (name, fields, new_next) =
                    parse_fields(tokens, next, "a name for the choice", errors);
                next = new_next;

                // Create the declaration.
                declarations.push(schema::Declaration {
                    source_range: span(
                        token_source_range(tokens, declaration_start),
                        token_source_range(tokens, next - 1),
                    )
                    .0,
                    variant: schema::DeclarationVariant::Choice(name, fields),
                });
            }
            _ => {
                break;
            }
        }
    }

    (
        schema::Schema {
            path: source_path,
            declarations,
        },
        next,
    )
}

// Parse a series of fields enclosed in curly braces and preceded by a name.
fn parse_fields<'a>(
    tokens: &'a [token::Token<'a>],
    start: usize,
    name_expectation: &str,
    errors: &mut Vec<ErrorFactory<'a>>,
) -> (&'a str, Vec<schema::Field<'a>>, usize) {
    // Parse the name.
    let (name, mut next) = consume_token_1!(
        tokens,
        start,
        errors,
        Identifier,
        name_expectation,
        ("?", vec![], start),
    );

    // Consume the `{`.
    next = consume_token_0!(tokens, next, errors, LeftCurly, (name, vec![], next));

    // Parse the fields.
    let mut fields = vec![];
    while next < tokens.len() {
        if let token::Variant::RightCurly = tokens[next].variant {
            break;
        }

        let (result, new_next) = parse_field(tokens, next, errors);
        next = new_next;

        if let Some(field) = result {
            fields.push(field);
        } else {
            while next < tokens.len() {
                next += 1;

                if let token::Variant::RightCurly = tokens[next - 1].variant {
                    break;
                }
            }

            return (name, fields, next);
        }
    }

    // Consume the `}`.
    next = consume_token_0!(tokens, next, errors, RightCurly, (name, fields, next));

    // Return the name, fields, and next token position.
    (name, fields, next)
}

// Parse a field.
fn parse_field<'a>(
    tokens: &'a [token::Token<'a>],
    start: usize,
    errors: &mut Vec<ErrorFactory<'a>>,
) -> (Option<schema::Field<'a>>, usize) {
    // Parse the name.
    let (name, next) = consume_token_1!(
        tokens,
        start,
        errors,
        Identifier,
        "a name for the field",
        (None, start),
    );

    // Consume the colon.
    let next = consume_token_0!(tokens, next, errors, Colon, (None, start));

    // Determine if the field is restricted.
    let (restricted, next) = if next == tokens.len() {
        (false, next)
    } else if let token::Variant::Restricted = tokens[next].variant {
        (true, next + 1)
    } else {
        (false, next)
    };

    // Parse the type.
    let type_pos = next;
    let (r#type, next) = consume_token_1!(
        tokens,
        next,
        errors,
        Identifier,
        "a type for the field",
        (None, start),
    );

    // Consume the equals sign.
    let next = consume_token_0!(tokens, next, errors, Equals, (None, start));

    // Parse the index.
    let (index, next) = consume_token_1!(
        tokens,
        next,
        errors,
        IntegerLiteral,
        "an index for the field",
        (None, start),
    );

    // Return the field.
    (
        Some(schema::Field {
            source_range: span(
                token_source_range(tokens, start),
                token_source_range(tokens, next - 1),
            )
            .0,
            name,
            restricted,
            r#type: schema::Type {
                source_range: token_source_range(tokens, type_pos).0,
                name: r#type,
            },
            index,
        }),
        next,
    )
}

#[cfg(test)]
mod tests {
    use crate::{assert_same, parser::parse, schema, tokenizer::tokenize};
    use std::path::Path;

    #[test]
    fn parse_empty() {
        let source_path = Path::new("foo.t");
        let source = "";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                path: source_path,
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
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 14),
                    variant: schema::DeclarationVariant::Struct("Foo", vec![]),
                },],
            },
        );
    }

    #[test]
    fn parse_struct_single() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { foo: Foo = 42 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 28),
                    variant: schema::DeclarationVariant::Struct(
                        "Foo",
                        vec![schema::Field {
                            source_range: (13, 26),
                            name: "foo",
                            restricted: false,
                            r#type: schema::Type {
                                source_range: (18, 21),
                                name: "Foo",
                            },
                            index: 42,
                        }],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_struct_multi() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { foo: Foo = 42 bar: Bar = 43 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 42),
                    variant: schema::DeclarationVariant::Struct(
                        "Foo",
                        vec![
                            schema::Field {
                                source_range: (13, 26),
                                name: "foo",
                                restricted: false,
                                r#type: schema::Type {
                                    source_range: (18, 21),
                                    name: "Foo",
                                },
                                index: 42,
                            },
                            schema::Field {
                                source_range: (27, 40),
                                name: "bar",
                                restricted: false,
                                r#type: schema::Type {
                                    source_range: (32, 35),
                                    name: "Bar",
                                },
                                index: 43,
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
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 14),
                    variant: schema::DeclarationVariant::Choice("Foo", vec![]),
                },],
            },
        );
    }

    #[test]
    fn parse_choice_single() {
        let source_path = Path::new("foo.t");
        let source = "choice Foo { foo: Foo = 42 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 28),
                    variant: schema::DeclarationVariant::Choice(
                        "Foo",
                        vec![schema::Field {
                            source_range: (13, 26),
                            name: "foo",
                            restricted: false,
                            r#type: schema::Type {
                                source_range: (18, 21),
                                name: "Foo",
                            },
                            index: 42,
                        }],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_choice_multi() {
        let source_path = Path::new("foo.t");
        let source = "choice Foo { foo: Foo = 42 bar: Bar = 43 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 42),
                    variant: schema::DeclarationVariant::Choice(
                        "Foo",
                        vec![
                            schema::Field {
                                source_range: (13, 26),
                                name: "foo",
                                restricted: false,
                                r#type: schema::Type {
                                    source_range: (18, 21),
                                    name: "Foo",
                                },
                                index: 42,
                            },
                            schema::Field {
                                source_range: (27, 40),
                                name: "bar",
                                restricted: false,
                                r#type: schema::Type {
                                    source_range: (32, 35),
                                    name: "Bar",
                                },
                                index: 43,
                            },
                        ],
                    ),
                }],
            },
        );
    }

    #[test]
    fn parse_field_restricted() {
        let source_path = Path::new("foo.t");
        let source = "struct Foo { foo: restricted Foo = 42 }";
        let tokens = tokenize(source_path, source).unwrap();

        assert_same!(
            parse(source_path, source, &tokens[..]).unwrap(),
            schema::Schema {
                path: source_path,
                declarations: vec![schema::Declaration {
                    source_range: (0, 39),
                    variant: schema::DeclarationVariant::Struct(
                        "Foo",
                        vec![schema::Field {
                            source_range: (13, 37),
                            name: "foo",
                            restricted: true,
                            r#type: schema::Type {
                                source_range: (29, 32),
                                name: "Foo",
                            },
                            index: 42,
                        }],
                    ),
                }],
            },
        );
    }
}
