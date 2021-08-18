use crate::{
    identifier::Identifier,
    schema::{self, relativize_namespace},
};
use std::{
    collections::BTreeMap,
    fmt::{self, Write},
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

// This struct represents a tree of schemas organized in a module hierarchy.
#[derive(Clone, Debug)]
struct Module {
    children: BTreeMap<Identifier, Module>,
    schema: schema::Schema,
}

// This enum represents a case convention for the `write_identifier` function below.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaseConvention {
    Pascal,
    Snake,
}

use CaseConvention::{Pascal, Snake};

// This enum is used to distinguish between the flavors of a struct type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StructFlavor {
    In,
    Out,
}

// This enum is used to distinguish between the flavors of a choice type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChoiceFlavor {
    InOrOut(StructFlavor),
    OutStable,
}

// Generate Rust code from a schema and its transitive dependencies.
pub fn generate(
    typical_version: &str,
    schemas: BTreeMap<schema::Namespace, (schema::Schema, PathBuf, String)>,
) -> String {
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

    // Write the code.
    let mut buffer = String::new();

    if !tree.children.is_empty() || !tree.schema.declarations.is_empty() {
        // The `unwrap` is safe because the `std::fmt::Write` impl for `String` is infallible.
        writeln!(
            &mut buffer,
            "\
// This file was automatically generated by Typical {}.
// Visit https://github.com/stepchowfun/typical for more information.

#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings)]",
            typical_version,
        )
        .unwrap();

        // The `unwrap` is safe because the `std::fmt::Write` impl for `String` is infallible.
        writeln!(&mut buffer).unwrap();

        // The `unwrap` is safe because the `std::fmt::Write` impl for `String` is infallible.
        write_module_contents(
            &mut buffer,
            0,
            &schema::Namespace { components: vec![] },
            &tree.children,
            &tree.schema,
        )
        .unwrap();
    }

    buffer
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

// Write a module, including a trailing line break.
fn write_module<T: Write>(
    buffer: &mut T,
    indentation: u64,
    namespace: &schema::Namespace,
    name: &Identifier,
    module: &Module,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    writeln!(buffer, "#[rustfmt::skip]")?;
    write_indentation(buffer, indentation)?;
    write!(buffer, "pub mod ")?;
    write_identifier(buffer, name, Snake, None)?;
    writeln!(buffer, " {{")?;

    let mut new_namespace = namespace.clone();
    new_namespace.components.push(name.clone());

    write_module_contents(
        buffer,
        indentation + 1,
        &new_namespace,
        &module.children,
        &module.schema,
    )?;

    write_indentation(buffer, indentation)?;
    writeln!(buffer, "}}")?;

    Ok(())
}

// Write the contents of a module, including a trailing line break if there was anything to render.
fn write_module_contents<T: Write>(
    buffer: &mut T,
    indentation: u64,
    namespace: &schema::Namespace,
    children: &BTreeMap<Identifier, Module>,
    schema: &schema::Schema,
) -> Result<(), fmt::Error> {
    let schema_empty = schema.declarations.is_empty();

    for (i, (child_name, child)) in children.iter().enumerate() {
        write_module(buffer, indentation, namespace, child_name, child)?;

        if i < children.len() - 1 || !schema_empty {
            writeln!(buffer)?;
        }
    }

    write_schema(buffer, indentation, namespace, schema)?;

    Ok(())
}

// Write a schema, including a trailing line break if there was anything to render.
#[allow(clippy::too_many_lines)]
fn write_schema<T: Write>(
    buffer: &mut T,
    indentation: u64,
    namespace: &schema::Namespace,
    schema: &schema::Schema,
) -> Result<(), fmt::Error> {
    // Construct a map from import name to namespace.
    let mut imports = BTreeMap::new();
    for (name, import) in &schema.imports {
        // The unwrap is safe due to [ref:namespace_populated].
        imports.insert(name.clone(), import.namespace.clone().unwrap());
    }

    // Write the declarations.
    let mut iter = schema.declarations.iter().peekable();
    while let Some((name, declaration)) = iter.next() {
        match &declaration.variant {
            schema::DeclarationVariant::Struct(fields) => {
                write_struct(
                    buffer,
                    indentation,
                    &imports,
                    namespace,
                    &name,
                    fields,
                    StructFlavor::In,
                )?;

                writeln!(buffer)?;

                write_struct(
                    buffer,
                    indentation,
                    &imports,
                    namespace,
                    &name,
                    fields,
                    StructFlavor::Out,
                )?;

                writeln!(buffer)?;

                write_indentation(buffer, indentation)?;
                write!(buffer, "impl From<")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::Out)),
                )?;
                write!(buffer, "> for ")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::In)),
                )?;
                writeln!(buffer, " {{")?;
                write_indentation(buffer, indentation + 1)?;
                write!(buffer, "fn from(message: ")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::Out)),
                )?;
                writeln!(buffer, ") -> Self {{")?;
                write_indentation(buffer, indentation + 2)?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::In)),
                )?;
                writeln!(buffer, " {{")?;
                for field in fields {
                    write_indentation(buffer, indentation + 3)?;
                    write_identifier(buffer, &field.name, Snake, None)?;
                    write!(buffer, ": ")?;
                    if field.unstable {
                        write!(buffer, "Some(")?;
                    }
                    write!(buffer, "message.")?;
                    write_identifier(buffer, &field.name, Snake, None)?;
                    write!(buffer, ".into()")?;
                    if field.unstable {
                        write!(buffer, ")")?;
                    }
                    writeln!(buffer, ",")?;
                }
                write_indentation(buffer, indentation + 2)?;
                writeln!(buffer, "}}")?;
                write_indentation(buffer, indentation + 1)?;
                writeln!(buffer, "}}")?;
                write_indentation(buffer, indentation)?;
                writeln!(buffer, "}}")?;
            }
            schema::DeclarationVariant::Choice(fields) => {
                write_choice(
                    buffer,
                    indentation,
                    &imports,
                    namespace,
                    &name,
                    fields,
                    ChoiceFlavor::InOrOut(StructFlavor::In),
                )?;

                writeln!(buffer)?;

                write_choice(
                    buffer,
                    indentation,
                    &imports,
                    namespace,
                    &name,
                    fields,
                    ChoiceFlavor::InOrOut(StructFlavor::Out),
                )?;

                writeln!(buffer)?;

                write_choice(
                    buffer,
                    indentation,
                    &imports,
                    namespace,
                    &name,
                    fields,
                    ChoiceFlavor::OutStable,
                )?;

                writeln!(buffer)?;

                write_indentation(buffer, indentation)?;
                write!(buffer, "impl From<")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::Out)),
                )?;
                write!(buffer, "> for ")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::In)),
                )?;
                writeln!(buffer, " {{")?;
                write_indentation(buffer, indentation + 1)?;
                write!(buffer, "fn from(message: ")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::Out)),
                )?;
                writeln!(buffer, ") -> Self {{")?;
                write_indentation(buffer, indentation + 2)?;
                writeln!(buffer, "match message {{")?;
                for field in fields {
                    write_indentation(buffer, indentation + 3)?;
                    write_identifier(
                        buffer,
                        &name,
                        Pascal,
                        Some(ChoiceFlavor::InOrOut(StructFlavor::Out)),
                    )?;
                    write!(buffer, "::")?;
                    write_identifier(buffer, &field.name, Pascal, None)?;
                    write!(buffer, "(payload")?;
                    if field.unstable {
                        write!(buffer, ", _, _")?;
                    }
                    write!(buffer, ") => ")?;
                    write_identifier(
                        buffer,
                        &name,
                        Pascal,
                        Some(ChoiceFlavor::InOrOut(StructFlavor::In)),
                    )?;
                    write!(buffer, "::")?;
                    write_identifier(buffer, &field.name, Pascal, None)?;
                    writeln!(buffer, "(payload.into()),")?;
                }
                write_indentation(buffer, indentation + 2)?;
                writeln!(buffer, "}}")?;
                write_indentation(buffer, indentation + 1)?;
                writeln!(buffer, "}}")?;
                write_indentation(buffer, indentation)?;
                writeln!(buffer, "}}")?;
            }
        }

        if iter.peek().is_some() {
            writeln!(buffer)?;
        }
    }

    Ok(())
}

