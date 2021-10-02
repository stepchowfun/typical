use crate::{
    error::SourceRange,
    identifier::Identifier,
    token::{OPTIONAL_KEYWORD, UNSTABLE_KEYWORD},
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter, Write},
    path::PathBuf,
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
    pub cardinality: Cardinality,
    pub name: Identifier,
    pub r#type: Type,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub enum Cardinality {
    Optional,
    Required,
    Unstable,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub source_range: SourceRange,
    pub variant: TypeVariant,
}

#[derive(Clone, Debug)]
pub enum TypeVariant {
    Bool,
    Bytes,
    F64,
    S64,
    String,
    U64,
    Custom(Option<Identifier>, Identifier), // (import, name)
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
        writeln!(f, "import '{}' as {}", self.path.display(), name.original())
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
        match self.cardinality {
            Cardinality::Optional => {
                write!(f, "  {} ", OPTIONAL_KEYWORD)?;
            }
            Cardinality::Required => {
                write!(f, "  ")?;
            }
            Cardinality::Unstable => {
                write!(f, "  {} ", UNSTABLE_KEYWORD)?;
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
            Self::Bool => {
                write!(f, "bool")?;
            }
            Self::Bytes => {
                write!(f, "bytes")?;
            }
            Self::F64 => {
                write!(f, "f64")?;
            }
            Self::S64 => {
                write!(f, "s64")?;
            }
            Self::String => {
                write!(f, "String")?;
            }
            Self::U64 => {
                write!(f, "u64")?;
            }
            Self::Custom(import, name) => {
                if let Some(import) = import {
                    write!(f, "{}.{}", import.original(), name.original())?;
                } else {
                    write!(f, "{}", name.original())?;
                }
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
    use crate::{
        assert_same,
        error::SourceRange,
        schema::{
            relativize_namespace, Cardinality, Declaration, DeclarationVariant, Field, Import,
            Namespace, Schema, Type, TypeVariant,
        },
    };
    use std::{collections::BTreeMap, path::Path};

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
            "bar".into(),
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
            import 'bar.t' as bar\n\
            import 'foo.t' as foo\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[test]
    fn schema_declarations_only_display() {
        let foo_fields = vec![
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                cardinality: Cardinality::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                cardinality: Cardinality::Optional,
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
                cardinality: Cardinality::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                cardinality: Cardinality::Unstable,
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
            "foo".into(),
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                variant: DeclarationVariant::Struct(foo_fields),
            },
        );

        declarations.insert(
            "bar".into(),
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
            choice bar {\n\
            \x20 x: bool = 0\n\
            \x20 unstable y: f64 = 1\n\
            }\n\
            \n\
            struct foo {\n\
            \x20 x: bool = 0\n\
            \x20 optional y: u64 = 1\n\
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
            "bar".into(),
            Import {
                source_range: SourceRange { start: 0, end: 0 },
                path: Path::new("bar.t").to_owned(),
                namespace: None,
            },
        );

        let foo_fields = vec![
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                cardinality: Cardinality::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                cardinality: Cardinality::Optional,
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
                cardinality: Cardinality::Required,
                name: "x".into(),
                r#type: Type {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: TypeVariant::Bool,
                },
                index: 0,
            },
            Field {
                source_range: SourceRange { start: 0, end: 0 },
                cardinality: Cardinality::Unstable,
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
            "foo".into(),
            Declaration {
                source_range: SourceRange { start: 0, end: 0 },
                variant: DeclarationVariant::Struct(foo_fields),
            },
        );

        declarations.insert(
            "bar".into(),
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
            import 'bar.t' as bar\n\
            import 'foo.t' as foo\n\
            \n\
            choice bar {\n\
            \x20 x: bool = 0\n\
            \x20 unstable y: f64 = 1\n\
            }\n\
            \n\
            struct foo {\n\
            \x20 x: bool = 0\n\
            \x20 optional y: u64 = 1\n\
            }\n\
        ";

        assert_eq!(schema.to_string(), expected);
    }

    #[test]
    fn type_display_bool() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Bool,
        };

        let expected = "bool";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_bytes() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::Bytes,
        };

        let expected = "bytes";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_f64() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::F64,
        };

        let expected = "f64";

        assert_eq!(r#type.to_string(), expected);
    }

    #[test]
    fn type_display_s64() {
        let r#type = Type {
            source_range: SourceRange { start: 0, end: 0 },
            variant: TypeVariant::S64,
        };

        let expected = "s64";

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

        let expected = "u64";

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
