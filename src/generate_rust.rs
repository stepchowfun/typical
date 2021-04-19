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

// This is the full list of Rust 2018 keywords, both in use and reserved.
const RUST_KEYWORDS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

// These are some names that can appear in the generated code.
const IN_VARIABLE: &str = "in";
const OUT_VARIABLE: &str = "out";
const IN_TO_OUT_VARIABLE: &str = "in_to_out";
const PAYLOAD_VARIABLE: &str = "payload";

// This struct represents a tree of schemas organized in a module hierarchy.
#[derive(Clone, Debug)]
struct Module {
    children: BTreeMap<String, Module>,
    schema: schema::Schema,
}

// For each declaration, we emit these three "flavors" of it.
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

// Convert a name to a raw identifier if it happens to be a Rust keyword.
fn emit_identifier(name: &str) -> String {
    if RUST_KEYWORDS.iter().any(|keyword| name == *keyword) {
        format!("r#{}", name)
    } else {
        name.to_owned()
    }
}

// Append a flavor to a name. Convert the result to a raw identifier if it happens to be a Rust
// keyword.
fn emit_identifier_with_flavor(name: &str, flavor: Flavor) -> String {
    emit_identifier(&format!("{}{}", name, flavor))
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
        "{}pub mod {} {{\n{}{}}}\n",
        indentation_str,
        emit_identifier(&snake_case(name)),
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

    let formatted_name = emit_identifier(&pascal_case(name));

    #[allow(clippy::blocks_in_if_conditions)]
    FLAVORS
        .iter()
        .map(|flavor| {
            format!(
                "\
                    {}#[allow(dead_code)]\n\
                    {}\
                    {}pub struct {} {{\n\
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
                emit_identifier_with_flavor(&formatted_name, *flavor),
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
                {}impl From<self::{}> for self::{} {{\n\
                {}{}fn from({}: self::{}) -> Self {{\n\
                {}{}{}self::{} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            indentation_str,
            INDENTATION,
            emit_identifier(&if fields.iter().any(|field| !field.restricted) {
                OUT_VARIABLE.to_owned()
            } else {
                format!("_{}", OUT_VARIABLE)
            }),
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            indentation_str,
            INDENTATION,
            INDENTATION,
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            fields
                .iter()
                .filter_map(|field| {
                    if field.restricted {
                        None
                    } else {
                        let formatted_field_name = emit_identifier(&snake_case(&field.name));

                        Some(format!(
                            "{}{}{}{}{}: {}.{}.into(),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            emit_identifier(OUT_VARIABLE),
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
                {}impl From<(self::{}, self::{})> for self::{} {{\n\
                {}{}fn from(({}, {}): (self::{}, self::{})) -> Self {{\n\
                {}{}{}self::{} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            emit_identifier_with_flavor(&formatted_name, Flavor::InToOut),
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            indentation_str,
            INDENTATION,
            emit_identifier(&if fields.iter().any(|field| !field.restricted) {
                IN_VARIABLE.to_owned()
            } else {
                format!("_{}", IN_VARIABLE)
            }),
            emit_identifier(&if fields.iter().any(|field| {
                field.restricted
                    || matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _))
            }) {
                IN_TO_OUT_VARIABLE.to_owned()
            } else {
                format!("_{}", IN_TO_OUT_VARIABLE)
            }),
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            emit_identifier_with_flavor(&formatted_name, Flavor::InToOut),
            indentation_str,
            INDENTATION,
            INDENTATION,
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            fields
                .iter()
                .map(|field| {
                    let formatted_field_name = emit_identifier(&snake_case(&field.name));

                    if field.restricted {
                        format!(
                            "{}{}{}{}{}: {}.{},\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            emit_identifier(IN_TO_OUT_VARIABLE),
                            formatted_field_name,
                        )
                    } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                        format!(
                            "{}{}{}{}{}: ({}.{}, {}.{}).into(),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            emit_identifier(IN_VARIABLE),
                            formatted_field_name,
                            emit_identifier(IN_TO_OUT_VARIABLE),
                            formatted_field_name,
                        )
                    } else {
                        format!(
                            "{}{}{}{}{}: {}.{},\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            formatted_field_name,
                            emit_identifier(IN_VARIABLE),
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

    let formatted_name = emit_identifier(&pascal_case(name));

    #[allow(clippy::blocks_in_if_conditions)]
    FLAVORS
        .iter()
        .map(|flavor| {
            format!(
                "\
                    {}#[allow(dead_code)]\n\
                    {}\
                    {}pub {} {} {{\n\
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
                emit_identifier_with_flavor(&formatted_name, *flavor),
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
                {}impl From<self::{}> for self::{} {{\n\
                {}{}fn from({}: self::{}) -> Self {{\n\
                {}{}{}match {} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            indentation_str,
            INDENTATION,
            emit_identifier(OUT_VARIABLE),
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            indentation_str,
            INDENTATION,
            INDENTATION,
            emit_identifier(OUT_VARIABLE),
            fields
                .iter()
                .filter_map(|field| {
                    if field.restricted {
                        None
                    } else {
                        let formatted_field_name = emit_identifier(&pascal_case(&field.name));

                        Some(format!(
                            "{}{}{}{}self::{}::{}({}) => self::{}::{}({}.into()),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
                            emit_identifier_with_flavor(&formatted_name, Flavor::In),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
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
                {}impl From<(self::{}, self::{})> for self::{} {{\n\
                {}{}fn from(({}, {}): (self::{}, self::{})) -> Self {{\n\
                {}{}{}match {} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            emit_identifier_with_flavor(&formatted_name, Flavor::InToOut),
            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
            indentation_str,
            INDENTATION,
            emit_identifier(IN_VARIABLE),
            emit_identifier(&if fields.iter().any(|field| {
                field.restricted
                    || matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _))
            }) {
                IN_TO_OUT_VARIABLE.to_owned()
            } else {
                format!("_{}", IN_TO_OUT_VARIABLE)
            }),
            emit_identifier_with_flavor(&formatted_name, Flavor::In),
            emit_identifier_with_flavor(&formatted_name, Flavor::InToOut),
            indentation_str,
            INDENTATION,
            INDENTATION,
            emit_identifier(IN_VARIABLE),
            fields
                .iter()
                .map(|field| {
                    let formatted_field_name = emit_identifier(&pascal_case(&field.name));

                    if field.restricted {
                        format!(
                            "{}{}{}{}self::{}::{}({}) => ({}.{})({}),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            emit_identifier_with_flavor(&formatted_name, Flavor::In),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
                            emit_identifier(IN_TO_OUT_VARIABLE),
                            emit_identifier(&snake_case(&field.name)),
                            emit_identifier(PAYLOAD_VARIABLE),
                        )
                    } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                        format!(
                            "{}{}{}{}self::{}::{}({}) => self::{}::{}(({}, {}.{}).into()),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            emit_identifier_with_flavor(&formatted_name, Flavor::In),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
                            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
                            emit_identifier(IN_TO_OUT_VARIABLE),
                            emit_identifier(&snake_case(&field.name)),
                        )
                    } else {
                        format!(
                            "{}{}{}{}self::{}::{}({}) => self::{}::{}({}),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            emit_identifier_with_flavor(&formatted_name, Flavor::In),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
                            emit_identifier_with_flavor(&formatted_name, Flavor::Out),
                            formatted_field_name,
                            emit_identifier(PAYLOAD_VARIABLE),
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
            "{}{}: {},\n",
            indentation_str,
            emit_identifier(&snake_case(&field.name)),
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
                    "{}{}({}),\n",
                    indentation_str,
                    emit_identifier(&pascal_case(&field.name)),
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
                    "{}{}: Box<dyn FnOnce({}) -> {}{}>,\n",
                    indentation_str,
                    emit_identifier(&snake_case(&field.name)),
                    render_type(imports, namespace, &field.r#type, Flavor::In),
                    emit_identifier(choice_name),
                    Flavor::Out,
                )
            } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                format!(
                    "{}{}: {},\n",
                    indentation_str,
                    emit_identifier(&snake_case(&field.name)),
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
                    .map(|component| emit_identifier(&snake_case(component))),
            );

            components.push(emit_identifier_with_flavor(&pascal_case(&name), flavor));

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
pub mod basic {
    pub mod unit {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct UnitIn {
        }

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct UnitOut {
        }

        #[allow(dead_code)]
        pub struct UnitInToOut {
        }

        impl From<self::UnitOut> for self::UnitIn {
            fn from(_out: self::UnitOut) -> Self {
                self::UnitIn {
                }
            }
        }

        impl From<(self::UnitIn, self::UnitInToOut)> for self::UnitOut {
            fn from((_in, _in_to_out): (self::UnitIn, self::UnitInToOut)) -> Self {
                self::UnitOut {
                }
            }
        }
    }

    pub mod void {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct VoidIn {
        }

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct VoidOut {
        }

        #[allow(dead_code)]
        pub struct VoidInToOut {
        }

        impl From<self::VoidOut> for self::VoidIn {
            fn from(_out: self::VoidOut) -> Self {
                self::VoidIn {
                }
            }
        }

        impl From<(self::VoidIn, self::VoidInToOut)> for self::VoidOut {
            fn from((_in, _in_to_out): (self::VoidIn, self::VoidInToOut)) -> Self {
                self::VoidOut {
                }
            }
        }
    }
}

pub mod main {
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct FooIn {
        x: bool,
        z: super::basic::void::VoidIn,
        s: super::basic::unit::UnitIn,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct FooOut {
        x: bool,
        y: bool,
        z: super::basic::void::VoidOut,
        w: super::basic::void::VoidOut,
        s: super::basic::unit::UnitOut,
        t: super::basic::unit::UnitOut,
    }

    #[allow(dead_code)]
    pub struct FooInToOut {
        y: bool,
        z: super::basic::void::VoidInToOut,
        w: super::basic::void::VoidOut,
        s: super::basic::unit::UnitInToOut,
        t: super::basic::unit::UnitOut,
    }

    impl From<self::FooOut> for self::FooIn {
        fn from(out: self::FooOut) -> Self {
            self::FooIn {
                x: out.x.into(),
                z: out.z.into(),
                s: out.s.into(),
            }
        }
    }

    impl From<(self::FooIn, self::FooInToOut)> for self::FooOut {
        fn from((r#in, in_to_out): (self::FooIn, self::FooInToOut)) -> Self {
            self::FooOut {
                x: r#in.x,
                y: in_to_out.y,
                z: (r#in.z, in_to_out.z).into(),
                w: in_to_out.w,
                s: (r#in.s, in_to_out.s).into(),
                t: in_to_out.t,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum BarIn {
        X(bool),
        Y(bool),
        Z(super::basic::void::VoidIn),
        W(super::basic::void::VoidIn),
        S(super::basic::unit::UnitIn),
        T(super::basic::unit::UnitIn),
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum BarOut {
        X(bool),
        Z(super::basic::void::VoidOut),
        S(super::basic::unit::UnitOut),
    }

    #[allow(dead_code)]
    pub struct BarInToOut {
        y: Box<dyn FnOnce(bool) -> BarOut>,
        z: super::basic::void::VoidInToOut,
        w: Box<dyn FnOnce(super::basic::void::VoidIn) -> BarOut>,
        s: super::basic::unit::UnitInToOut,
        t: Box<dyn FnOnce(super::basic::unit::UnitIn) -> BarOut>,
    }

    impl From<self::BarOut> for self::BarIn {
        fn from(out: self::BarOut) -> Self {
            match out {
                self::BarOut::X(payload) => self::BarIn::X(payload.into()),
                self::BarOut::Z(payload) => self::BarIn::Z(payload.into()),
                self::BarOut::S(payload) => self::BarIn::S(payload.into()),
            }
        }
    }

    impl From<(self::BarIn, self::BarInToOut)> for self::BarOut {
        fn from((r#in, in_to_out): (self::BarIn, self::BarInToOut)) -> Self {
            match r#in {
                self::BarIn::X(payload) => self::BarOut::X(payload),
                self::BarIn::Y(payload) => (in_to_out.y)(payload),
                self::BarIn::Z(payload) => self::BarOut::Z((payload, in_to_out.z).into()),
                self::BarIn::W(payload) => (in_to_out.w)(payload),
                self::BarIn::S(payload) => self::BarOut::S((payload, in_to_out.s).into()),
                self::BarIn::T(payload) => (in_to_out.t)(payload),
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct FooAndBarIn {
        foo: FooIn,
        bar: BarIn,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct FooAndBarOut {
        foo: FooOut,
        bar: BarOut,
    }

    #[allow(dead_code)]
    pub struct FooAndBarInToOut {
        foo: FooInToOut,
        bar: BarInToOut,
    }

    impl From<self::FooAndBarOut> for self::FooAndBarIn {
        fn from(out: self::FooAndBarOut) -> Self {
            self::FooAndBarIn {
                foo: out.foo.into(),
                bar: out.bar.into(),
            }
        }
    }

    impl From<(self::FooAndBarIn, self::FooAndBarInToOut)> for self::FooAndBarOut {
        fn from((r#in, in_to_out): (self::FooAndBarIn, self::FooAndBarInToOut)) -> Self {
            self::FooAndBarOut {
                foo: (r#in.foo, in_to_out.foo).into(),
                bar: (r#in.bar, in_to_out.bar).into(),
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum FooOrBarIn {
        Foo(FooIn),
        Bar(BarIn),
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum FooOrBarOut {
        Foo(FooOut),
        Bar(BarOut),
    }

    #[allow(dead_code)]
    pub struct FooOrBarInToOut {
        foo: FooInToOut,
        bar: BarInToOut,
    }

    impl From<self::FooOrBarOut> for self::FooOrBarIn {
        fn from(out: self::FooOrBarOut) -> Self {
            match out {
                self::FooOrBarOut::Foo(payload) => self::FooOrBarIn::Foo(payload.into()),
                self::FooOrBarOut::Bar(payload) => self::FooOrBarIn::Bar(payload.into()),
            }
        }
    }

    impl From<(self::FooOrBarIn, self::FooOrBarInToOut)> for self::FooOrBarOut {
        fn from((r#in, in_to_out): (self::FooOrBarIn, self::FooOrBarInToOut)) -> Self {
            match r#in {
                self::FooOrBarIn::Foo(payload) => \
                    self::FooOrBarOut::Foo((payload, in_to_out.foo).into()),
                self::FooOrBarIn::Bar(payload) => \
                    self::FooOrBarOut::Bar((payload, in_to_out.bar).into()),
            }
        }
    }
}
",
        );
    }
}