// Write a struct, including a trailing line break.
fn write_struct<T: Write>(
    buffer: &mut T,
    indentation: u64,
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    fields: &[schema::Field],
    flavor: StructFlavor,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    writeln!(buffer, "#[derive({})]", TRAITS_TO_DERIVE.join(", "))?;
    write_indentation(buffer, indentation)?;
    write!(buffer, "pub struct ")?;
    write_identifier(buffer, &name, Pascal, Some(ChoiceFlavor::InOrOut(flavor)))?;
    writeln!(buffer, " {{")?;

    for field in fields {
        write_indentation(buffer, indentation + 1)?;
        write_identifier(buffer, &field.name, Snake, None)?;
        write!(buffer, ": ")?;
        if field.unstable && flavor == StructFlavor::In {
            write!(buffer, "Option<")?;
        }
        write_type(
            buffer,
            imports,
            namespace,
            &field.r#type,
            ChoiceFlavor::InOrOut(flavor),
        )?;
        if field.unstable && flavor == StructFlavor::In {
            write!(buffer, ">")?;
        }
        writeln!(buffer, ",")?;
    }

    write_indentation(buffer, indentation)?;
    writeln!(buffer, "}}")?;

    Ok(())
}

// Write a choice, including a trailing line break.
fn write_choice<T: Write>(
    buffer: &mut T,
    indentation: u64,
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    fields: &[schema::Field],
    flavor: ChoiceFlavor,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    writeln!(buffer, "#[derive({})]", TRAITS_TO_DERIVE.join(", "))?;
    write_indentation(buffer, indentation)?;
    write!(buffer, "pub enum ")?;
    write_identifier(buffer, &name, Pascal, Some(flavor))?;
    writeln!(buffer, " {{")?;

    for field in fields {
        if !(flavor == ChoiceFlavor::OutStable && field.unstable) {
            let flavor = match flavor {
                ChoiceFlavor::InOrOut(flavor) => flavor,
                ChoiceFlavor::OutStable => StructFlavor::Out,
            };
            write_indentation(buffer, indentation + 1)?;
            write_identifier(buffer, &field.name, Pascal, None)?;
            write!(buffer, "(")?;
            write_type(
                buffer,
                imports,
                namespace,
                &field.r#type,
                ChoiceFlavor::InOrOut(flavor),
            )?;
            if flavor == StructFlavor::Out && field.unstable {
                write!(buffer, ", Vec<")?;
                write_identifier(
                    buffer,
                    &name,
                    Pascal,
                    Some(ChoiceFlavor::InOrOut(StructFlavor::Out)),
                )?;
                write!(buffer, ">, ")?;
                write_identifier(buffer, &name, Pascal, Some(ChoiceFlavor::OutStable))?;
            }
            writeln!(buffer, "),")?;
        }
    }

    write_indentation(buffer, indentation)?;
    writeln!(buffer, "}}")?;

    Ok(())
}

