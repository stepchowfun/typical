use crate::{
    naming_conventions::{pascal_case, snake_case},
    schema,
    schema::relativize_namespace,
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    iter::{empty, once},
    path::PathBuf,
};

// The string to be used for each indentation level.
const INDENTATION: &str = "    ";

// Any generated types will derive these traits.
const TRAITS_TO_DERIVE: &[&str] = &["Clone", "Debug"];

// This struct represents a tree of schemas organized in a module hierarchy.
#[derive(Clone, Debug)]
struct Module {
    children: BTreeMap<String, Module>,
    schema: schema::Schema,
}

// Insert a schema into a module.
fn insert_schema(module: &mut Module, namespace: &schema::Namespace, schema: schema::Schema) {
    let mut iter = namespace.components.iter();

    if let Some(head) = iter.next() {
        if let Some(child) = module.children.get_mut(head) {
            insert_schema(
                child,
                &schema::Namespace {
                    components: iter.cloned().collect(),
                },
                schema,
            );
        } else {
            let mut child = Module {
                children: BTreeMap::new(),
                schema: schema::Schema {
                    imports: vec![],
                    declarations: vec![],
                },
            };

            insert_schema(
                &mut child,
                &schema::Namespace {
                    components: iter.cloned().collect(),
                },
                schema,
            );

            module.children.insert(head.to_owned(), child);
        }
    } else {
        module.schema = schema;
    }
}

// Generate Rust code from a schema and its transitive dependencies.
pub fn generate(schemas: BTreeMap<schema::Namespace, (schema::Schema, PathBuf, String)>) -> String {
    // Construct a tree of modules and schemas. We start with an empty tree.
    let mut tree = Module {
        children: BTreeMap::new(),
        schema: schema::Schema {
            imports: vec![],
            declarations: vec![],
        },
    };

    // Populate the tree with all the schemas.
    for (namespace, (schema, _, _)) in schemas {
        insert_schema(&mut tree, &namespace, schema);
    }

    // Render the code.
    render_module_contents(
        &schema::Namespace { components: vec![] },
        &tree.children,
        &tree.schema,
        0,
    )
}

// Render a module, including a trailing line break.
fn render_module(
    namespace: &schema::Namespace,
    name: &str,
    module: &Module,
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    let mut new_namespace = namespace.clone();
    new_namespace.components.push(name.to_owned());

    format!(
        "{}pub mod r#{} {{\n{}{}}}\n",
        indentation_str,
        snake_case(name),
        render_module_contents(
            &new_namespace,
            &module.children,
            &module.schema,
            indentation + 1,
        ),
        indentation_str,
    )
}

// Render the contents of a module, including a trailing line break if there was anything to render.
fn render_module_contents(
    namespace: &schema::Namespace,
    children: &BTreeMap<String, Module>,
    schema: &schema::Schema,
    indentation: u64,
) -> String {
    {
        let rendered_schema = render_schema(namespace, schema, indentation);

        if rendered_schema.is_empty() {
            Box::new(empty()) as Box<dyn Iterator<Item = String>>
        } else {
            Box::new(once(rendered_schema)) as Box<dyn Iterator<Item = String>>
        }
    }
    .chain(
        children
            .iter()
            .map(|(name, child)| render_module(namespace, name, child, indentation)),
    )
    .collect::<Vec<_>>()
    .join("\n")
}

// Render a schema, including a trailing line break if there was anything to render.
fn render_schema(
    namespace: &schema::Namespace,
    schema: &schema::Schema,
    indentation: u64,
) -> String {
    // Construct a map from import name to namespace.
    let mut imports = BTreeMap::new();
    for import in &schema.imports {
        // The unwrap is safe due to [ref:namespace_populated].
        imports.insert(import.name.clone(), import.namespace.clone().unwrap());
    }

    // Combine the results from rendering each declaration.
    schema
        .declarations
        .iter()
        .map(|declaration| match &declaration.variant {
            schema::DeclarationVariant::Struct(name, fields) => {
                render_struct(&imports, namespace, name, fields, indentation)
            }
            schema::DeclarationVariant::Choice(name, fields) => {
                render_choice(&imports, namespace, name, fields, indentation)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Copy, Clone, Debug)]
pub enum Flavor {
    In,
    Out,
    InToOut,
}

pub const FLAVORS: &[Flavor] = &[Flavor::In, Flavor::Out, Flavor::InToOut];

impl Display for Flavor {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::In => {
                write!(f, "In")?;
                Ok(())
            }
            Self::Out => {
                write!(f, "Out")?;
                Ok(())
            }
            Self::InToOut => {
                write!(f, "InToOut")?;
                Ok(())
            }
        }
    }
}

