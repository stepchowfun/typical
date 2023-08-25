use {
    crate::{
        error::SourceRange,
        identifier::Identifier,
        token::{
            ASYMMETRIC_KEYWORD, AS_KEYWORD, BOOL_KEYWORD, BYTES_KEYWORD, CHOICE_KEYWORD,
            DELETED_KEYWORD, F64_KEYWORD, IMPORT_KEYWORD, OPTIONAL_KEYWORD, S64_KEYWORD,
            STRING_KEYWORD, STRUCT_KEYWORD, U64_KEYWORD, UNIT_KEYWORD,
        },
    },
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt::{self, Display, Formatter, Write},
        path::PathBuf,
    },
};

const MAX_COLUMNS: usize = 79;

#[derive(Clone, Debug)]
pub struct Schema {
    pub comment: Vec<String>,
    pub imports: BTreeMap<Identifier, Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug)]
pub struct Import {
    pub source_range: SourceRange,
    pub path: PathBuf, // The literal path as it appears in the source file
    pub namespace: Option<Namespace>, // A normalized form of the path
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub source_range: SourceRange,
    pub comment: Vec<String>,
    pub variant: DeclarationVariant,
    pub name: Identifier,
    pub fields: Vec<Field>,
    pub deleted: BTreeSet<usize>,
}

#[derive(Clone, Debug)]
pub enum DeclarationVariant {
    Struct,
    Choice,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub source_range: SourceRange,
    pub comment: Vec<String>,
    pub rule: Rule,
    pub name: Identifier,
    pub r#type: Type,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub enum Rule {
    Asymmetric,
    Optional,
    Required,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub source_range: SourceRange,
    pub variant: TypeVariant,
}

#[derive(Clone, Debug)]
pub enum TypeVariant {
    Array(Box<Type>),
    Bool,
    Bytes,
    Custom(Option<Identifier>, Identifier), // (import, name)
    F64,
    S64,
    String,
    U64,
    Unit,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Namespace {
    // This is a representation of a path to a schema, relative to the base directory, i.e., the
    // parent of the schema path provided by the user. However, it differs from paths as follows:
    // - It doesn't include the file extension in the final component.
    // - It can only contain "normal" path components. For example, `.` and `..` are not allowed.
    pub components: Vec<Identifier>,
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

// Write the paragraphs of a comment separated by line breaks.
fn write_comment<W: Write>(indentation: &str, paragraphs: &[String], f: &mut W) -> fmt::Result {
    for (i, paragraph) in paragraphs.iter().enumerate() {
        if i != 0 {
            writeln!(f, "{indentation}#")?;
        }

        for line in textwrap::fill(paragraph, MAX_COLUMNS - indentation.len() - 2).lines() {
            writeln!(f, "{indentation}# {line}")?;
        }
    }

    Ok(())
}

impl Schema {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        write_comment("", &self.comment, f)?;

        if !self.comment.is_empty() && (!self.imports.is_empty() || !self.declarations.is_empty()) {
            writeln!(f)?;
        }

        for (name, import) in &self.imports {
            import.write(f, name)?;
        }

        let mut skip_blank_line = self.imports.is_empty();

        for declaration in &self.declarations {
            if skip_blank_line {
                skip_blank_line = false;
            } else {
                writeln!(f)?;
            }

            declaration.write(f)?;
        }

        Ok(())
    }
}

impl Import {
    fn write<W: Write>(&self, f: &mut W, name: &Identifier) -> fmt::Result {
        if self
            .path
            .file_stem()
            .map(|file_stem| file_stem.to_string_lossy().as_ref().into())
            .as_ref()
            == Some(name)
        {
            writeln!(f, "{} '{}'", IMPORT_KEYWORD, self.path.display())
        } else {
            writeln!(
                f,
                "{} '{}' {} {}",
                IMPORT_KEYWORD,
                self.path.display(),
                AS_KEYWORD,
                name.snake_case(),
            )
        }
    }
}

impl Declaration {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        write_comment("", &self.comment, f)?;

        self.variant.write(f)?;

        writeln!(f, " {} {{", self.name.pascal_case())?;

        let mut previous_field_has_comment = false;

        for (i, field) in self.fields.iter().enumerate() {
            if (previous_field_has_comment || !field.comment.is_empty()) && i != 0 {
                writeln!(f)?;
            }

            previous_field_has_comment = !field.comment.is_empty();

            field.write(f)?;
        }

        if !self.fields.is_empty() && !self.deleted.is_empty() {
            writeln!(f)?;
        }

        if !self.deleted.is_empty() {
            write!(f, "    {DELETED_KEYWORD}")?;

            for deleted_index in &self.deleted {
                write!(f, " {deleted_index}")?;
            }

            writeln!(f)?;
        }

        writeln!(f, "}}")
    }
}

impl DeclarationVariant {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Self::Struct => write!(f, "{STRUCT_KEYWORD}"),
            Self::Choice => write!(f, "{CHOICE_KEYWORD}"),
        }
    }
}

impl Field {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        write_comment("    ", &self.comment, f)?;

