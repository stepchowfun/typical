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
#[derive(Copy, Clone, Debug)]
enum CaseConvention {
    Pascal,
    Snake,
}

use CaseConvention::{Pascal, Snake};

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

    // Write the code.
    let mut buffer = String::new();

    if !tree.children.is_empty() || !tree.schema.declarations.is_empty() {
        // The `unwrap` is safe because the `std::fmt::Write` impl for `String` is infallible.
        writeln!(
            &mut buffer,
            "#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings)]",
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
    write_identifier(buffer, name, Snake)?;
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
                write_struct(buffer, indentation, &imports, namespace, &name, fields)?;
            }
            schema::DeclarationVariant::Choice(fields) => {
                write_choice(buffer, indentation, &imports, namespace, &name, fields)?;
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
    fields: &BTreeMap<Identifier, schema::Field>,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    writeln!(buffer, "#[derive({})]", TRAITS_TO_DERIVE.join(", "))?;
    write_indentation(buffer, indentation)?;
    write!(buffer, "pub struct ")?;
    write_identifier(buffer, name, Pascal)?;
    writeln!(buffer, " {{")?;

    for (name, field) in fields {
        write_struct_field(buffer, indentation + 1, imports, namespace, name, field)?;
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
    fields: &BTreeMap<Identifier, schema::Field>,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    writeln!(buffer, "#[derive({})]", TRAITS_TO_DERIVE.join(", "))?;
    write_indentation(buffer, indentation)?;
    write!(buffer, "pub enum ")?;
    write_identifier(buffer, name, Pascal)?;
    writeln!(buffer, " {{")?;

    for (name, field) in fields {
        write_choice_field(buffer, indentation + 1, imports, namespace, name, field)?;
    }

    write_indentation(buffer, indentation)?;
    writeln!(buffer, "}}")?;

    Ok(())
}

// Write a field of a struct, including a trailing line break.
fn write_struct_field<T: Write>(
    buffer: &mut T,
    indentation: u64,
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    field: &schema::Field,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    write_identifier(buffer, name, Snake)?;
    write!(buffer, ": ")?;
    write_type(buffer, imports, namespace, &field.r#type)?;
    writeln!(buffer, ",")?;

    Ok(())
}

// Write a field of a choice, including a trailing line break.
fn write_choice_field<T: Write>(
    buffer: &mut T,
    indentation: u64,
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &Identifier,
    field: &schema::Field,
) -> Result<(), fmt::Error> {
    write_indentation(buffer, indentation)?;
    write_identifier(buffer, name, Pascal)?;
    write!(buffer, "(")?;
    write_type(buffer, imports, namespace, &field.r#type)?;
    writeln!(buffer, "),")?;

    Ok(())
}

// Write a type.
fn write_type<T: Write>(
    buffer: &mut T,
    imports: &BTreeMap<Identifier, schema::Namespace>,
    namespace: &schema::Namespace,
    r#type: &schema::Type,
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
                write_identifier(buffer, &component, Snake)?;
                write!(buffer, "::")?;
            }

            write_identifier(buffer, name, Pascal)?;
        }
    }

    Ok(())
}

// Write an identifier with an optional flavor suffix in a way that Rust will be happy with.
fn write_identifier<T: Write>(
    buffer: &mut T,
    identifier: &Identifier,
    case: CaseConvention,
) -> Result<(), fmt::Error> {
    let converted_name = match case {
        CaseConvention::Pascal => identifier.pascal_case(),
        CaseConvention::Snake => identifier.snake_case(),
    };

    if !converted_name.starts_with("r#")
        && RUST_KEYWORDS
            .iter()
            .any(|keyword| converted_name == *keyword)
    {
        write!(buffer, "r#")?;
    }

    write!(buffer, "{}", converted_name)?;

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
            generate(schemas),
            "\
#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings)]

#[rustfmt::skip]
pub mod basic {
    #[rustfmt::skip]
    pub mod unit {
        #[derive(Clone, Debug)]
        pub struct Unit {
        }
    }

    #[rustfmt::skip]
    pub mod void {
        #[derive(Clone, Debug)]
        pub enum Void {
        }
    }
}

#[rustfmt::skip]
pub mod main {
    #[derive(Clone, Debug)]
    pub enum Bar {
        S(super::basic::unit::Unit),
        T(super::basic::unit::Unit),
        W(super::basic::void::Void),
        X(bool),
        Y(bool),
        Z(super::basic::void::Void),
    }

    #[derive(Clone, Debug)]
    pub struct Foo {
        s: super::basic::unit::Unit,
        t: super::basic::unit::Unit,
        w: super::basic::void::Void,
        x: bool,
        y: bool,
        z: super::basic::void::Void,
    }

    #[derive(Clone, Debug)]
    pub struct FooAndBar {
        bar: Bar,
        foo: Foo,
    }

    #[derive(Clone, Debug)]
    pub enum FooOrBar {
        Bar(Bar),
        Foo(Foo),
    }
}
",
        );
    }
}
