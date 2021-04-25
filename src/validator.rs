use crate::{
    error::{listing, throw, Error},
    format::CodeStr,
    schema,
};
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

// This function validates a schema and its transitive dependencies.
#[allow(clippy::too_many_lines)]
pub fn validate(
    schemas: &BTreeMap<schema::Namespace, (schema::Schema, PathBuf, String)>,
) -> Result<(), Vec<Error>> {
    // We'll add any errors to this.
    let mut errors: Vec<Error> = vec![];

    // For the purpose of validating types, construct a complete set of (path, declaration) pairs.
    let mut all_types = HashSet::new();

    for (namespace, (schema, _, _)) in schemas {
        for (name, declaration) in &schema.declarations {
            match &declaration.variant {
                schema::DeclarationVariant::Struct(_) | schema::DeclarationVariant::Choice(_) => {
                    all_types.insert((namespace.clone(), name.clone()));
                }
            }
        }
    }

    // Validate each file.
    for (namespace, (schema, source_path, source_contents)) in schemas {
        // Validate the declarations in the file.
        for declaration in schema.declarations.values() {
            match &declaration.variant {
                schema::DeclarationVariant::Struct(fields)
                | schema::DeclarationVariant::Choice(fields) => {
                    // Validate the fields in the declaration.
                    let mut field_indices = HashSet::new();

                    for field in fields.values() {
                        // Check that the index of the field is unique within the declaration.
                        if !field_indices.insert(field.index) {
                            errors.push(throw::<Error>(
                                &format!(
                                    "A field with index {} already exists in this declaration.",
                                    field.index.to_string().code_str(),
                                ),
                                Some(source_path),
                                Some(&listing(source_contents, field.source_range)),
                                None,
                            ));
                        }

                        // Validate the type.
                        match &field.r#type.variant {
                            schema::TypeVariant::Bool => {}
                            schema::TypeVariant::Custom(import, name) => {
                                // Determine which file the type is from.
                                let type_path = if let Some(import) = import {
                                    if let Some(import) = schema.imports.get(import) {
                                        // The `unwrap` is safe due to [ref:namespace_populated].
                                        import.namespace.clone().unwrap()
                                    } else {
                                        errors.push(throw::<Error>(
                                            &format!(
                                                "There is no import named {} in this file.",
                                                import.code_str(),
                                            ),
                                            Some(source_path),
                                            Some(&listing(
                                                source_contents,
                                                field.r#type.source_range,
                                            )),
                                            None,
                                        ));

                                        continue;
                                    }
                                } else {
                                    namespace.clone()
                                };

                                // Check that the type exists in that file.
                                if !all_types.contains(&(type_path.clone(), name.clone())) {
                                    errors.push(throw::<Error>(
                                        &if let Some(import) = import {
                                            format!(
                                                "There is no type named {} in import {}.",
                                                name.code_str(),
                                                import.code_str(),
                                            )
                                        } else {
                                            format!(
                                                "There is no type named {} in this file.",
                                                name.code_str(),
                                            )
                                        },
                                        Some(source_path),
                                        Some(&listing(source_contents, field.r#type.source_range)),
                                        None,
                                    ));
                                }
                            }
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
        assert_fails, assert_same, parser::parse, schema::Namespace, tokenizer::tokenize,
        validator::validate,
    };
    use std::{collections::BTreeMap, path::Path};

    #[test]
    fn validate_empty() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "".to_owned();

        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_same!(validate(&schemas), Ok(()));
    }

    #[test]
    fn validate_example() {
        let foo_namespace = Namespace {
            components: vec!["foo".into()],
        };
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t' as bar

            struct Foo {
              x: bar.Bar = 0
              y: bar.Bar = 1
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            import 'foo.t' as foo

            choice Bar {
              x: foo.Foo = 0
              y: foo.Foo = 1
            }
        "
        .to_owned();

        let foo_tokens = tokenize(&foo_path, &foo_contents).unwrap();
        let mut foo_schema = parse(&foo_path, &foo_contents, &foo_tokens).unwrap();
        foo_schema.imports.get_mut(&"bar".into()).unwrap().namespace = Some(bar_namespace.clone());

        let bar_tokens = tokenize(&bar_path, &bar_contents).unwrap();
        let mut bar_schema = parse(&bar_path, &bar_contents, &bar_tokens).unwrap();
        bar_schema.imports.get_mut(&"foo".into()).unwrap().namespace = Some(foo_namespace.clone());

        let mut schemas = BTreeMap::new();
        schemas.insert(foo_namespace, (foo_schema, foo_path, foo_contents));
        schemas.insert(bar_namespace, (bar_schema, bar_path, bar_contents));

        assert_same!(validate(&schemas), Ok(()));
    }

    #[test]
    fn validate_duplicate_struct_field_indices() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
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

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "A field with index `0` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_duplicate_choice_field_indices() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
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

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "A field with index `0` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_non_existent_field_import() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
              x: bar.Bar = 0
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "There is no import named `bar` in this file.",
        );
    }

    #[test]
    fn validate_non_existent_field_type_same_file() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct Foo {
              x: Bar = 0
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "There is no type named `Bar` in this file.",
        );
    }

    #[test]
    fn validate_non_existent_field_type_different_file() {
        let foo_namespace = Namespace {
            components: vec!["foo".into()],
        };
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t' as bar

            struct Foo {
              x: bar.Bar = 0
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            struct Qux {
            }
        "
        .to_owned();

        let foo_tokens = tokenize(&foo_path, &foo_contents).unwrap();
        let mut foo_schema = parse(&foo_path, &foo_contents, &foo_tokens).unwrap();
        foo_schema.imports.get_mut(&"bar".into()).unwrap().namespace = Some(bar_namespace.clone());

        let bar_tokens = tokenize(&bar_path, &bar_contents).unwrap();
        let bar_schema = parse(&bar_path, &bar_contents, &bar_tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(foo_namespace, (foo_schema, foo_path, foo_contents));
        schemas.insert(bar_namespace, (bar_schema, bar_path, bar_contents));

        assert_fails!(
            validate(&schemas),
            "There is no type named `Bar` in import `bar`.",
        );
    }
}