        match self.rule {
            Rule::Asymmetric => {
                write!(f, "    {ASYMMETRIC_KEYWORD} ")?;
            }
            Rule::Optional => {
                write!(f, "    {OPTIONAL_KEYWORD} ")?;
            }
            Rule::Required => {
                write!(f, "    ")?;
            }
        }

        write!(f, "{}", self.name.snake_case())?;

        if let TypeVariant::Unit = self.r#type.variant {
        } else {
            write!(f, ": ")?;
            self.r#type.write(f)?;
        }

        writeln!(f, " = {}", self.index)
    }
}

impl Type {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        self.variant.write(f)
    }
}

impl TypeVariant {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Self::Array(inner_type) => {
                write!(f, "[{inner_type}]")?;
            }
            Self::Bool => {
                write!(f, "{BOOL_KEYWORD}")?;
            }
            Self::Bytes => {
                write!(f, "{BYTES_KEYWORD}")?;
            }
            Self::Custom(import, name) => {
                if let Some(import) = import {
                    write!(f, "{}.{}", import.snake_case(), name.pascal_case())?;
                } else {
                    write!(f, "{}", name.pascal_case())?;
                }
            }
            Self::F64 => {
                write!(f, "{F64_KEYWORD}")?;
            }
            Self::S64 => {
                write!(f, "{S64_KEYWORD}")?;
            }
            Self::String => {
                write!(f, "{STRING_KEYWORD}")?;
            }
            Self::U64 => {
                write!(f, "{U64_KEYWORD}")?;
            }
            Self::Unit => {
                write!(f, "{UNIT_KEYWORD}")?;
            }
        }
        Ok(())
    }
}

