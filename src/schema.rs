use crate::error::SourceRange;
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct Schema {
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Namespace {
    // This is a representation of a path to a schema, relative to the base directory, i.e., the
    // parent of the schema path provided by the user. However, it differs from paths as follows:
    // - It doesn't include the file extension in the final component.
    // - It can only contain "normal" path components. For example, `.` and `..` are not allowed.
    // - It must be in `snake_case` to facilitate uniqueness validation
    //   [ref:names_unique_after_normalization].
    pub components: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Import {
    pub source_range: SourceRange,
    pub path: PathBuf, // The literal path as it appears in the source file
    pub namespace: Option<Namespace>, // A normalized form of the path
    pub name: String,  // Non-empty due to [ref:identifier_non_empty]
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub source_range: SourceRange,
    pub variant: DeclarationVariant,
}

#[derive(Clone, Debug)]
pub enum DeclarationVariant {
    Struct(String, Vec<Field>), // (non-empty name [ref:identifier_non_empty], fields)
    Choice(String, Vec<Field>), // (non-empty name [ref:identifier_non_empty], fields)
}

#[derive(Clone, Debug)]
pub struct Field {
    pub source_range: SourceRange,
    pub name: String, // Non-empty due to [ref:identifier_non_empty]
    pub restricted: bool,
    pub r#type: Type,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub source_range: SourceRange,
    pub import: Option<String>,
    pub name: String, // Non-empty due to [ref:identifier_non_empty]
}

// This function returns takes two namespaces and returns a version of the former relative to the
// latter. The `usize` in the return value corresponds to the number of `..` that is understood to
// come before the returned namespace (since namespaces don't have a way to encode this
// information).
pub fn relativize_namespace(namespace1: &Namespace, namespace2: &Namespace) -> (Namespace, usize) {
    // Compute when the namespaces diverge.
    let mut common_components: usize = 0;
    while let Some(component1) = namespace1.components.get(common_components) {
        if let Some(component2) = namespace2.components.get(common_components) {
            if component1 == component2 {
                common_components += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Construct and return the namespace.
    (
        Namespace {
            components: namespace1.components[common_components..].to_owned(),
        },
        namespace2.components.len() - common_components,
    )
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut skip_blank_line = true;

        for import in &self.imports {
            if skip_blank_line {
                skip_blank_line = false;
            } else {
                writeln!(f)?;
            }

            write!(f, "import '{}' as {}", import.path.display(), import.name)?;
        }

        for declaration in &self.declarations {
            if skip_blank_line {
                skip_blank_line = false;
            } else {
                writeln!(f, "\n")?;
            }

            write!(f, "{}", declaration)?;
        }
        Ok(())
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant)?;
        Ok(())
    }
}

impl Display for DeclarationVariant {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Struct(name, fields) => {
                writeln!(f, "struct {} {{", name)?;

                for field in fields.iter() {
                    writeln!(f, "{}", field)?;
                }

                write!(f, "}}")?;

                Ok(())
            }
            Self::Choice(name, fields) => {
                writeln!(f, "choice {} {{", name)?;

                for field in fields.iter() {
                    writeln!(f, "{}", field)?;
                }

                write!(f, "}}")?;

                Ok(())
            }
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.restricted {
            write!(
                f,
                "  {}: restricted {} = {}",
                self.name, self.r#type, self.index,
            )?;
        } else {
            write!(f, "  {}: {} = {}", self.name, self.r#type, self.index)?;
        }

        Ok(())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(import) = &self.import {
            write!(f, "{}.{}", import, self.name)?;
        } else {
            write!(f, "{}", self.name)?;
        }
        Ok(())
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.components.join("."))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_same,
        error::SourceRange,
        schema::{
            relativize_namespace, Declaration, DeclarationVariant, Field, Import, Namespace,
            Schema, Type,
        },
    };
    use std::path::Path;

    #[test]
    fn relativize_namespace_both_empty() {
        assert_same!(
            relativize_namespace(
                &Namespace { components: vec![] },
                &Namespace { components: vec![] },
            ),
            (Namespace { components: vec![] }, 0),
        );
    }

    #[test]
    fn relativize_namespace_first_empty() {
        assert_same!(
            relativize_namespace(
                &Namespace { components: vec![] },
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
            ),
            (Namespace { components: vec![] }, 2),
        );
    }

    #[test]
    fn relativize_namespace_second_empty() {
        assert_same!(
            relativize_namespace(
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
                &Namespace { components: vec![] },
            ),
            (
                Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()]
                },
                0,
            ),
        );
    }

    #[test]
    fn relativize_namespace_no_overlap() {
        assert_same!(
            relativize_namespace(
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
                &Namespace {
                    components: vec!["baz".to_owned(), "qux".to_owned()],
                },
            ),
            (
                Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()]
                },
                2,
            ),
        );
    }

    #[test]
    fn relativize_namespace_some_overlap() {
        assert_same!(
            relativize_namespace(
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
                &Namespace {
                    components: vec!["foo".to_owned(), "baz".to_owned()],
                },
            ),
            (
                Namespace {
                    components: vec!["bar".to_owned()]
                },
                1,
            ),
        );
    }

    #[test]
    fn relativize_namespace_complete_overlap() {
        assert_same!(
            relativize_namespace(
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
            ),
            (Namespace { components: vec![] }, 0),
        );
    }

    #[test]
    fn relativize_namespace_child() {
        assert_same!(
            relativize_namespace(
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
                },
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
            ),
            (
                Namespace {
                    components: vec!["baz".to_owned()]
                },
                0,
            ),
        );
    }

    #[test]
    fn relativize_namespace_parent() {
        assert_same!(
            relativize_namespace(
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
                &Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
                },
            ),
            (Namespace { components: vec![] }, 1),
        );
    }

    #[test]
    fn schema_empty_display() {
        assert_eq!(
            format!(
                "{}",
                Schema {
                    imports: vec![],
                    declarations: vec![],
                },
            ),
            "",
        );
    }

    #[test]
    fn schema_imports_only_display() {
        assert_eq!(
            format!(
                "{}",
                Schema {
                    imports: vec![
                        Import {
                            source_range: SourceRange { start: 0, end: 0 },
                            path: Path::new("foo.t").to_owned(),
                            namespace: None,
                            name: "foo".to_owned(),
                        },
                        Import {
                            source_range: SourceRange { start: 0, end: 0 },
                            path: Path::new("bar.t").to_owned(),
                            namespace: None,
                            name: "bar".to_owned(),
                        },
                    ],
                    declarations: vec![],
                },
            ),
            "\
                import 'foo.t' as foo\n\
                import 'bar.t' as bar\
            ",
        );
    }

    #[test]
    fn schema_declarations_only_display() {
        assert_eq!(
            format!(
                "{}",
                Schema {
                    imports: vec![],
                    declarations: vec![
                        Declaration {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: DeclarationVariant::Struct(
                                "Foo".to_owned(),
                                vec![
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "x".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 0,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "y".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 1,
                                    },
                                ],
                            ),
                        },
                        Declaration {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: DeclarationVariant::Choice(
                                "Bar".to_owned(),
                                vec![
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "x".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 0,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "y".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 1,
                                    },
                                ],
                            ),
                        },
                    ],
                },
            ),
            "\
                struct Foo {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\n\
                \n\
                choice Bar {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\
            ",
        );
    }

    #[test]
    fn schema_imports_and_declarations_display() {
        assert_eq!(
            format!(
                "{}",
                Schema {
                    imports: vec![
                        Import {
                            source_range: SourceRange { start: 0, end: 0 },
                            path: Path::new("foo.t").to_owned(),
                            namespace: None,
                            name: "foo".to_owned(),
                        },
                        Import {
                            source_range: SourceRange { start: 0, end: 0 },
                            path: Path::new("bar.t").to_owned(),
                            namespace: None,
                            name: "bar".to_owned(),
                        },
                    ],
                    declarations: vec![
                        Declaration {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: DeclarationVariant::Struct(
                                "Foo".to_owned(),
                                vec![
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "x".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 0,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "y".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 1,
                                    },
                                ],
                            ),
                        },
                        Declaration {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: DeclarationVariant::Choice(
                                "Bar".to_owned(),
                                vec![
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "x".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 0,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "y".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 1,
                                    },
                                ],
                            ),
                        },
                    ],
                },
            ),
            "\
                import 'foo.t' as foo\n\
                import 'bar.t' as bar\n\
                \n\
                struct Foo {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\n\
                \n\
                choice Bar {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\
            ",
        );
    }

    #[test]
    fn declaration_display() {
        assert_eq!(
            format!(
                "{}",
                Declaration {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: DeclarationVariant::Struct(
                        "Foo".to_owned(),
                        vec![
                            Field {
                                source_range: SourceRange { start: 0, end: 0 },
                                name: "x".to_owned(),
                                restricted: false,
                                r#type: Type {
                                    source_range: SourceRange { start: 0, end: 0 },
                                    import: None,
                                    name: "Int".to_owned(),
                                },
                                index: 0,
                            },
                            Field {
                                source_range: SourceRange { start: 0, end: 0 },
                                name: "y".to_owned(),
                                restricted: false,
                                r#type: Type {
                                    source_range: SourceRange { start: 0, end: 0 },
                                    import: None,
                                    name: "String".to_owned(),
                                },
                                index: 1,
                            },
                        ],
                    ),
                },
            ),
            "\
                struct Foo {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\
            ",
        );
    }

    #[test]
    fn declaration_variant_struct_display() {
        assert_eq!(
            format!(
                "{}",
                DeclarationVariant::Struct(
                    "Foo".to_owned(),
                    vec![
                        Field {
                            source_range: SourceRange { start: 0, end: 0 },
                            name: "x".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "Int".to_owned(),
                            },
                            index: 0,
                        },
                        Field {
                            source_range: SourceRange { start: 0, end: 0 },
                            name: "y".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "String".to_owned(),
                            },
                            index: 1,
                        },
                    ],
                ),
            ),
            "\
                struct Foo {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\
            ",
        );
    }

    #[test]
    fn declaration_variant_choice_display() {
        assert_eq!(
            format!(
                "{}",
                DeclarationVariant::Choice(
                    "Foo".to_owned(),
                    vec![
                        Field {
                            source_range: SourceRange { start: 0, end: 0 },
                            name: "x".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "Int".to_owned(),
                            },
                            index: 0,
                        },
                        Field {
                            source_range: SourceRange { start: 0, end: 0 },
                            name: "y".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "String".to_owned(),
                            },
                            index: 1,
                        },
                    ],
                ),
            ),
            "\
                choice Foo {\n\
                \x20 x: Int = 0\n\
                \x20 y: String = 1\n\
                }\
            ",
        );
    }

    #[test]
    fn field_display_non_restricted() {
        assert_eq!(
            format!(
                "{}",
                Field {
                    source_range: SourceRange { start: 0, end: 0 },
                    name: "x".to_owned(),
                    restricted: false,
                    r#type: Type {
                        source_range: SourceRange { start: 0, end: 0 },
                        import: None,
                        name: "Int".to_owned(),
                    },
                    index: 0,
                },
            ),
            "  x: Int = 0",
        );
    }

    #[test]
    fn field_display_restricted() {
        assert_eq!(
            format!(
                "{}",
                Field {
                    source_range: SourceRange { start: 0, end: 0 },
                    name: "x".to_owned(),
                    restricted: true,
                    r#type: Type {
                        source_range: SourceRange { start: 0, end: 0 },
                        import: None,
                        name: "Int".to_owned(),
                    },
                    index: 0,
                },
            ),
            "  x: restricted Int = 0",
        );
    }

    #[test]
    fn type_display_no_import() {
        assert_eq!(
            format!(
                "{}",
                Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    import: None,
                    name: "Int".to_owned(),
                },
            ),
            "Int",
        );
    }

    #[test]
    fn type_display_import() {
        assert_eq!(
            format!(
                "{}",
                Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    import: Some("foo".to_owned()),
                    name: "Int".to_owned(),
                },
            ),
            "foo.Int",
        );
    }

    #[test]
    fn namespace_display_empty() {
        assert_eq!(format!("{}", Namespace { components: vec![] }), "");
    }

    #[test]
    fn namespace_display_single() {
        assert_eq!(
            format!(
                "{}",
                Namespace {
                    components: vec!["foo".to_owned()],
                },
            ),
            "foo",
        );
    }

    #[test]
    fn namespace_display_multiple() {
        assert_eq!(
            format!(
                "{}",
                Namespace {
                    components: vec!["foo".to_owned(), "bar".to_owned()],
                },
            ),
            "foo.bar",
        );
    }
}
