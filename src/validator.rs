use {
    crate::{
        error::{listing, throw, Error},
        format::CodeStr,
        identifier::Identifier,
        schema,
    },
    std::{
        collections::{BTreeMap, HashMap, HashSet},
        path::{Path, PathBuf},
    },
};

// The index will be encoded as a 64-bit integer, but two of the bits are used to help encode the
// size of the field. So the maximum index is 2^62 - 1.
const MAX_FIELD_INDEX: usize = (1 << 62) - 1;

// This function validates a schema and its transitive dependencies.
pub fn validate(
    schemas: &BTreeMap<schema::Namespace, (schema::Schema, PathBuf, String)>,
) -> Result<(), Vec<Error>> {
    // We'll add any errors to this.
    let mut errors: Vec<Error> = vec![];

    // For the purpose of validating types, construct a map from (namespace, name) to
    // (schema, declaration).
    let mut all_types = HashMap::new();
    for (namespace, (schema, _, _)) in schemas {
        for (name, declaration) in &schema.declarations {
            match &declaration.variant {
                schema::DeclarationVariant::Struct(_) | schema::DeclarationVariant::Choice(_) => {
                    all_types.insert((namespace.clone(), name.clone()), (schema, declaration));
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
                    let mut field_names = HashSet::new();
                    let mut field_indices = HashSet::new();

                    for field in fields {
                        // Check that the name of the field is unique within the declaration.
                        if !field_names.insert(field.name.clone()) {
                            errors.push(throw::<Error>(
                                &format!(
                                    "A field named {} already exists in this declaration.",
                                    field.name.code_str(),
                                ),
                                Some(source_path),
                                Some(&listing(source_contents, field.source_range)),
                                None,
                            ));
                        }

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

                        // Check that the index isn't too big.
                        if field.index > MAX_FIELD_INDEX {
                            errors.push(throw::<Error>(
                                &format!(
                                    "Field index {} is too large. The maximum field index is {}.",
                                    field.index.to_string().code_str(),
                                    MAX_FIELD_INDEX.to_string().code_str(),
                                ),
                                Some(source_path),
                                Some(&listing(source_contents, field.source_range)),
                                None,
                            ));
                        }

                        // Validate the type.
                        validate_type(
                            &all_types,
                            &mut errors,
                            namespace,
                            schema,
                            source_path,
                            source_contents,
                            &field.r#type,
                        );
                    }
                }
            }
        }
    }

    // Check for cycles if the schemas are otherwise valid
    // [tag:schemas_valid_except_possible_cycles].
    if errors.is_empty() {
        let mut types_checked = HashSet::new();
        let mut types_visited_set = HashSet::new();
        let mut types_visited_vec = vec![];

        for (namespace, (schema, _, _)) in schemas {
            for name in schema.declarations.keys() {
                check_declaration_for_cycles(
                    &all_types,
                    &mut types_checked,
                    &mut types_visited_set,
                    &mut types_visited_vec,
                    &mut errors,
                    namespace,
                    name,
                );
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

// This function validates an individual type.
fn validate_type(
    all_types: &HashMap<(schema::Namespace, Identifier), (&schema::Schema, &schema::Declaration)>,
    errors: &mut Vec<Error>,
    namespace: &schema::Namespace,
    schema: &schema::Schema,
    source_path: &Path,
    source_contents: &str,
    r#type: &schema::Type,
) {
    match &r#type.variant {
        schema::TypeVariant::Array(inner_type) => {
            validate_type(
                all_types,
                errors,
                namespace,
                schema,
                source_path,
                source_contents,
                inner_type,
            );
        }
        schema::TypeVariant::Bool
        | schema::TypeVariant::Bytes
        | schema::TypeVariant::F64
        | schema::TypeVariant::S64
        | schema::TypeVariant::String
        | schema::TypeVariant::U64
        | schema::TypeVariant::Unit => {}
        schema::TypeVariant::Custom(import, name) => {
            // Determine which file the type is from.
            let type_namespace = if let Some(import) = import {
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
                        Some(&listing(source_contents, r#type.source_range)),
                        None,
                    ));

                    return;
                }
            } else {
                namespace.clone()
            };

            // Check that the type exists in that file.
            if !all_types.contains_key(&(type_namespace, name.clone())) {
                errors.push(throw::<Error>(
                    &if let Some(import) = import {
                        format!(
                            "There is no type named {} in import {}.",
                            name.code_str(),
                            import.code_str(),
                        )
                    } else {
                        format!("There is no type named {} in this file.", name.code_str())
                    },
                    Some(source_path),
                    Some(&listing(source_contents, r#type.source_range)),
                    None,
                ));
            }
        }
    }
}

// This function checks that declarations have no cycles.
fn check_declaration_for_cycles(
    all_types: &HashMap<(schema::Namespace, Identifier), (&schema::Schema, &schema::Declaration)>,
    types_checked: &mut HashSet<(schema::Namespace, Identifier)>,
    types_visited_set: &mut HashSet<(schema::Namespace, Identifier)>,
    types_visited_vec: &mut Vec<(schema::Namespace, Identifier)>,
    errors: &mut Vec<Error>,
    namespace: &schema::Namespace,
    name: &Identifier,
) {
    // Compute this once here so we don't have to compute it in multiple places below.
    let qualified_type = (namespace.clone(), name.clone());

    // Stop if we've already checked this type.
    if types_checked.contains(&qualified_type) {
        return;
    }

    // Visit this type or report a cycle if the type has already been visited.
    types_visited_vec.push(qualified_type.clone());
    if !types_visited_set.insert(qualified_type.clone()) {
        errors.push(throw::<Error>(
            &format!(
                "Cycle detected: {}.",
                types_visited_vec
                    .iter()
                    .map(|(namespace, name)| {
                        let mut namespace = namespace.clone();
                        namespace.components.push(name.clone());
                        namespace.to_string().code_str().to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" \u{2192} "),
            ),
            None,
            None,
            None,
        ));

        // Un-visit this type.
        types_visited_vec.pop();

        // Record that the type has been checked to avoid reporting this cycle multiple times. Note
        // that we haven't necessarily checked all the fields of this type (and the fields of the
        // types of those fields, etc.), but we know they are being checked by a call higher in the
        // stack (since this is the second time we've seen this type in this call stack).
        types_checked.insert(qualified_type);

        return;
    }

    // Check the type of each field. The `unwrap` is safe due to
    // [ref:schemas_valid_except_possible_cycles].
    let (schema, declaration) = all_types.get(&qualified_type).unwrap();
    match &declaration.variant {
        schema::DeclarationVariant::Struct(fields) | schema::DeclarationVariant::Choice(fields) => {
            for field in fields {
                check_type_for_cycles(
                    all_types,
                    types_checked,
                    types_visited_set,
                    types_visited_vec,
                    errors,
                    namespace,
                    schema,
                    &field.r#type,
                );
            }
        }
    }

    // Un-visit this type.
    types_visited_set.remove(&qualified_type);
    types_visited_vec.pop();

    // Record that the type has been checked.
    types_checked.insert(qualified_type);
}

// This function checks that types have no cycles.
#[allow(clippy::too_many_arguments)]
fn check_type_for_cycles(
    all_types: &HashMap<(schema::Namespace, Identifier), (&schema::Schema, &schema::Declaration)>,
    types_checked: &mut HashSet<(schema::Namespace, Identifier)>,
    types_visited_set: &mut HashSet<(schema::Namespace, Identifier)>,
    types_visited_vec: &mut Vec<(schema::Namespace, Identifier)>,
    errors: &mut Vec<Error>,
    namespace: &schema::Namespace,
    schema: &schema::Schema,
    r#type: &schema::Type,
) {
    match &r#type.variant {
        schema::TypeVariant::Array(inner_type) => {
            check_type_for_cycles(
                all_types,
                types_checked,
                types_visited_set,
                types_visited_vec,
                errors,
                namespace,
                schema,
                inner_type,
            );
        }
        schema::TypeVariant::Bool
        | schema::TypeVariant::Bytes
        | schema::TypeVariant::F64
        | schema::TypeVariant::S64
        | schema::TypeVariant::String
        | schema::TypeVariant::U64
        | schema::TypeVariant::Unit => {}
        schema::TypeVariant::Custom(import, name) => {
            let type_namespace = import.as_ref().map_or_else(
                || namespace.clone(),
                |import|
                    // The first `unwrap` is safe due to
                    // [ref:schemas_valid_except_possible_cycles]. The second `unwrap`
                    // is safe due to [ref:namespace_populated].
                    schema
                    .imports
                    .get(import)
                    .unwrap()
                    .namespace
                    .clone()
                    .unwrap(),
            );

            check_declaration_for_cycles(
                all_types,
                types_checked,
                types_visited_set,
                types_visited_vec,
                errors,
                &type_namespace,
                name,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            assert_fails, assert_same, parser::parse, schema::Namespace, tokenizer::tokenize,
            validator::validate,
        },
        std::{collections::BTreeMap, path::Path},
    };

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
            import 'bar.t'

            struct foo {
              x: bar.bar = 0
              y: [bar.bar] = 1
              unstable z: string = 2
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            choice bar {
              x: bool = 0
              y: f64 = 1
              optional z: u64 = 2
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

        assert_same!(validate(&schemas), Ok(()));
    }

    #[test]
    fn validate_duplicate_struct_field_names() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct bar {
              x: bool = 0
              x: f64 = 1
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "A field named `x` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_duplicate_struct_field_indices() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct bar {
              x: bool = 0
              y: f64 = 0
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
    fn validate_duplicate_choice_field_names() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            choice bar {
              x: bool = 0
              x: f64 = 1
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "A field named `x` already exists in this declaration.",
        );
    }

    #[test]
    fn validate_duplicate_choice_field_indices() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            choice bar {
              x: bool = 0
              y: f64 = 0
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
            struct foo {
              x: bar.bar = 0
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
    fn validate_non_existent_field_import_in_array() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct foo {
              x: [bar.bar] = 0
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
            struct foo {
              x: bar = 0
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "There is no type named `bar` in this file.",
        );
    }

    #[test]
    fn validate_non_existent_field_type_in_array_same_file() {
        let namespace = Namespace {
            components: vec!["foo".into()],
        };
        let path = Path::new("foo.t").to_owned();
        let contents = "
            struct foo {
              x: [bar] = 0
            }
        "
        .to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();

        let mut schemas = BTreeMap::new();
        schemas.insert(namespace, (schema, path, contents));

        assert_fails!(
            validate(&schemas),
            "There is no type named `bar` in this file.",
        );
    }

    #[test]
    fn validate_non_existent_field_type_different_file() {
        let foo_namespace = Namespace {
            components: vec!["foo".into()],
        };
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t'

            struct foo {
              x: bar.bar = 0
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            struct qux {
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
            "There is no type named `bar` in import `bar`.",
        );
    }

    #[test]
    fn validate_non_existent_field_type_in_array_different_file() {
        let foo_namespace = Namespace {
            components: vec!["foo".into()],
        };
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t'

            struct foo {
              x: [bar.bar] = 0
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            struct qux {
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
            "There is no type named `bar` in import `bar`.",
        );
    }

    #[test]
    fn validate_cycle() {
        let foo_namespace = Namespace {
            components: vec!["foo".into()],
        };
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t'

            struct foo {
              x: bar.bar = 0
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            import 'foo.t'

            choice bar {
              x: foo.foo = 0
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

        assert_fails!(
            validate(&schemas),
            "Cycle detected: `bar.bar` \u{2192} `foo.foo` \u{2192} `bar.bar`.",
        );
    }

    #[test]
    fn validate_cycle_in_array() {
        let foo_namespace = Namespace {
            components: vec!["foo".into()],
        };
        let foo_path = Path::new("foo.t").to_owned();
        let foo_contents = "
            import 'bar.t'

            struct foo {
              x: [bar.bar] = 0
            }
        "
        .to_owned();

        let bar_namespace = Namespace {
            components: vec!["bar".into()],
        };
        let bar_path = Path::new("bar.t").to_owned();
        let bar_contents = "
            import 'foo.t'

            choice bar {
              x: foo.foo = 0
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

        assert_fails!(
            validate(&schemas),
            "Cycle detected: `bar.bar` \u{2192} `foo.foo` \u{2192} `bar.bar`.",
        );
    }
}