// Render a struct, including a trailing line break.
#[allow(clippy::too_many_lines)]
fn render_struct(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &str,
    fields: &[schema::Field],
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    let formatted_name = pascal_case(name);

    FLAVORS
        .iter()
        .map(|flavor| {
            format!(
                "\
                    {}#[allow(dead_code)]\n\
                    {}\
                    {}pub struct r#{}{} {{\n\
                    {}\
                    {}}}\n\
                ",
                indentation_str,
                if matches!(flavor, Flavor::InToOut) {
                    "".to_owned()
                } else {
                    format!(
                        "{}#[derive({})]\n",
                        indentation_str,
                        TRAITS_TO_DERIVE.join(", "),
                    )
                },
                indentation_str,
                formatted_name,
                flavor,
                fields
                    .iter()
                    .map(|field| {
                        render_struct_field(imports, namespace, field, *flavor, indentation + 1)
                    })
                    .collect::<Vec<_>>()
                    .join(""),
                indentation_str,
            )
        })
        .chain(once(format!(
            "\
                {}impl From<self::r#{}{}> for self::r#{}{} {{\n\
                {}{}fn from({}out: self::r#{}{}) -> Self {{\n\
                {}{}{}self::r#{}{} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            formatted_name,
            Flavor::Out,
            formatted_name,
            Flavor::In,
            indentation_str,
            INDENTATION,
            if fields.iter().any(|field| !field.restricted) {
                ""
            } else {
                "_"
            },
            formatted_name,
            Flavor::Out,
            indentation_str,
            INDENTATION,
            INDENTATION,
            formatted_name,
            Flavor::In,
            fields
                .iter()
                .filter_map(|field| {
                    if field.restricted {
                        None
                    } else {
                        let formatted_field_name = snake_case(&field.name);

                        Some(format!(
                            "\
                                {}{}{}{}r#{}: out.r#{}.into(),\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            formatted_field_name,
                        ))
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
            indentation_str,
            INDENTATION,
            INDENTATION,
            indentation_str,
            INDENTATION,
            indentation_str,
        )))
        .chain(once(format!(
            "\
                {}impl From<(self::r#{}{}, self::r#{}{})> for self::r#{}{} {{\n\
                {}{}fn from((r#{}in, r#{}in_to_out): (self::r#{}{}, self::r#{}{})) -> Self {{\n\
                {}{}{}self::r#{}{} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            formatted_name,
            Flavor::In,
            formatted_name,
            Flavor::InToOut,
            formatted_name,
            Flavor::Out,
            indentation_str,
            INDENTATION,
            if fields.is_empty() { "_" } else { "" },
            if fields.is_empty() { "_" } else { "" },
            formatted_name,
            Flavor::In,
            formatted_name,
            Flavor::InToOut,
            indentation_str,
            INDENTATION,
            INDENTATION,
            formatted_name,
            Flavor::Out,
            fields
                .iter()
                .map(|field| {
                    let formatted_field_name = snake_case(&field.name);

                    if field.restricted {
                        format!(
                            "\
                                {}{}{}{}r#{}: in_to_out.r#{},\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            formatted_field_name,
                        )
                    } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                        format!(
                            "\
                                {}{}{}{}r#{}: (r#in.r#{}, in_to_out.r#{}).into(),\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            formatted_field_name,
                            formatted_field_name,
                        )
                    } else {
                        format!(
                            "\
                                {}{}{}{}r#{}: r#in.r#{},\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            formatted_field_name,
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
            indentation_str,
            INDENTATION,
            INDENTATION,
            indentation_str,
            INDENTATION,
            indentation_str,
        )))
        .collect::<Vec<_>>()
        .join("\n")
}

// Render a choice, including a trailing line break.
#[allow(clippy::too_many_lines)]
fn render_choice(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &str,
    fields: &[schema::Field],
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    let formatted_name = pascal_case(name);

    FLAVORS
        .iter()
        .map(|flavor| {
            format!(
                "\
                    {}#[allow(dead_code)]\n\
                    {}\
                    {}pub {} r#{}{} {{\n\
                    {}\
                    {}}}\n\
                ",
                indentation_str,
                if matches!(flavor, Flavor::InToOut) {
                    "".to_owned()
                } else {
                    format!(
                        "{}#[derive({})]\n",
                        indentation_str,
                        TRAITS_TO_DERIVE.join(", "),
                    )
                },
                indentation_str,
                if let Flavor::InToOut = flavor {
                    "struct"
                } else {
                    "enum"
                },
                formatted_name,
                flavor,
                fields
                    .iter()
                    .map(|field| {
                        render_choice_field(
                            imports,
                            namespace,
                            name,
                            field,
                            *flavor,
                            indentation + 1,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(""),
                indentation_str,
            )
        })
        .chain(once(format!(
            "\
                {}impl From<self::r#{}{}> for self::r#{}{} {{\n\
                {}{}fn from(out: self::r#{}{}) -> Self {{\n\
                {}{}{}match out {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            formatted_name,
            Flavor::Out,
            formatted_name,
            Flavor::In,
            indentation_str,
            INDENTATION,
            formatted_name,
            Flavor::Out,
            indentation_str,
            INDENTATION,
            INDENTATION,
            fields
                .iter()
                .filter_map(|field| {
                    if field.restricted {
                        None
                    } else {
                        let formatted_field_name = pascal_case(&field.name);

                        Some(format!(
                            "\
                                {}{}{}{}self::{}{}::r#{}(payload) => \
                                    self::{}{}::r#{}(payload.into()),\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_name,
                            Flavor::Out,
                            formatted_field_name,
                            formatted_name,
                            Flavor::In,
                            formatted_field_name,
                        ))
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
            indentation_str,
            INDENTATION,
            INDENTATION,
            indentation_str,
            INDENTATION,
            indentation_str,
        )))
        .chain(once(format!(
            "\
                {}impl From<(self::r#{}{}, self::r#{}{})> for self::r#{}{} {{\n\
                {}{}fn from((r#{}in, r#{}in_to_out): (self::r#{}{}, self::r#{}{})) -> Self {{\n\
                {}{}{}match r#in {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            formatted_name,
            Flavor::In,
            formatted_name,
            Flavor::InToOut,
            formatted_name,
            Flavor::Out,
            indentation_str,
            INDENTATION,
            if fields.is_empty() { "_" } else { "" },
            if fields.is_empty() { "_" } else { "" },
            formatted_name,
            Flavor::In,
            formatted_name,
            Flavor::InToOut,
            indentation_str,
            INDENTATION,
            INDENTATION,
            fields
                .iter()
                .map(|field| {
                    let formatted_field_name = pascal_case(&field.name);

                    if field.restricted {
                        format!(
                            "\
                                {}{}{}{}self::{}{}::r#{}(payload) => (r#in_to_out.r#{})(payload),\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_name,
                            Flavor::In,
                            formatted_field_name,
                            snake_case(&field.name),
                        )
                    } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                        format!(
                            "\
                                {}{}{}{}self::{}{}::r#{}(payload) => \
                                    self::{}{}::r#{}((payload, r#in_to_out.r#{}).into()),\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_name,
                            Flavor::In,
                            formatted_field_name,
                            formatted_name,
                            Flavor::Out,
                            formatted_field_name,
                            snake_case(&field.name),
                        )
                    } else {
                        format!(
                            "\
                                {}{}{}{}self::{}{}::r#{}(payload) => self::{}{}::r#{}(payload),\n\
                            ",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_name,
                            Flavor::In,
                            formatted_field_name,
                            formatted_name,
                            Flavor::Out,
                            formatted_field_name,
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
            indentation_str,
            INDENTATION,
            INDENTATION,
            indentation_str,
            INDENTATION,
            indentation_str,
        )))
        .collect::<Vec<_>>()
        .join("\n")
}

// Render a field of a struct, including a trailing line break.
fn render_struct_field(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    field: &schema::Field,
    flavor: Flavor,
    indentation: u64,
) -> String {
    if match flavor {
        Flavor::In => !field.restricted,
        Flavor::Out => true,
        Flavor::InToOut => {
            if field.restricted {
                true
            } else {
                matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _))
            }
        }
    } {
        let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

        format!(
            "{}r#{}: {},\n",
            indentation_str,
            snake_case(&field.name),
            render_type(
                imports,
                namespace,
                &field.r#type,
                if let Flavor::InToOut = flavor {
                    if field.restricted {
                        Flavor::Out
                    } else {
                        flavor
                    }
                } else {
                    flavor
                },
            ),
        )
    } else {
        "".to_owned()
    }
}

// Render a field of a choice, including a trailing line break.
fn render_choice_field(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    choice_name: &str,
    field: &schema::Field,
    flavor: Flavor,
    indentation: u64,
) -> String {
    match flavor {
        Flavor::In | Flavor::Out => {
            if matches!(flavor, Flavor::In) || !field.restricted {
                let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

                format!(
                    "{}r#{}({}),\n",
                    indentation_str,
                    pascal_case(&field.name),
                    render_type(imports, namespace, &field.r#type, flavor),
                )
            } else {
                "".to_owned()
            }
        }
        Flavor::InToOut => {
            let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

            if field.restricted {
                format!(
                    "{}r#{}: Box<dyn FnOnce({}) -> r#{}{}>,\n",
                    indentation_str,
                    snake_case(&field.name),
                    render_type(imports, namespace, &field.r#type, Flavor::In),
                    choice_name,
                    Flavor::Out,
                )
            } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                format!(
                    "{}r#{}: {},\n",
                    indentation_str,
                    snake_case(&field.name),
                    render_type(imports, namespace, &field.r#type, Flavor::InToOut),
                )
            } else {
                "".to_owned()
            }
        }
    }
}

// Render a type with no line breaks.
fn render_type(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    r#type: &schema::Type,
    flavor: Flavor,
) -> String {
    match &r#type.variant {
        schema::TypeVariant::Bool => "bool".to_owned(),
        schema::TypeVariant::Custom(import, name) => {
            let type_namespace = schema::Namespace {
                components: import.as_ref().map_or_else(
                    || namespace.components.clone(),
                    |import| imports[import].components.clone(),
                ),
            };

            let (relative_type_namespace, ancestors) =
                relativize_namespace(&type_namespace, namespace);

            let mut components = vec![];

            for _ in 0..ancestors {
                components.push("super".to_owned());
            }

            components.extend(
                relative_type_namespace
                    .components
                    .iter()
                    .map(|component| format!("r#{}", snake_case(component))),
            );

            components.push(format!("r#{}{}", pascal_case(&name), flavor));

            components.join("::")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        generate_rust::generate, parser::parse, schema, tokenizer::tokenize, validator::validate,
    };
    use std::{collections::BTreeMap, path::Path};

    #[allow(clippy::too_many_lines)]
    #[test]
    fn generate_example() {
        let unit_namespace = schema::Namespace {
            components: vec!["basic".to_owned(), "unit".to_owned()],
        };
        let unit_path = Path::new("basic/unit.t").to_owned();
        let unit_contents = "\
            struct Unit {\n\
            }\n\
        "
        .to_owned();

        let void_namespace = schema::Namespace {
            components: vec!["basic".to_owned(), "void".to_owned()],
        };
        let void_path = Path::new("basic/void.t").to_owned();
        let void_contents = "\
            struct Void {\n\
            }\n\
        "
        .to_owned();

        let main_namespace = schema::Namespace {
            components: vec!["main".to_owned()],
        };
        let main_path = Path::new("main.t").to_owned();
        let main_contents = "\
            import 'basic/unit.t' as unit\n\
            import 'basic/void.t' as void\n\
            \n\
            struct Foo {\n\
              x: Bool = 0\n\
              y: restricted Bool = 1\n\
              z: void.Void = 2\n\
              w: restricted void.Void = 3\n\
              s: unit.Unit = 4\n\
              t: restricted unit.Unit = 5\n\
            }\n\
            \n\
            choice Bar {\n\
              x: Bool = 0\n\
              y: restricted Bool = 1\n\
              z: void.Void = 2\n\
              w: restricted void.Void = 3\n\
              s: unit.Unit = 4\n\
              t: restricted unit.Unit = 5\n\
            }
            \n\
            struct FooAndBar {\n\
              foo: Foo = 0\n\
              bar: Bar = 1\n\
            }\n\
            \n\
            choice FooOrBar {\n\
              foo: Foo = 0\n\
              bar: Bar = 1\n\
            }\n\
        "
        .to_owned();

        let unit_tokens = tokenize(&unit_path, &unit_contents).unwrap();
        let unit_schema = parse(&unit_path, &unit_contents, &unit_tokens).unwrap();

        let void_tokens = tokenize(&void_path, &void_contents).unwrap();
        let void_schema = parse(&void_path, &void_contents, &void_tokens).unwrap();

        let main_tokens = tokenize(&main_path, &main_contents).unwrap();
        let mut main_schema = parse(&main_path, &main_contents, &main_tokens).unwrap();
        main_schema.imports[0].namespace = Some(unit_namespace.clone());
        main_schema.imports[1].namespace = Some(void_namespace.clone());

        let mut schemas = BTreeMap::new();
        schemas.insert(unit_namespace, (unit_schema, unit_path, unit_contents));
        schemas.insert(void_namespace, (void_schema, void_path, void_contents));
        schemas.insert(main_namespace, (main_schema, main_path, main_contents));
        validate(&schemas).unwrap();

        assert_eq!(
            generate(schemas),
            "\
pub mod r#basic {
    pub mod r#unit {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct r#UnitIn {
        }

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct r#UnitOut {
        }

        #[allow(dead_code)]
        pub struct r#UnitInToOut {
        }

        impl From<self::r#UnitOut> for self::r#UnitIn {
            fn from(_out: self::r#UnitOut) -> Self {
                self::r#UnitIn {
                }
            }
        }

        impl From<(self::r#UnitIn, self::r#UnitInToOut)> for self::r#UnitOut {
            fn from((r#_in, r#_in_to_out): (self::r#UnitIn, self::r#UnitInToOut)) -> Self {
                self::r#UnitOut {
                }
            }
        }
    }

    pub mod r#void {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct r#VoidIn {
        }

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct r#VoidOut {
        }

        #[allow(dead_code)]
        pub struct r#VoidInToOut {
        }

        impl From<self::r#VoidOut> for self::r#VoidIn {
            fn from(_out: self::r#VoidOut) -> Self {
                self::r#VoidIn {
                }
            }
        }

        impl From<(self::r#VoidIn, self::r#VoidInToOut)> for self::r#VoidOut {
            fn from((r#_in, r#_in_to_out): (self::r#VoidIn, self::r#VoidInToOut)) -> Self {
                self::r#VoidOut {
                }
            }
        }
    }
}

pub mod r#main {
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct r#FooIn {
        r#x: bool,
        r#z: super::r#basic::r#void::r#VoidIn,
        r#s: super::r#basic::r#unit::r#UnitIn,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct r#FooOut {
        r#x: bool,
        r#y: bool,
        r#z: super::r#basic::r#void::r#VoidOut,
        r#w: super::r#basic::r#void::r#VoidOut,
        r#s: super::r#basic::r#unit::r#UnitOut,
        r#t: super::r#basic::r#unit::r#UnitOut,
    }

    #[allow(dead_code)]
    pub struct r#FooInToOut {
        r#y: bool,
        r#z: super::r#basic::r#void::r#VoidInToOut,
        r#w: super::r#basic::r#void::r#VoidOut,
        r#s: super::r#basic::r#unit::r#UnitInToOut,
        r#t: super::r#basic::r#unit::r#UnitOut,
    }

    impl From<self::r#FooOut> for self::r#FooIn {
        fn from(out: self::r#FooOut) -> Self {
            self::r#FooIn {
                r#x: out.r#x.into(),
                r#z: out.r#z.into(),
                r#s: out.r#s.into(),
            }
        }
    }

    impl From<(self::r#FooIn, self::r#FooInToOut)> for self::r#FooOut {
        fn from((r#in, r#in_to_out): (self::r#FooIn, self::r#FooInToOut)) -> Self {
            self::r#FooOut {
                r#x: r#in.r#x,
                r#y: in_to_out.r#y,
                r#z: (r#in.r#z, in_to_out.r#z).into(),
                r#w: in_to_out.r#w,
                r#s: (r#in.r#s, in_to_out.r#s).into(),
                r#t: in_to_out.r#t,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum r#BarIn {
        r#X(bool),
        r#Y(bool),
        r#Z(super::r#basic::r#void::r#VoidIn),
        r#W(super::r#basic::r#void::r#VoidIn),
        r#S(super::r#basic::r#unit::r#UnitIn),
        r#T(super::r#basic::r#unit::r#UnitIn),
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum r#BarOut {
        r#X(bool),
        r#Z(super::r#basic::r#void::r#VoidOut),
        r#S(super::r#basic::r#unit::r#UnitOut),
    }

    #[allow(dead_code)]
    pub struct r#BarInToOut {
        r#y: Box<dyn FnOnce(bool) -> r#BarOut>,
        r#z: super::r#basic::r#void::r#VoidInToOut,
        r#w: Box<dyn FnOnce(super::r#basic::r#void::r#VoidIn) -> r#BarOut>,
        r#s: super::r#basic::r#unit::r#UnitInToOut,
        r#t: Box<dyn FnOnce(super::r#basic::r#unit::r#UnitIn) -> r#BarOut>,
    }

    impl From<self::r#BarOut> for self::r#BarIn {
        fn from(out: self::r#BarOut) -> Self {
            match out {
                self::BarOut::r#X(payload) => self::BarIn::r#X(payload.into()),
                self::BarOut::r#Z(payload) => self::BarIn::r#Z(payload.into()),
                self::BarOut::r#S(payload) => self::BarIn::r#S(payload.into()),
            }
        }
    }

    impl From<(self::r#BarIn, self::r#BarInToOut)> for self::r#BarOut {
        fn from((r#in, r#in_to_out): (self::r#BarIn, self::r#BarInToOut)) -> Self {
            match r#in {
                self::BarIn::r#X(payload) => self::BarOut::r#X(payload),
                self::BarIn::r#Y(payload) => (r#in_to_out.r#y)(payload),
                self::BarIn::r#Z(payload) => self::BarOut::r#Z((payload, r#in_to_out.r#z).into()),
                self::BarIn::r#W(payload) => (r#in_to_out.r#w)(payload),
                self::BarIn::r#S(payload) => self::BarOut::r#S((payload, r#in_to_out.r#s).into()),
                self::BarIn::r#T(payload) => (r#in_to_out.r#t)(payload),
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct r#FooAndBarIn {
        r#foo: r#FooIn,
        r#bar: r#BarIn,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct r#FooAndBarOut {
        r#foo: r#FooOut,
        r#bar: r#BarOut,
    }

    #[allow(dead_code)]
    pub struct r#FooAndBarInToOut {
        r#foo: r#FooInToOut,
        r#bar: r#BarInToOut,
    }

    impl From<self::r#FooAndBarOut> for self::r#FooAndBarIn {
        fn from(out: self::r#FooAndBarOut) -> Self {
            self::r#FooAndBarIn {
                r#foo: out.r#foo.into(),
                r#bar: out.r#bar.into(),
            }
        }
    }

    impl From<(self::r#FooAndBarIn, self::r#FooAndBarInToOut)> for self::r#FooAndBarOut {
        fn from((r#in, r#in_to_out): (self::r#FooAndBarIn, self::r#FooAndBarInToOut)) -> Self {
            self::r#FooAndBarOut {
                r#foo: (r#in.r#foo, in_to_out.r#foo).into(),
                r#bar: (r#in.r#bar, in_to_out.r#bar).into(),
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum r#FooOrBarIn {
        r#Foo(r#FooIn),
        r#Bar(r#BarIn),
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum r#FooOrBarOut {
        r#Foo(r#FooOut),
        r#Bar(r#BarOut),
    }

    #[allow(dead_code)]
    pub struct r#FooOrBarInToOut {
        r#foo: r#FooInToOut,
        r#bar: r#BarInToOut,
    }

    impl From<self::r#FooOrBarOut> for self::r#FooOrBarIn {
        fn from(out: self::r#FooOrBarOut) -> Self {
            match out {
                self::FooOrBarOut::r#Foo(payload) => self::FooOrBarIn::r#Foo(payload.into()),
                self::FooOrBarOut::r#Bar(payload) => self::FooOrBarIn::r#Bar(payload.into()),
            }
        }
    }

    impl From<(self::r#FooOrBarIn, self::r#FooOrBarInToOut)> for self::r#FooOrBarOut {
        fn from((r#in, r#in_to_out): (self::r#FooOrBarIn, self::r#FooOrBarInToOut)) -> Self {
            match r#in {
                self::FooOrBarIn::r#Foo(payload) => \
                    self::FooOrBarOut::r#Foo((payload, r#in_to_out.r#foo).into()),
                self::FooOrBarIn::r#Bar(payload) => \
                    self::FooOrBarOut::r#Bar((payload, r#in_to_out.r#bar).into()),
            }
        }
    }
}
",
        );
    }
}
