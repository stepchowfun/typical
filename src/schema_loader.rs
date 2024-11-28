use {
    crate::{
        error::{listing, throw, Error},
        format::CodeStr,
        parser::parse,
        schema,
        tokenizer::tokenize,
    },
    std::{
        borrow::ToOwned,
        collections::{BTreeMap, HashSet},
        fs::read_to_string,
        io::{self, ErrorKind},
        path::PathBuf,
        path::{Component, Path},
    },
};

// Convert a path to a namespace. This function will panic if the path cannot be converted into a
// namespace (e.g., because it contains `..`).
fn path_to_namespace(path: &Path) -> schema::Namespace {
    let mut path = path.to_owned();
    path.set_extension("");

    schema::Namespace {
        components: path
            .components()
            .map(|component| {
                if let Component::Normal(component) = component {
                    component.to_string_lossy().to_string().as_str().into()
                } else {
                    panic!()
                }
            })
            .collect(),
    }
}

// Load a schema and its transitive dependencies. The imports in the returned schemas are guaranteed
// to resolve.
#[allow(clippy::too_many_lines)]
#[allow(clippy::type_complexity)]
pub fn load_schemas(
    schema_path: &Path,
) -> Result<BTreeMap<schema::Namespace, (schema::Schema, PathBuf, String)>, Vec<Error>> {
    // The schema and all its transitive dependencies will end up here.
    let mut schemas = BTreeMap::new();

    // Any errors will end up here.
    let mut errors = vec![];

    // The base directory for the schema's dependencies is the directory containing the schema.
    let Some(base_path) = schema_path.parent() else {
        errors.push(throw::<Error>(
            &format!(
                "{} is not a file.",
                schema_path.to_string_lossy().code_str(),
            ),
            None,
            None,
            None,
        ));

        return Err(errors);
    };

    // Canonicalize the path to the base directory. This will be used to calculate namespaces below.
    // Note that even with this we still need `base_path` from above, because canonicalization on
    // Windows adds a `\\?\` prefix to the path, which changes the meaning of `..` and thus prevents
    // us from joining it with other paths containing `..`. Note also that we can't simply
    // compute this by canonicalizing `base_path`, since `base_path` might have zero components,
    // which is considered invalid for canonicalization. So, instead, we canonicalize `schma_path`
    // and take the parent of the result.
    let canonical_base_path = match schema_path
        .canonicalize()
        .and_then(|canonical_schema_path| {
            canonical_schema_path
                .parent()
                .map(ToOwned::to_owned)
                .ok_or_else(|| io::Error::from(ErrorKind::Other))
        }) {
        Ok(canonical_base_path) => canonical_base_path,
        Err(error) => {
            errors.push(throw(
                &format!(
                    "{} is not a file.",
                    schema_path.to_string_lossy().code_str(),
                ),
                None,
                None,
                Some(error),
            ));

            return Err(errors);
        }
    };

    // Relative to the base directory, the path to the schema is the name of the schema file
    // [tag:based_schema_path_is_file_name].
    let based_schema_path = if let Some(based_schema_path) = schema_path.file_name() {
        AsRef::<Path>::as_ref(based_schema_path)
    } else {
        errors.push(throw::<Error>(
            &format!(
                "{} is not a file.",
                schema_path.to_string_lossy().code_str(),
            ),
            None,
            None,
            None,
        ));

        return Err(errors);
    };

    // Compute the namespace of the schema. This is safe due to
    // [ref:based_schema_path_is_file_name].
    let schema_namespace = path_to_namespace(based_schema_path);

    // Initialize the "frontier" with the given path. Paths in the frontier are relative to
    // `base_path` [tag:frontier_paths_based].
    let mut schemas_to_load = vec![(
        schema_namespace.clone(),
        based_schema_path.to_owned(),
        None as Option<(PathBuf, String)>,
    )];
    let mut visited_namespaces = HashSet::new();
    visited_namespaces.insert(schema_namespace);

    // Perform a depth-first traversal of the transitive dependencies.
    while let Some((namespace, path, origin)) = schemas_to_load.pop() {
        // Read the file.
        let contents = match read_to_string(base_path.join(&path)) {
            Ok(contents) => contents,
            Err(error) => {
                let message = format!("Unable to load {}.", path.to_string_lossy().code_str());

                if let Some((origin_path, origin_listing)) = origin {
                    errors.push(throw(
                        &message,
                        Some(&origin_path),
                        Some(&origin_listing),
                        Some(error),
                    ));
                } else {
                    errors.push(throw(&message, None, None, Some(error)));
                }

                continue;
            }
        };

        // Tokenize the contents.
        let tokens = match tokenize(&path, &contents) {
            Ok(tokens) => tokens,
            Err(error) => {
                errors.extend_from_slice(&error);

                continue;
            }
        };

        // Parse the tokens.
        let mut schema = match parse(&path, &contents, &tokens) {
            Ok(schema) => schema,
            Err(error) => {
                errors.extend_from_slice(&error);

                continue;
            }
        };

        // Compute the base directory for this schema's dependencies. The `unwrap` is safe due to
        // [ref:frontier_paths_based].
        let parent_path = path.parent().unwrap();

        // Add the dependencies to the frontier.
        for import in schema.imports.values_mut() {
            // Compute the source listing for this import for error reporting.
            let origin_listing = listing(&contents, import.source_range);

            // Compute the import path.
            let non_canonical_import_path = base_path.join(parent_path.join(&import.path));

            // Canonicalize the import path.
            let canonical_import_path = match non_canonical_import_path.canonicalize() {
                Ok(canonical_import_path) => canonical_import_path,
                Err(error) => {
                    errors.push(throw(
                        &format!(
                            "Unable to load {}.",
                            non_canonical_import_path.to_string_lossy().code_str(),
                        ),
                        Some(&path),
                        Some(&origin_listing),
                        Some(error),
                    ));

                    continue;
                }
            };

            // Strip the base path from the schema path. Since this is computed from two canonical
            // paths, its guaranteed to contain only normal components
            // [tag:based_import_path_only_has_normal_components].
            let based_import_path = if let Ok(based_import_path) =
                canonical_import_path.strip_prefix(&canonical_base_path)
            {
                based_import_path.to_owned()
            } else {
                errors.push(throw::<Error>(
                    &format!(
                        "{} is not a descendant of {}, which is the base directory for this run.",
                        canonical_import_path.to_string_lossy().code_str(),
                        canonical_base_path.to_string_lossy().code_str(),
                    ),
                    Some(&path),
                    Some(&origin_listing),
                    None,
                ));

                continue;
            };

            // Populate the namespace of the import [tag:namespace_populated]. The
            // path-to-namespace conversion is safe due to
            // [ref:based_import_path_only_has_normal_components].
            let import_namespace = path_to_namespace(&based_import_path);
            import.namespace = Some(import_namespace.clone());

            // Visit this import if it hasn't been visited already.
            if !visited_namespaces.contains(&import_namespace) {
                visited_namespaces.insert(import_namespace.clone());
                schemas_to_load.push((
                    import_namespace,
                    based_import_path,
                    Some((path.clone(), origin_listing)),
                ));
            }
        }

        // Store the schema.
        if let Some((_, conflicting_schema_path, _)) =
            schemas.insert(namespace.clone(), (schema, path.clone(), contents))
        {
            errors.push(throw::<Error>(
                &format!(
                    "This file conflicts with {}, since both correspond to the same namespace {}.",
                    conflicting_schema_path.to_string_lossy().code_str(),
                    namespace.to_string().code_str(),
                ),
                Some(&path),
                None,
                None,
            ));
        }
    }

    // Return a success or report any errors.
    if errors.is_empty() {
        Ok(schemas)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            schema::Namespace,
            schema_loader::{load_schemas, path_to_namespace},
        },
        std::path::Path,
    };

    #[test]
    fn path_to_namespace_empty() {
        assert_eq!(
            path_to_namespace(Path::new("")),
            Namespace { components: vec![] },
        );
    }

    #[test]
    fn path_to_namespace_single() {
        assert_eq!(
            path_to_namespace(Path::new("foo")),
            Namespace {
                components: vec!["foo".into()],
            },
        );
    }

    #[test]
    fn path_to_namespace_double() {
        assert_eq!(
            path_to_namespace(Path::new("foo/bar")),
            Namespace {
                components: vec!["foo".into(), "bar".into()],
            },
        );
    }

    #[test]
    fn path_to_namespace_triple() {
        assert_eq!(
            path_to_namespace(Path::new("foo/bar/baz")),
            Namespace {
                components: vec!["foo".into(), "bar".into(), "baz".into()],
            },
        );
    }

    // This test doesn't work on Windows, for some reason.
    #[test]
    fn load_schemas_example() {
        load_schemas(Path::new("integration_tests/types/types.t")).unwrap();
    }
}
