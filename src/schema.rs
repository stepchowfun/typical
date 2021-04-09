use std::{
    fmt::{Display, Formatter},
    path::Path,
};

#[derive(Clone, Debug)]
pub struct Schema<'a> {
    pub path: &'a Path,
    pub declarations: Vec<Declaration<'a>>,
}

#[derive(Clone, Debug)]
pub struct Declaration<'a> {
    pub source_range: (usize, usize), // Inclusive on the left and exclusive on the right
    pub variant: DeclarationVariant<'a>,
}

#[derive(Clone, Debug)]
pub enum DeclarationVariant<'a> {
    Struct(&'a str, Vec<Field<'a>>), // (name, fields)
    Choice(&'a str, Vec<Field<'a>>), // (name, fields)
}

#[derive(Clone, Debug)]
pub struct Field<'a> {
    pub source_range: (usize, usize), // Inclusive on the left and exclusive on the right
    pub name: &'a str,
    pub restricted: bool,
    pub r#type: Type<'a>,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Type<'a> {
    pub source_range: (usize, usize), // Inclusive on the left and exclusive on the right
    pub prefix: Option<&'a str>,
    pub name: &'a str,
}

impl<'a> Display for Schema<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut first = true;
        for declaration in &self.declarations {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }
            write!(f, "{}", declaration)?;
        }
        Ok(())
    }
}

impl<'a> Display for Declaration<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant)?;
        Ok(())
    }
}

impl<'a> Display for DeclarationVariant<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Struct(name, fields) => {
                writeln!(f, "struct {} {{", name)?;
                for field in fields.iter() {
                    write!(f, "{}", field)?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
            Self::Choice(name, fields) => {
                writeln!(f, "choice {} {{", name)?;
                for field in fields.iter() {
                    write!(f, "{}", field)?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
        }
    }
}

impl<'a> Display for Field<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.restricted {
            writeln!(
                f,
                "  {}: restricted {} = {}",
                self.name, self.r#type, self.index,
            )?;
        } else {
            writeln!(f, "  {}: {} = {}", self.name, self.r#type, self.index)?;
        }

        Ok(())
    }
}

impl<'a> Display for Type<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(prefix) = self.prefix {
            write!(f, "{}.{}", prefix, self.name)?;
        } else {
            write!(f, "{}", self.name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::{Declaration, DeclarationVariant, Field, Schema, Type};
    use std::path::Path;

    #[test]
    fn schema_display() {
        assert_eq!(
            format!(
                "{}",
                Schema {
                    path: Path::new("foo.t"),
                    declarations: vec![
                        Declaration {
                            source_range: (0, 0),
                            variant: DeclarationVariant::Struct(
                                "Foo",
                                vec![
                                    Field {
                                        source_range: (0, 0),
                                        name: "foo",
                                        restricted: false,
                                        r#type: Type {
                                            source_range: (0, 0),
                                            prefix: None,
                                            name: "Int",
                                        },
                                        index: 42,
                                    },
                                    Field {
                                        source_range: (0, 0),
                                        name: "bar",
                                        restricted: false,
                                        r#type: Type {
                                            source_range: (0, 0),
                                            prefix: None,
                                            name: "String",
                                        },
                                        index: 43,
                                    },
                                ],
                            ),
                        },
                        Declaration {
                            source_range: (0, 0),
                            variant: DeclarationVariant::Choice(
                                "Bar",
                                vec![
                                    Field {
                                        source_range: (0, 0),
                                        name: "foo",
                                        restricted: false,
                                        r#type: Type {
                                            source_range: (0, 0),
                                            prefix: None,
                                            name: "Int",
                                        },
                                        index: 42,
                                    },
                                    Field {
                                        source_range: (0, 0),
                                        name: "bar",
                                        restricted: false,
                                        r#type: Type {
                                            source_range: (0, 0),
                                            prefix: None,
                                            name: "String",
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
            }\n\
            ",
        );
    }

    #[test]
    fn declaration_display() {
        assert_eq!(
            format!(
                "{}",
                Declaration {
                    source_range: (0, 0),
                    variant: DeclarationVariant::Struct(
                        "Foo",
                        vec![
                            Field {
                                source_range: (0, 0),
                                name: "foo",
                                restricted: false,
                                r#type: Type {
                                    source_range: (0, 0),
                                    prefix: None,
                                    name: "Int"
                                },
                                index: 42,
                            },
                            Field {
                                source_range: (0, 0),
                                name: "bar",
                                restricted: false,
                                r#type: Type {
                                    source_range: (0, 0),
                                    prefix: None,
                                    name: "String"
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
            }\n\
            ",
        );
    }

    #[test]
    fn declaration_variant_struct_display() {
        assert_eq!(
            format!(
                "{}",
                DeclarationVariant::Struct(
                    "Foo",
                    vec![
                        Field {
                            source_range: (0, 0),
                            name: "foo",
                            restricted: false,
                            r#type: Type {
                                source_range: (0, 0),
                                prefix: None,
                                name: "Int"
                            },
                            index: 42,
                        },
                        Field {
                            source_range: (0, 0),
                            name: "bar",
                            restricted: false,
                            r#type: Type {
                                source_range: (0, 0),
                                prefix: None,
                                name: "String"
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
            }\n\
            ",
        );
    }

    #[test]
    fn declaration_variant_choice_display() {
        assert_eq!(
            format!(
                "{}",
                DeclarationVariant::Choice(
                    "Foo",
                    vec![
                        Field {
                            source_range: (0, 0),
                            name: "foo",
                            restricted: false,
                            r#type: Type {
                                source_range: (0, 0),
                                prefix: None,
                                name: "Int"
                            },
                            index: 42,
                        },
                        Field {
                            source_range: (0, 0),
                            name: "bar",
                            restricted: false,
                            r#type: Type {
                                source_range: (0, 0),
                                prefix: None,
                                name: "String"
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
            }\n\
            ",
        );
    }

    #[test]
    fn field_display_non_restricted() {
        assert_eq!(
            format!(
                "{}",
                Field {
                    source_range: (0, 0),
                    name: "foo",
                    restricted: false,
                    r#type: Type {
                        source_range: (0, 0),
                        prefix: None,
                        name: "Int",
                    },
                    index: 42,
                },
            ),
            "  foo: Int = 42\n",
        );
    }

    #[test]
    fn field_display_restricted() {
        assert_eq!(
            format!(
                "{}",
                Field {
                    source_range: (0, 0),
                    name: "foo",
                    restricted: true,
                    r#type: Type {
                        source_range: (0, 0),
                        prefix: None,
                        name: "Int",
                    },
                    index: 42,
                },
            ),
            "  foo: restricted Int = 42\n",
        );
    }

    #[test]
    fn type_display_no_prefix() {
        assert_eq!(
            format!(
                "{}",
                Type {
                    source_range: (0, 0),
                    prefix: None,
                    name: "Int",
                },
            ),
            "Int",
        );
    }

    #[test]
    fn type_display_prefix() {
        assert_eq!(
            format!(
                "{}",
                Type {
                    source_range: (0, 0),
                    prefix: Some("foo"),
                    name: "Int",
                },
            ),
            "foo.Int",
        );
    }
}