impl Namespace {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        write!(
            f,
            "{}",
            self.components
                .iter()
                .map(Identifier::snake_case)
                .collect::<Vec<_>>()
                .join("."),
        )?;
        Ok(())
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write(f)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write(f)
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write(f)
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            assert_same,
            error::SourceRange,
            schema::{
                relativize_namespace, Declaration, DeclarationVariant, Field, Import, Namespace,
                Rule, Schema, Type, TypeVariant,
            },
        },
        std::{
            collections::{BTreeMap, BTreeSet},
            path::Path,
        },
    };

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
                    components: vec!["foo".into(), "bar".into()],
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
                    components: vec!["foo".into(), "bar".into()],
                },
                &Namespace { components: vec![] },
            ),
            (
                Namespace {
                    components: vec!["foo".into(), "bar".into()],
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
                    components: vec!["foo".into(), "bar".into()],
                },
                &Namespace {
                    components: vec!["baz".into(), "qux".into()],
                },
            ),
            (
                Namespace {
                    components: vec!["foo".into(), "bar".into()],
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
                    components: vec!["foo".into(), "bar".into()],
                },
                &Namespace {
                    components: vec!["foo".into(), "baz".into()],
                },
            ),
            (
                Namespace {
                    components: vec!["bar".into()],
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
                    components: vec!["foo".into(), "bar".into()],
                },
                &Namespace {
                    components: vec!["foo".into(), "bar".into()],
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
                    components: vec!["foo".into(), "bar".into(), "baz".into()],
                },
                &Namespace {
                    components: vec!["foo".into(), "bar".into()],
                },
            ),
            (
                Namespace {
                    components: vec!["baz".into()],
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
                    components: vec!["foo".into(), "bar".into()],
                },
                &Namespace {
                    components: vec!["foo".into(), "bar".into(), "baz".into()],
                },
            ),
            (Namespace { components: vec![] }, 1),
        );
    }

    #[test]
    fn schema_empty_display() {
        let schema = Schema {
            comment: vec![],
            imports: BTreeMap::new(),
            declarations: vec![],
        };

        let expected = "";

        assert_eq!(schema.to_string(), expected);
    }

    #[test]
    fn schema_imports_only_display() {
        let mut imports = BTreeMap::new();

        imports.insert(
            "foo".into(),
            Import {
                source_range: SourceRange { start: 0, end: 0 },
                path: Path::new("foo.t").to_owned(),
                namespace: None,
            },
        );

        imports.insert(
            "qux".into(),
            Import {
                source_range: SourceRange { start: 0, end: 0 },
                path: Path::new("bar.t").to_owned(),
                namespace: None,
            },
        );

        let schema = Schema {
            comment: vec![],
            imports,
            declarations: vec![],
        };

        let expected = "\
            import 'foo.t'\n\
            import 'bar.t' as qux\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[test]
    fn schema_declarations_only_display() {
        let declarations = vec![
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                comment: vec![],
                variant: DeclarationVariant::Struct,
                name: "foo".into(),
                fields: vec![
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec![],
                        rule: Rule::Required,
                        name: "X".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::Bool,
                        },
                        index: 0,
                    },
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec![],
                        rule: Rule::Optional,
                        name: "Y".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::U64,
                        },
                        index: 1,
                    },
                ],
                deleted: BTreeSet::new(),
            },
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                comment: vec![],
                variant: DeclarationVariant::Choice,
                name: "bar".into(),
                fields: vec![
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec![],
                        rule: Rule::Required,
                        name: "X".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::Bool,
                        },
                        index: 0,
                    },
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec![],
                        rule: Rule::Asymmetric,
                        name: "Y".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::F64,
                        },
                        index: 1,
                    },
                ],
                deleted: BTreeSet::new(),
            },
        ];

        let schema = Schema {
            comment: vec![],
            imports: BTreeMap::new(),
            declarations,
        };

        let expected = "\
            struct Foo {\n\
            \x20   x: Bool = 0\n\
            \x20   optional y: U64 = 1\n\
            }\n\
            \n\
            choice Bar {\n\
            \x20   x: Bool = 0\n\
            \x20   asymmetric y: F64 = 1\n\
            }\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn schema_imports_and_declarations_display() {
        let mut imports = BTreeMap::new();

        imports.insert(
            "Foo".into(),
            Import {
                source_range: SourceRange { start: 0, end: 0 },
                path: Path::new("foo.t").to_owned(),
                namespace: None,
            },
        );

        imports.insert(
            "Qux".into(),
            Import {
                source_range: SourceRange { start: 0, end: 0 },
                path: Path::new("bar.t").to_owned(),
                namespace: None,
            },
        );

        let declarations = vec![
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                comment: vec!["This is a struct.".to_owned()],
                variant: DeclarationVariant::Struct,
                name: "foo".into(),
                fields: vec![
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec!["This is a field.".to_owned()],
                        rule: Rule::Required,
                        name: "X".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::Bool,
                        },
                        index: 0,
                    },
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec!["This is a field.".to_owned()],
                        rule: Rule::Optional,
                        name: "Y".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::U64,
                        },
                        index: 1,
                    },
                ],
                deleted: BTreeSet::from_iter(vec![2, 3, 4]),
            },
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                comment: vec!["This is a choice.".to_owned()],
                variant: DeclarationVariant::Choice,
                name: "bar".into(),
                fields: vec![
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec!["This is a field.".to_owned()],
                        rule: Rule::Required,
                        name: "X".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::Bool,
                        },
                        index: 0,
                    },
                    Field {
                        source_range: SourceRange { start: 0, end: 0 },
                        comment: vec!["This is a field.".to_owned()],
                        rule: Rule::Asymmetric,
                        name: "Y".into(),
                        r#type: Type {
                            source_range: SourceRange { start: 0, end: 0 },
                            variant: TypeVariant::F64,
                        },
                        index: 1,
                    },
                ],
                deleted: BTreeSet::from_iter(vec![2, 3, 4]),
            },
        ];

        let schema = Schema {
            comment: vec![
                "\
                This is a long comment. I can't believe how long this comment is! \
                Surely it will wrap, right?\
            "
                .to_owned(),
                "This is a second paragraph in this comment.".to_owned(),
            ],
            imports,
            declarations,
        };

        let expected = "\
            # This is a long comment. I can't believe how long this comment is! Surely it\n\
            # will wrap, right?\n\
            #\n\
            # This is a second paragraph in this comment.\n\
            \n\
            import 'foo.t'\n\
            import 'bar.t' as qux\n\
            \n\
            # This is a struct.\n\
            struct Foo {\n\
            \x20   # This is a field.\n\
            \x20   x: Bool = 0\n\
            \n\
            \x20   # This is a field.\n\
            \x20   optional y: U64 = 1\n\
            \n\
            \x20   deleted 2 3 4\n\
            }\n\
            \n\
            # This is a choice.\n\
            choice Bar {\n\
            \x20   # This is a field.\n\
            \x20   x: Bool = 0\n\
            \n\
            \x20   # This is a field.\n\
            \x20   asymmetric y: F64 = 1\n\
            \n\
            \x20   deleted 2 3 4\n\
            }\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[test]
    fn type_display_array() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Array(Box::new(Type {
                source_range: SourceRange { start: 0, end: 0 },
                variant: TypeVariant::Bool,
            })),
        };

        let expected = "[Bool]";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_bool() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Bool,
        };

        let expected = "Bool";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_bytes() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Bytes,
        };

        let expected = "Bytes";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_custom_no_import() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Custom(None, "Int".into()),
        };

        let expected = "Int";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_custom_import() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Custom(Some("foo".into()), "Int".into()),
        };

        let expected = "foo.Int";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_f64() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::F64,
        };

        let expected = "F64";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_s64() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::S64,
        };

        let expected = "S64";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_string() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::String,
        };

        let expected = "String";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_u64() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::U64,
        };

        let expected = "U64";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_unit() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Unit,
        };

        let expected = "Unit";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn namespace_display_empty() {
        let namespace = Namespace { components: vec![] };

        let expected = "";

        assert_eq!(namespace.to_string(), expected);
    }

    #[test]
    fn namespace_display_single() {
        let namespace = Namespace {
            components: vec!["Foo".into()],
        };

        let expected = "foo";

        assert_eq!(namespace.to_string(), expected);
    }

    #[test]
    fn namespace_display_multiple() {
        let namespace = Namespace {
            components: vec!["Foo".into(), "Bar".into()],
        };

        let expected = "foo.bar";

        assert_eq!(namespace.to_string(), expected);
    }
}