// Write a type.
fn write_type<T: Write>(
    buffer: &mut T,
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    r#type: &schema::Type,
    flavor: ChoiceFlavor,
) -> Result<(), fmt::Error> {
    match &r#type.variant {
        schema::TypeVariant::Bool => {
            write!(buffer, "bool")?;
        }
        schema::TypeVariant::Custom(import, name) => {
            let type_namespace = schema::Namespace {
                components: import.as_ref().map_or_else(
                    || namespace.components.clone(),
                    |import| imports[import].components.clone(),
                ),
            };

            let (relative_type_namespace, ancestors) =
                relativize_namespace(&type_namespace, namespace);

            for _ in 0..ancestors {
                write!(buffer, "super::")?;
            }

            for component in relative_type_namespace.components {
                write_identifier(buffer, &component, Snake, None)?;
                write!(buffer, "::")?;
            }

            write_identifier(buffer, &name, Pascal, Some(flavor))?;
        }
    }

    Ok(())
}

// Write an identifier with an optional flavor suffix in a way that Rust will be happy with.
fn write_identifier<T: Write>(
    buffer: &mut T,
    identifier: &Identifier,
    case: CaseConvention,
    suffix: Option<ChoiceFlavor>,
) -> Result<(), fmt::Error> {
    let identifier_with_suffix = suffix.map_or_else(
        || identifier.clone(),
        |suffix| {
            identifier.join(
                &match suffix {
                    ChoiceFlavor::InOrOut(StructFlavor::In) => "In",
                    ChoiceFlavor::InOrOut(StructFlavor::Out) => "Out",
                    ChoiceFlavor::OutStable => "OutStable",
                }
                .into(),
            )
        },
    );

    let converted_identifier = match case {
        CaseConvention::Pascal => identifier_with_suffix.pascal_case(),
        CaseConvention::Snake => identifier_with_suffix.snake_case(),
    };

    if !converted_identifier.starts_with("r#")
        && RUST_KEYWORDS
            .iter()
            .any(|keyword| converted_identifier == *keyword)
    {
        write!(buffer, "r#")?;
    }

    write!(buffer, "{}", converted_identifier)?;

    Ok(())
}

