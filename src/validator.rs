use crate::{
    error::{listing, throw, Error},
    format::CodeStr,
    schema,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

// This function validates a schema and its transitive dependencies. The based import paths are
// assumed to be in the `HashMap` [ref:valid_based_paths].
pub fn validate(schemas: &HashMap<PathBuf, (schema::Schema, String)>) -> Result<(), Vec<Error>> {
    // We'll add any errors to this.
    let mut errors: Vec<Error> = vec![];

    // Validate uniqueness of various things within each file.
    for (path, (schema, source_contents)) in schemas {
        // Check that imports are unique within each file.
        let mut import_names = HashSet::new();

        for import in &schema.imports {
            if !import_names.insert(import.name.to_owned()) {
                errors.push(throw::<Error>(
                    &format!(
                        "An import named {} already exists in this file.",
                        import.name.code_str(),
                    ),
                    Some(path),
                    Some(&listing(source_contents, import.source_range)),
                    None,
                ));
            }
        }

        // Validate uniqueness of declarations and of field names and indices within each
        // declaration.
        let mut declaration_names = HashSet::new();

        for declaration in &schema.declarations {
            match &declaration.variant {
                schema::DeclarationVariant::Struct(name, fields)
                | schema::DeclarationVariant::Choice(name, fields) => {
                    if !declaration_names.insert(name.to_owned()) {
                        errors.push(throw::<Error>(
                            &format!(
                                "A declaration named {} already exists in this file.",
                                name.code_str(),
                            ),
                            Some(path),
                            Some(&listing(source_contents, declaration.source_range)),
                            None,
                        ));
                    }

                    let mut field_names = HashSet::new();
                    let mut field_indices = HashSet::new();

                    for field in fields {
                        if !field_names.insert(field.name.to_owned()) {
                            errors.push(throw::<Error>(
                                &format!(
                                    "A field named {} already exists in this declaration.",
                                    field.name.code_str(),
                                ),
                                Some(path),
                                Some(&listing(source_contents, field.source_range)),
                                None,
                            ));
                        }

                        if !field_indices.insert(field.index.to_owned()) {
                            errors.push(throw::<Error>(
                                &format!(
                                    "A field with index {} already exists in this declaration.",
                                    field.index.to_string().code_str(),
                                ),
                                Some(path),
                                Some(&listing(source_contents, field.source_range)),
                                None,
                            ));
                        }
                    }
                }
            }
        }
    }

    // Return a success or report any errors.
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_fails, assert_same, parser::parse, tokenizer::tokenize, validator::validate,
    };
    use std::{collections::HashMap, path::Path};

    #[test]
    fn validate_empty() {
        let path = Path::new("foo.t").to_owned();
        let contents = "".to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = HashMap::new();
        schemas.insert(path, (schema, contents));

        assert_same!(validate(&schemas), Ok(()));
    }

    #[test]
    fn validate_example() {
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t' as bar

            struct Foo {
              corge: bar.Bar = 0
              grault: bar.Bar = 1
            }
        "
        .to_owned();
        let foo_tokens = tokenize(&foo_path, &foo_contents).unwrap();
        let mut foo_schema = parse(&foo_path, &foo_contents, &foo_tokens).unwrap();
        foo_schema.imports[0].based_path = foo_schema.imports[0].original_path.clone();

        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            import 'foo.t' as foo

            choice Qux {
              corge: foo.Foo = 0
              grault: foo.Foo = 1
            }
        "
        .to_owned();
        let bar_tokens = tokenize(&bar_path, &bar_contents).unwrap();
        let mut bar_schema = parse(&bar_path, &bar_contents, &bar_tokens).unwrap();
        bar_schema.imports[0].based_path = bar_schema.imports[0].original_path.clone();

        let mut schemas = HashMap::new();
        schemas.insert(foo_path, (foo_schema, foo_contents));
        schemas.insert(bar_path, (bar_schema, bar_contents));

        assert_same!(validate(&schemas), Ok(()));
    }

    #[allow(clippy::similar_names)]
    #[test]
    fn validate_duplicate_imports() {
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t' as bar
            import 'baz.t' as bar

            struct Foo {
              corge: bar.Bar = 0
              grault: bar.Bar = 1
            }
        "
        .to_owned();
        let foo_tokens = tokenize(&foo_path, &foo_contents).unwrap();
        let mut foo_schema = parse(&foo_path, &foo_contents, &foo_tokens).unwrap();
        foo_schema.imports[0].based_path = foo_schema.imports[0].original_path.clone();

        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            import 'foo.t' as foo

            choice Qux {
              corge: foo.Foo = 0
              grault: foo.Foo = 1
            }
        "
        .to_owned();
        let bar_tokens = tokenize(&bar_path, &bar_contents).unwrap();
        let mut bar_schema = parse(&bar_path, &bar_contents, &bar_tokens).unwrap();
        bar_schema.imports[0].based_path = bar_schema.imports[0].original_path.clone();

        let baz_path = Path::new("baz.t").to_owned();
        let baz_contents = "
            import 'foo.t' as foo

            choice Qux {
              corge: foo.Foo = 0
              grault: foo.Foo = 1
            }
        "
        .to_owned();
        let baz_tokens = tokenize(&baz_path, &baz_contents).unwrap();
        let mut baz_schema = parse(&baz_path, &baz_contents, &baz_tokens).unwrap();
        baz_schema.imports[0].based_path = baz_schema.imports[0].original_path.clone();

        let mut schemas = HashMap::new();
        schemas.insert(foo_path, (foo_schema, foo_contents));
        schemas.insert(bar_path, (bar_schema, bar_contents));
        schemas.insert(baz_path, (baz_schema, baz_contents));

        assert_fails!(
            validate(&schemas),
            "An import named `bar` already exists in this file.",
        );
    }

    #[test]
    fn validate_duplicate_declarations() {
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
            }

            choice Foo {
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = HashMap::new();
        schemas.insert(path, (schema, contents));

        assert_fails!(
            validate(&schemas),
            "A declaration named `Foo` already exists in this file.",
        );
    }

    #[test]
    fn validate_duplicate_struct_field_names() {
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
            }

            struct Bar {
              x: Foo = 0
              x: Foo = 1
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = HashMap::new();
        schemas.insert(path, (schema, contents));

        assert_fails!(
            validate(&schemas),
            "A field named `x` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_duplicate_struct_field_indices() {
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
            }

            struct Bar {
              x: Foo = 0
              y: Foo = 0
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = HashMap::new();
        schemas.insert(path, (schema, contents));

        assert_fails!(
            validate(&schemas),
            "A field with index `0` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_duplicate_choice_field_names() {
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
            }

            choice Bar {
              x: Foo = 0
              x: Foo = 1
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = HashMap::new();
        schemas.insert(path, (schema, contents));

        assert_fails!(
            validate(&schemas),
            "A field named `x` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_duplicate_choice_field_indices() {
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
            }

            choice Bar {
              x: Foo = 0
              y: Foo = 0
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = HashMap::new();
        schemas.insert(path, (schema, contents));

        assert_fails!(
            validate(&schemas),
            "A field with index `0` already exists in this declaration.",
        );
    }
}
