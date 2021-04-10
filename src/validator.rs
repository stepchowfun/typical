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
pub fn validate(schemas: &HashMap<PathBuf, (schema::Schema, String)>) -> Result<(), Error> {
    // We'll add any errors to this.
    let mut errors: Vec<Error> = vec![];

    // Check that imports are unique within each file.
    for (path, (schema, source_contents)) in schemas {
        let mut import_names = HashSet::<String>::new();

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
    }

    // Return a success or report any errors.
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error {
            message: errors
                .iter()
                .fold(String::new(), |acc, error| {
                    format!(
                        "{}\n{}{}",
                        acc,
                        // Only render an empty line between errors here if the previous line
                        // doesn't already visually look like an empty line. See
                        // [ref:overline_u203e].
                        if acc
                            .split('\n')
                            .last()
                            .unwrap()
                            .chars()
                            .all(|c| c == ' ' || c == '\u{203e}')
                        {
                            ""
                        } else {
                            "\n"
                        },
                        error,
                    )
                })
                .trim()
                .to_owned(),
            reason: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_same, parser::parse, tokenizer::tokenize, validator::validate};
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
}
