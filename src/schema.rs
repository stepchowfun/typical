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

#[derive(Clone, Debug)]
pub struct Import {
    pub source_range: SourceRange,
    pub original_path: PathBuf, // The literal path from the source file
    pub based_path: PathBuf,    // Relative to the base directory
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub source_range: SourceRange,
    pub variant: DeclarationVariant,
}

#[derive(Clone, Debug)]
pub enum DeclarationVariant {
    Struct(String, Vec<Field>), // (name, fields)
    Choice(String, Vec<Field>), // (name, fields)
}

#[derive(Clone, Debug)]
pub struct Field {
    pub source_range: SourceRange,
    pub name: String,
    pub restricted: bool,
    pub r#type: Type,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub source_range: SourceRange,
    pub import: Option<String>,
    pub name: String,
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

            write!(
                f,
                "import '{}' as {}",
                import.original_path.display(),
                import.name,
            )?;
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
                self.name,
                self.r#type,
                self.index,
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

#[cfg(test)]
mod tests {
    use crate::{
        error::SourceRange,
        schema::{Declaration, DeclarationVariant, Field, Import, Schema, Type},
    };
    use std::path::Path;

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
                            original_path: Path::new("qux.t").to_owned(),
                            based_path: Path::new("qux.t").to_owned(),
                            name: "qux".to_owned(),
                        },
                        Import {
                            source_range: SourceRange { start: 0, end: 0 },
                            original_path: Path::new("corge.t").to_owned(),
                            based_path: Path::new("corge.t").to_owned(),
                            name: "corge".to_owned(),
                        },
                    ],
                    declarations: vec![],
                },
            ),
            "\
            import 'qux.t' as qux\n\
            import 'corge.t' as corge\
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
                                        name: "foo".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 42,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "bar".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 43,
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
                                        name: "foo".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 42,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "bar".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 43,
                                    },
                                ],
                            ),
                        },
                    ],
                },
            ),
            "\
            struct Foo {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
            }\n\
            \n\
            choice Bar {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
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
                            original_path: Path::new("qux.t").to_owned(),
                            based_path: Path::new("qux.t").to_owned(),
                            name: "qux".to_owned(),
                        },
                        Import {
                            source_range: SourceRange { start: 0, end: 0 },
                            original_path: Path::new("corge.t").to_owned(),
                            based_path: Path::new("corge.t").to_owned(),
                            name: "corge".to_owned(),
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
                                        name: "foo".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 42,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "bar".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 43,
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
                                        name: "foo".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "Int".to_owned(),
                                        },
                                        index: 42,
                                    },
                                    Field {
                                        source_range: SourceRange { start: 0, end: 0 },
                                        name: "bar".to_owned(),
                                        restricted: false,
                                        r#type: Type {
                                            source_range: SourceRange { start: 0, end: 0 },
                                            import: None,
                                            name: "String".to_owned(),
                                        },
                                        index: 43,
                                    },
                                ],
                            ),
                        },
                    ],
                },
            ),
            "\
            import 'qux.t' as qux\n\
            import 'corge.t' as corge\n\
            \n\
            struct Foo {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
            }\n\
            \n\
            choice Bar {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
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
                                name: "foo".to_owned(),
                                restricted: false,
                                r#type: Type {
                                    source_range: SourceRange { start: 0, end: 0 },
                                    import: None,
                                    name: "Int".to_owned(),
                                },
                                index: 42,
                            },
                            Field {
                                source_range: SourceRange { start: 0, end: 0 },
                                name: "bar".to_owned(),
                                restricted: false,
                                r#type: Type {
                                    source_range: SourceRange { start: 0, end: 0 },
                                    import: None,
                                    name: "String".to_owned(),
                                },
                                index: 43,
                            },
                        ],
                    ),
                },
            ),
            "\
            struct Foo {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
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
                            name: "foo".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "Int".to_owned(),
                            },
                            index: 42,
                        },
                        Field {
                            source_range: SourceRange { start: 0, end: 0 },
                            name: "bar".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "String".to_owned(),
                            },
                            index: 43,
                        },
                    ],
                ),
            ),
            "\
            struct Foo {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
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
                            name: "foo".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "Int".to_owned(),
                            },
                            index: 42,
                        },
                        Field {
                            source_range: SourceRange { start: 0, end: 0 },
                            name: "bar".to_owned(),
                            restricted: false,
                            r#type: Type {
                                source_range: SourceRange { start: 0, end: 0 },
                                import: None,
                                name: "String".to_owned(),
                            },
                            index: 43,
                        },
                    ],
                ),
            ),
            "\
            choice Foo {\n\
            \x20 foo: Int = 42\n\
            \x20 bar: String = 43\n\
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
                    name: "foo".to_owned(),
                    restricted: false,
                    r#type: Type {
                        source_range: SourceRange { start: 0, end: 0 },
                        import: None,
                        name: "Int".to_owned(),
                    },
                    index: 42,
                },
            ),
            "  foo: Int = 42",
        );
    }

    #[test]
    fn field_display_restricted() {
        assert_eq!(
            format!(
                "{}",
                Field {
                    source_range: SourceRange { start: 0, end: 0 },
                    name: "foo".to_owned(),
                    restricted: true,
                    r#type: Type {
                        source_range: SourceRange { start: 0, end: 0 },
                        import: None,
                        name: "Int".to_owned(),
                    },
                    index: 42,
                },
            ),
            "  foo: restricted Int = 42",
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
}
