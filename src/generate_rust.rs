use crate::{identifier::Identifier, schema, schema::relativize_namespace};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    iter::{empty, once},
    path::PathBuf,
};

// The string to be used for each indentation level.
const INDENTATION: &str = "    ";

// The generated types will derive these traits.
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
    children: BTreeMap<Identifier, Module>,
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

// This enum represents a case convention for the `render_identifier` function below.
#[derive(Copy, Clone, Debug)]
pub enum CaseConvention {
    Pascal,
    Snake,
}

use CaseConvention::{Pascal, Snake};

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
                    imports: BTreeMap::new(),
                    declarations: BTreeMap::new(),
                },
            };

            insert_schema(
                &mut child,
                &schema::Namespace {
                    components: iter.cloned().collect(),
                },
                schema,
            );

            module.children.insert(head.clone(), child);
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
            imports: BTreeMap::new(),
            declarations: BTreeMap::new(),
        },
    };

    // Populate the tree with all the schemas.
    for (namespace, (schema, _, _)) in schemas {
        insert_schema(&mut tree, &namespace, schema);
    }

    // Render the code.
    let module_contents = render_module_contents(
        &schema::Namespace { components: vec![] },
        &tree.children,
        &tree.schema,
        0,
    );

    format!(
        "#![allow(clippy::all, clippy::pedantic, warnings)]\n{}{}",
        if module_contents.is_empty() { "" } else { "\n" },
        module_contents,
    )
}