// Write the given level of indentation.
fn write_indentation<T: Write>(buffer: &mut T, indentation: u64) -> Result<(), fmt::Error> {
    for _ in 0..indentation {
        write!(buffer, "{}", INDENTATION)?;
    }

    Ok(())
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
        let unit_path = Path::new("integration-tests/types/basic/unit.t").to_owned();
        let unit_contents = read_to_string(&unit_path).unwrap();

        let void_namespace = schema::Namespace {
            components: vec!["basic".into(), "void".into()],
        };
        let void_path = Path::new("integration-tests/types/basic/void.t").to_owned();
        let void_contents = read_to_string(&void_path).unwrap();

        let main_namespace = schema::Namespace {
            components: vec!["main".into()],
        };
        let main_path = Path::new("integration-tests/types/main.t").to_owned();
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
            generate("0.0.0", schemas),
            "\
// This file was automatically generated by Typical 0.0.0.
// Visit https://github.com/stepchowfun/typical for more information.

#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings)]

#[rustfmt::skip]
pub mod basic {
    #[rustfmt::skip]
    pub mod unit {
        #[derive(Clone, Debug)]
        pub struct UnitIn {
        }

        #[derive(Clone, Debug)]
        pub struct UnitOut {
        }

        impl From<UnitOut> for UnitIn {
            fn from(message: UnitOut) -> Self {
                UnitIn {
                }
            }
        }
    }

    #[rustfmt::skip]
    pub mod void {
        #[derive(Clone, Debug)]
        pub enum VoidIn {
        }

        #[derive(Clone, Debug)]
        pub enum VoidOut {
        }

        #[derive(Clone, Debug)]
        pub enum VoidOutStable {
        }

        impl From<VoidOut> for VoidIn {
            fn from(message: VoidOut) -> Self {
                match message {
                }
            }
        }
    }
}

#[rustfmt::skip]
pub mod main {
    #[derive(Clone, Debug)]
    pub enum BarIn {
        X(bool),
        Y(bool),
        Z(super::basic::void::VoidIn),
        W(super::basic::void::VoidIn),
        S(super::basic::unit::UnitIn),
        T(super::basic::unit::UnitIn),
    }

    #[derive(Clone, Debug)]
    pub enum BarOut {
        X(bool),
        Y(bool, Vec<BarOut>, BarOutStable),
        Z(super::basic::void::VoidOut),
        W(super::basic::void::VoidOut, Vec<BarOut>, BarOutStable),
        S(super::basic::unit::UnitOut),
        T(super::basic::unit::UnitOut, Vec<BarOut>, BarOutStable),
    }

    #[derive(Clone, Debug)]
    pub enum BarOutStable {
        X(bool),
        Z(super::basic::void::VoidOut),
        S(super::basic::unit::UnitOut),
    }

    impl From<BarOut> for BarIn {
        fn from(message: BarOut) -> Self {
            match message {
                BarOut::X(payload) => BarIn::X(payload.into()),
                BarOut::Y(payload, _, _) => BarIn::Y(payload.into()),
                BarOut::Z(payload) => BarIn::Z(payload.into()),
                BarOut::W(payload, _, _) => BarIn::W(payload.into()),
                BarOut::S(payload) => BarIn::S(payload.into()),
                BarOut::T(payload, _, _) => BarIn::T(payload.into()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct FooIn {
        x: bool,
        y: Option<bool>,
        z: super::basic::void::VoidIn,
        w: Option<super::basic::void::VoidIn>,
        s: super::basic::unit::UnitIn,
        t: Option<super::basic::unit::UnitIn>,
    }

    #[derive(Clone, Debug)]
    pub struct FooOut {
        x: bool,
        y: bool,
        z: super::basic::void::VoidOut,
        w: super::basic::void::VoidOut,
        s: super::basic::unit::UnitOut,
        t: super::basic::unit::UnitOut,
    }

    impl From<FooOut> for FooIn {
        fn from(message: FooOut) -> Self {
            FooIn {
                x: message.x.into(),
                y: Some(message.y.into()),
                z: message.z.into(),
                w: Some(message.w.into()),
                s: message.s.into(),
                t: Some(message.t.into()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct FooAndBarIn {
        foo: FooIn,
        bar: BarIn,
    }

    #[derive(Clone, Debug)]
    pub struct FooAndBarOut {
        foo: FooOut,
        bar: BarOut,
    }

    impl From<FooAndBarOut> for FooAndBarIn {
        fn from(message: FooAndBarOut) -> Self {
            FooAndBarIn {
                foo: message.foo.into(),
                bar: message.bar.into(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum FooOrBarIn {
        Foo(FooIn),
        Bar(BarIn),
    }

    #[derive(Clone, Debug)]
    pub enum FooOrBarOut {
        Foo(FooOut),
        Bar(BarOut),
    }

    #[derive(Clone, Debug)]
    pub enum FooOrBarOutStable {
        Foo(FooOut),
        Bar(BarOut),
    }

    impl From<FooOrBarOut> for FooOrBarIn {
        fn from(message: FooOrBarOut) -> Self {
            match message {
                FooOrBarOut::Foo(payload) => FooOrBarIn::Foo(payload.into()),
                FooOrBarOut::Bar(payload) => FooOrBarIn::Bar(payload.into()),
            }
        }
    }
}
",
        );
    }
}
