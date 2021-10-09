use {
    crate::{
        error::SourceRange,
        identifier::Identifier,
        token::{ASYMMETRIC_KEYWORD, OPTIONAL_KEYWORD},
    },
    std::{
        collections::BTreeMap,
        fmt::{self, Display, Formatter, Write},
        path::PathBuf,
    },
};

#[derive(Clone, Debug)]
pub struct Schema {
    pub imports: BTreeMap<Identifier, Import>,
    pub declarations: BTreeMap<Identifier, Declaration>,
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
    pub variant: DeclarationVariant,
}

#[derive(Clone, Debug)]
pub enum DeclarationVariant {
    Struct(Vec<Field>),
    Choice(Vec<Field>),
}

#[derive(Clone, Debug)]
pub struct Field {
    pub source_range: SourceRange,
    pub rule: Rule,
    pub name: Identifier,
    pub r#type: Type,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub enum Rule {
    Optional,
    Required,
    Asymmetric,
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

impl Schema {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        for (name, import) in &self.imports {
            import.write(f, name)?;
        }

        let mut skip_blank_line = self.imports.is_empty();

        for (name, declaration) in &self.declarations {
            if skip_blank_line {
                skip_blank_line = false;
            } else {
                writeln!(f)?;
            }

            declaration.write(f, name)?;
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
            writeln!(f, "import '{}'", self.path.display())
        } else {
            writeln!(f, "import '{}' as {}", self.path.display(), name.original())
        }
    }
}

impl Declaration {
    fn write<W: Write>(&self, f: &mut W, name: &Identifier) -> fmt::Result {
        self.variant.write(f, name)
    }
}

impl DeclarationVariant {
    fn write<W: Write>(&self, f: &mut W, name: &Identifier) -> fmt::Result {
        match self {
            Self::Struct(fields) => {
                writeln!(f, "struct {} {{", name.original())?;

                for field in fields {
                    field.write(f)?;
                }

                writeln!(f, "}}")
            }
            Self::Choice(fields) => {
                writeln!(f, "choice {} {{", name.original())?;

                for field in fields {
                    field.write(f)?;
                }

                writeln!(f, "}}")
            }
        }
    }
}

impl Field {
    fn write<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self.rule {
            Rule::Optional => {
                write!(f, "  {} ", OPTIONAL_KEYWORD)?;
            }
            Rule::Required => {
                write!(f, "  ")?;
            }
            Rule::Asymmetric => {
                write!(f, "  {} ", ASYMMETRIC_KEYWORD)?;
            }
        }

        write!(f, "{}: ", self.name.original())?;

        self.r#type.write(f)?;

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
                write!(f, "[{}]", inner_type)?;
            }
            Self::Bool => {
                write!(f, "Bool")?;
            }
            Self::Bytes => {
                write!(f, "Bytes")?;
            }
            Self::Custom(import, name) => {
                if let Some(import) = import {
                    write!(f, "{}.{}", import.original(), name.original())?;
                } else {
                    write!(f, "{}", name.original())?;
                }
            }
            Self::F64 => {
                write!(f, "F64")?;
            }
            Self::S64 => {
                write!(f, "S64")?;
            }
            Self::String => {
                write!(f, "String")?;
            }
            Self::U64 => {
                write!(f, "U64")?;
            }
            Self::Unit => {
                write!(f, "Unit")?;
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
                .map(Identifier::original)
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
        std::{collections::BTreeMap, path::Path},
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
            imports: BTreeMap::new(),
            declarations: BTreeMap::new(),
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
            imports,
            declarations: BTreeMap::new(),
        };

        let expected = "\
            import 'foo.t'\n\
            import 'bar.t' as qux\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[test]
    fn schema_declarations_only_display() {
        let foo_fields = vec![
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Optional,
                name: "y".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::U64,
                },
                index: 1,
            },
        ];

        let bar_fields = vec![
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Asymmetric,
                name: "y".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::F64,
                },
                index: 1,
            },
        ];

        let mut declarations = BTreeMap::new();

        declarations.insert(
            "Foo".into(),
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                variant: DeclarationVariant::Struct(foo_fields),
            },
        );

        declarations.insert(
            "Bar".into(),
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                variant: DeclarationVariant::Choice(bar_fields),
            },
        );

        let schema = Schema {
            imports: BTreeMap::new(),
            declarations,
        };

        let expected = "\
            choice Bar {\n\
            \x20 x: Bool = 0\n\
            \x20 asymmetric y: F64 = 1\n\
            }\n\
            \n\
            struct Foo {\n\
            \x20 x: Bool = 0\n\
            \x20 optional y: U64 = 1\n\
            }\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn schema_imports_and_declarations_display() {
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

        let foo_fields = vec![
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Optional,
                name: "y".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::U64,
                },
                index: 1,
            },
        ];

        let bar_fields = vec![
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                rule: Rule::Asymmetric,
                name: "y".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::F64,
                },
                index: 1,
            },
        ];

        let mut declarations = BTreeMap::new();

        declarations.insert(
            "Foo".into(),
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                variant: DeclarationVariant::Struct(foo_fields),
            },
        );

        declarations.insert(
            "Bar".into(),
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                variant: DeclarationVariant::Choice(bar_fields),
            },
        );

        let schema = Schema {
            imports,
            declarations,
        };

        let expected = "\
            import 'foo.t'\n\
            import 'bar.t' as qux\n\
            \n\
            choice Bar {\n\
            \x20 x: Bool = 0\n\
            \x20 asymmetric y: F64 = 1\n\
            }\n\
            \n\
            struct Foo {\n\
            \x20 x: Bool = 0\n\
            \x20 optional y: U64 = 1\n\
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
            components: vec!["foo".into()],
        };

        let expected = "foo";

        assert_eq!(namespace.to_string(), expected);
    }

    #[test]
    fn namespace_display_multiple() {
        let namespace = Namespace {
            components: vec!["foo".into(), "bar".into()],
        };

        let expected = "foo.bar";

        assert_eq!(namespace.to_string(), expected);
    }
}