// Render a module, including a trailing line break.
fn render_module(
    namespace: &schema::Namespace,
    name: &Identifier,
    module: &Module,
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    let mut new_namespace = namespace.clone();
    new_namespace.components.push(name.clone());

    format!(
        "{}pub mod {} {{\n{}{}}}\n",
        indentation_str,
        render_identifier(name, Snake, None),
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
    children: &BTreeMap<Identifier, Module>,
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
    for (name, import) in &schema.imports {
        // The unwrap is safe due to [ref:namespace_populated].
        imports.insert(name.clone(), import.namespace.clone().unwrap());
    }

    // Combine the results from rendering each declaration.
    schema
        .declarations
        .iter()
        .map(|(name, declaration)| match &declaration.variant {
            schema::DeclarationVariant::Struct(fields) => {
                render_struct(&imports, namespace, name, fields, indentation)
            }
            schema::DeclarationVariant::Choice(fields) => {
                render_choice(&imports, namespace, name, fields, indentation)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Render a struct, including a trailing line break.
#[allow(clippy::too_many_lines)]
fn render_struct(
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    fields: &BTreeMap<Identifier, schema::Field>,
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    FLAVORS
        .iter()
        .map(|flavor| {
            format!(
                "\
                    {}\
                    {}pub struct {} {{\n\
                    {}\
                    {}}}\n\
                ",
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
                render_identifier(name, Pascal, Some(*flavor)),
                fields
                    .iter()
                    .map(|(field_name, field)| {
                        render_struct_field(
                            imports,
                            namespace,
                            field_name,
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
                {}{}{}self::{} {{\n\
                {}\
                {}{}{}}}\n\
                {}{}}}\n\
                {}}}\n\
            ",
            indentation_str,
            render_identifier(name, Pascal, Some(Flavor::Out)),
            render_identifier(name, Pascal, Some(Flavor::In)),
            indentation_str,
            INDENTATION,
            render_identifier(&(OUT_VARIABLE.into()), Snake, None),
            render_identifier(name, Pascal, Some(Flavor::Out)),
            indentation_str,
            INDENTATION,
            INDENTATION,
            render_identifier(name, Pascal, Some(Flavor::In)),
            fields
                .iter()
                .filter_map(|(field_name, field)| {
                    if field.transitional {
                        None
                    } else {
                        Some(format!(
                            "{}{}{}{}{}: {}.{}.into(),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(field_name, Snake, None),
                            render_identifier(&(OUT_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
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
            render_identifier(name, Pascal, Some(Flavor::In)),
            render_identifier(name, Pascal, Some(Flavor::InToOut)),
            render_identifier(name, Pascal, Some(Flavor::Out)),
            indentation_str,
            INDENTATION,
            render_identifier(&(IN_VARIABLE.into()), Snake, None),
            render_identifier(&(IN_TO_OUT_VARIABLE.into()), Snake, None),
            render_identifier(name, Pascal, Some(Flavor::In)),
            render_identifier(name, Pascal, Some(Flavor::InToOut)),
            indentation_str,
            INDENTATION,
            INDENTATION,
            render_identifier(name, Pascal, Some(Flavor::Out)),
            fields
                .iter()
                .map(|(field_name, field)| {
                    if field.transitional {
                        format!(
                            "{}{}{}{}{}: {}.{},\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(field_name, Snake, None),
                            render_identifier(&(IN_TO_OUT_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
                        )
                    } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                        format!(
                            "{}{}{}{}{}: ({}.{}, {}.{}).into(),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(field_name, Snake, None),
                            render_identifier(&(IN_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
                            render_identifier(&(IN_TO_OUT_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
                        )
                    } else {
                        format!(
                            "{}{}{}{}{}: {}.{},\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(field_name, Snake, None),
                            render_identifier(&(IN_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
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
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    fields: &BTreeMap<Identifier, schema::Field>,
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    FLAVORS
        .iter()
        .map(|flavor| {
            format!(
                "\
                    {}\
                    {}pub {} {} {{\n\
                    {}\
                    {}}}\n\
                ",
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
                render_identifier(name, Pascal, Some(*flavor)),
                fields
                    .iter()
                    .map(|(field_name, field)| {
                        render_choice_field(
                            imports,
                            namespace,
                            name,
                            field_name,
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
            render_identifier(name, Pascal, Some(Flavor::Out)),
            render_identifier(name, Pascal, Some(Flavor::In)),
            indentation_str,
            INDENTATION,
            render_identifier(&(OUT_VARIABLE.into()), Snake, None),
            render_identifier(name, Pascal, Some(Flavor::Out)),
            indentation_str,
            INDENTATION,
            INDENTATION,
            render_identifier(&(OUT_VARIABLE.into()), Snake, None),
            fields
                .iter()
                .filter_map(|(field_name, field)| {
                    if field.transitional {
                        None
                    } else {
                        Some(format!(
                            "{}{}{}{}self::{}::{}({}) => self::{}::{}({}.into()),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(name, Pascal, Some(Flavor::Out)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
                            render_identifier(name, Pascal, Some(Flavor::In)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
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
            render_identifier(name, Pascal, Some(Flavor::In)),
            render_identifier(name, Pascal, Some(Flavor::InToOut)),
            render_identifier(name, Pascal, Some(Flavor::Out)),
            indentation_str,
            INDENTATION,
            render_identifier(&(IN_VARIABLE.into()), Snake, None),
            render_identifier(&(IN_TO_OUT_VARIABLE.into()), Snake, None),
            render_identifier(name, Pascal, Some(Flavor::In)),
            render_identifier(name, Pascal, Some(Flavor::InToOut)),
            indentation_str,
            INDENTATION,
            INDENTATION,
            render_identifier(&(IN_VARIABLE.into()), Snake, None),
            fields
                .iter()
                .map(|(field_name, field)| {
                    if field.transitional {
                        format!(
                            "{}{}{}{}self::{}::{}({}) => ({}.{})({}),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(name, Pascal, Some(Flavor::In)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
                            render_identifier(&(IN_TO_OUT_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
                        )
                    } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                        format!(
                            "{}{}{}{}self::{}::{}({}) => self::{}::{}(({}, {}.{}).into()),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(name, Pascal, Some(Flavor::In)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
                            render_identifier(name, Pascal, Some(Flavor::Out)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
                            render_identifier(&(IN_TO_OUT_VARIABLE.into()), Snake, None),
                            render_identifier(field_name, Snake, None),
                        )
                    } else {
                        format!(
                            "{}{}{}{}self::{}::{}({}) => self::{}::{}({}),\n",
                            indentation_str,
                            INDENTATION,
                            INDENTATION,
                            INDENTATION,
                            render_identifier(name, Pascal, Some(Flavor::In)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
                            render_identifier(name, Pascal, Some(Flavor::Out)),
                            render_identifier(field_name, Pascal, None),
                            render_identifier(&(PAYLOAD_VARIABLE.into()), Snake, None),
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
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    field: &schema::Field,
    flavor: Flavor,
    indentation: u64,
) -> String {
    if match flavor {
        Flavor::In => !field.transitional,
        Flavor::Out => true,
        Flavor::InToOut => {
            if field.transitional {
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
            render_identifier(name, Snake, None),
            render_type(
                imports,
                namespace,
                &field.r#type,
                if let Flavor::InToOut = flavor {
                    if field.transitional {
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
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    choice_name: &Identifier,
    name: &Identifier,
    field: &schema::Field,
    flavor: Flavor,
    indentation: u64,
) -> String {
    match flavor {
        Flavor::In | Flavor::Out => {
            if matches!(flavor, Flavor::In) || !field.transitional {
                let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

                format!(
                    "{}{}({}),\n",
                    indentation_str,
                    render_identifier(name, Pascal, None),
                    render_type(imports, namespace, &field.r#type, flavor),
                )
            } else {
                "".to_owned()
            }
        }
        Flavor::InToOut => {
            let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

            if field.transitional {
                format!(
                    "{}{}: Box<dyn FnOnce({}) -> {}{}>,\n",
                    indentation_str,
                    render_identifier(name, Snake, None),
                    render_type(imports, namespace, &field.r#type, Flavor::In),
                    render_identifier(choice_name, Pascal, None),
                    Flavor::Out,
                )
            } else if matches!(field.r#type.variant, schema::TypeVariant::Custom(_, _)) {
                format!(
                    "{}{}: {},\n",
                    indentation_str,
                    render_identifier(name, Snake, None),
                    render_type(imports, namespace, &field.r#type, Flavor::InToOut),
                )
            } else {
                "".to_owned()
            }
        }
    }
}

// Render a type.
fn render_type(
    imports: &BTreeMap<Identifier, schema::Namespace>,
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
                    .map(|component| render_identifier(component, Snake, None)),
            );

            components.push(render_identifier(&name, Pascal, Some(flavor)));

            components.join("::")
        }
    }
}

// Render an identifier with an optional flavor suffix in a way that Rust will be happy with.
fn render_identifier(
    identifier: &Identifier,
    case: CaseConvention,
    flavor: Option<Flavor>,
) -> String {
    let converted_name = match case {
        CaseConvention::Pascal => identifier.pascal_case(),
        CaseConvention::Snake => identifier.snake_case(),
    };

    format!(
        "{}{}{}",
        if !converted_name.starts_with("r#")
            && RUST_KEYWORDS
                .iter()
                .any(|keyword| converted_name == *keyword)
        {
            "r#"
        } else {
            ""
        },
        converted_name,
        flavor.map_or("".to_owned(), |flavor| flavor.to_string()),
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        generate_rust::generate, parser::parse, schema, tokenizer::tokenize, validator::validate,
    };
    use std::{collections::BTreeMap, fs::read_to_string, path::Path};

    #[allow(clippy::too_many_lines)]
    #[test]
    fn generate_example() {
        let unit_namespace = schema::Namespace {
            components: vec!["basic".into(), "unit".into()],
        };
        let unit_path = Path::new("integration/types/basic/unit.t").to_owned();
        let unit_contents = read_to_string(&unit_path).unwrap();

        let void_namespace = schema::Namespace {
            components: vec!["basic".into(), "void".into()],
        };
        let void_path = Path::new("integration/types/basic/void.t").to_owned();
        let void_contents = read_to_string(&void_path).unwrap();

        let main_namespace = schema::Namespace {
            components: vec!["main".into()],
        };
        let main_path = Path::new("integration/types/main.t").to_owned();
        let main_contents = read_to_string(&main_path).unwrap();

        let unit_tokens = tokenize(&unit_path, &unit_contents).unwrap();
        let unit_schema = parse(&unit_path, &unit_contents, &unit_tokens).unwrap();

        let void_tokens = tokenize(&void_path, &void_contents).unwrap();
        let void_schema = parse(&void_path, &void_contents, &void_tokens).unwrap();

        let main_tokens = tokenize(&main_path, &main_contents).unwrap();
        let mut main_schema = parse(&main_path, &main_contents, &main_tokens).unwrap();
        main_schema
            .imports
            .get_mut(&"unit".into())
            .unwrap()
            .namespace = Some(unit_namespace.clone());
        main_schema
            .imports
            .get_mut(&"void".into())
            .unwrap()
            .namespace = Some(void_namespace.clone());

        let mut schemas = BTreeMap::new();
        schemas.insert(unit_namespace, (unit_schema, unit_path, unit_contents));
        schemas.insert(void_namespace, (void_schema, void_path, void_contents));
        schemas.insert(main_namespace, (main_schema, main_path, main_contents));
        validate(&schemas).unwrap();

        assert_eq!(
            generate(schemas),
            "\
#![allow(clippy::all, clippy::pedantic, warnings)]

pub mod basic {
    pub mod unit {
        #[derive(Clone, Debug)]
        pub struct UnitIn {
        }

        #[derive(Clone, Debug)]
        pub struct UnitOut {
        }

        pub struct UnitInToOut {
        }

        impl From<self::UnitOut> for self::UnitIn {
            fn from(out: self::UnitOut) -> Self {
                self::UnitIn {
                }
            }
        }

        impl From<(self::UnitIn, self::UnitInToOut)> for self::UnitOut {
            fn from((r#in, in_to_out): (self::UnitIn, self::UnitInToOut)) -> Self {
                self::UnitOut {
                }
            }
        }
    }

    pub mod void {
        #[derive(Clone, Debug)]
        pub enum VoidIn {
        }

        #[derive(Clone, Debug)]
        pub enum VoidOut {
        }

        pub struct VoidInToOut {
        }

        impl From<self::VoidOut> for self::VoidIn {
            fn from(out: self::VoidOut) -> Self {
                match out {
                }
            }
        }

        impl From<(self::VoidIn, self::VoidInToOut)> for self::VoidOut {
            fn from((r#in, in_to_out): (self::VoidIn, self::VoidInToOut)) -> Self {
                match r#in {
                }
            }
        }
    }
}

pub mod main {
    #[derive(Clone, Debug)]
    pub enum BarIn {
        S(super::basic::unit::UnitIn),
        T(super::basic::unit::UnitIn),
        W(super::basic::void::VoidIn),
        X(bool),
        Y(bool),
        Z(super::basic::void::VoidIn),
    }

    #[derive(Clone, Debug)]
    pub enum BarOut {
        S(super::basic::unit::UnitOut),
        X(bool),
        Z(super::basic::void::VoidOut),
    }

    pub struct BarInToOut {
        s: super::basic::unit::UnitInToOut,
        t: Box<dyn FnOnce(super::basic::unit::UnitIn) -> BarOut>,
        w: Box<dyn FnOnce(super::basic::void::VoidIn) -> BarOut>,
        y: Box<dyn FnOnce(bool) -> BarOut>,
        z: super::basic::void::VoidInToOut,
    }

    impl From<self::BarOut> for self::BarIn {
        fn from(out: self::BarOut) -> Self {
            match out {
                self::BarOut::S(payload) => self::BarIn::S(payload.into()),
                self::BarOut::X(payload) => self::BarIn::X(payload.into()),
                self::BarOut::Z(payload) => self::BarIn::Z(payload.into()),
            }
        }
    }

    impl From<(self::BarIn, self::BarInToOut)> for self::BarOut {
        fn from((r#in, in_to_out): (self::BarIn, self::BarInToOut)) -> Self {
            match r#in {
                self::BarIn::S(payload) => self::BarOut::S((payload, in_to_out.s).into()),
                self::BarIn::T(payload) => (in_to_out.t)(payload),
                self::BarIn::W(payload) => (in_to_out.w)(payload),
                self::BarIn::X(payload) => self::BarOut::X(payload),
                self::BarIn::Y(payload) => (in_to_out.y)(payload),
                self::BarIn::Z(payload) => self::BarOut::Z((payload, in_to_out.z).into()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct FooIn {
        s: super::basic::unit::UnitIn,
        x: bool,
        z: super::basic::void::VoidIn,
    }

    #[derive(Clone, Debug)]
    pub struct FooOut {
        s: super::basic::unit::UnitOut,
        t: super::basic::unit::UnitOut,
        w: super::basic::void::VoidOut,
        x: bool,
        y: bool,
        z: super::basic::void::VoidOut,
    }

    pub struct FooInToOut {
        s: super::basic::unit::UnitInToOut,
        t: super::basic::unit::UnitOut,
        w: super::basic::void::VoidOut,
        y: bool,
        z: super::basic::void::VoidInToOut,
    }

    impl From<self::FooOut> for self::FooIn {
        fn from(out: self::FooOut) -> Self {
            self::FooIn {
                s: out.s.into(),
                x: out.x.into(),
                z: out.z.into(),
            }
        }
    }

    impl From<(self::FooIn, self::FooInToOut)> for self::FooOut {
        fn from((r#in, in_to_out): (self::FooIn, self::FooInToOut)) -> Self {
            self::FooOut {
                s: (r#in.s, in_to_out.s).into(),
                t: in_to_out.t,
                w: in_to_out.w,
                x: r#in.x,
                y: in_to_out.y,
                z: (r#in.z, in_to_out.z).into(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct FooAndBarIn {
        bar: BarIn,
        foo: FooIn,
    }

    #[derive(Clone, Debug)]
    pub struct FooAndBarOut {
        bar: BarOut,
        foo: FooOut,
    }

    pub struct FooAndBarInToOut {
        bar: BarInToOut,
        foo: FooInToOut,
    }

    impl From<self::FooAndBarOut> for self::FooAndBarIn {
        fn from(out: self::FooAndBarOut) -> Self {
            self::FooAndBarIn {
                bar: out.bar.into(),
                foo: out.foo.into(),
            }
        }
    }

    impl From<(self::FooAndBarIn, self::FooAndBarInToOut)> for self::FooAndBarOut {
        fn from((r#in, in_to_out): (self::FooAndBarIn, self::FooAndBarInToOut)) -> Self {
            self::FooAndBarOut {
                bar: (r#in.bar, in_to_out.bar).into(),
                foo: (r#in.foo, in_to_out.foo).into(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum FooOrBarIn {
        Bar(BarIn),
        Foo(FooIn),
    }

    #[derive(Clone, Debug)]
    pub enum FooOrBarOut {
        Bar(BarOut),
        Foo(FooOut),
    }

    pub struct FooOrBarInToOut {
        bar: BarInToOut,
        foo: FooInToOut,
    }

    impl From<self::FooOrBarOut> for self::FooOrBarIn {
        fn from(out: self::FooOrBarOut) -> Self {
            match out {
                self::FooOrBarOut::Bar(payload) => self::FooOrBarIn::Bar(payload.into()),
                self::FooOrBarOut::Foo(payload) => self::FooOrBarIn::Foo(payload.into()),
            }
        }
    }

    impl From<(self::FooOrBarIn, self::FooOrBarInToOut)> for self::FooOrBarOut {
        fn from((r#in, in_to_out): (self::FooOrBarIn, self::FooOrBarInToOut)) -> Self {
            match r#in {
                self::FooOrBarIn::Bar(payload) => \
                    self::FooOrBarOut::Bar((payload, in_to_out.bar).into()),
                self::FooOrBarIn::Foo(payload) => \
                    self::FooOrBarOut::Foo((payload, in_to_out.foo).into()),
            }
        }
    }
}
",
        );
    }
}
