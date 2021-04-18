use crate::{
    naming_conventions::{pascal_case, snake_case},
    schema,
    schema::relativize_namespace,
};
use std::{
    collections::BTreeMap,
    iter::{empty, once},
    path::PathBuf,
};

// The string to be used for each indentation level.
const INDENTATION: &str = "    ";

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
        "{}pub mod {} {{\n{}{}}}\n",
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

// Render a struct, including a trailing line break.
fn render_struct(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &str,
    fields: &[schema::Field],
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    format!(
        "{}#[allow(dead_code)]\n{}pub struct {} {{\n{}{}}}\n",
        indentation_str,
        indentation_str,
        pascal_case(name),
        fields
            .iter()
            .map(|field| render_struct_field(imports, namespace, field, indentation + 1))
            .collect::<Vec<_>>()
            .join(""),
        indentation_str,
    )
}

// Render a choice, including a trailing line break.
fn render_choice(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    name: &str,
    fields: &[schema::Field],
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    format!(
        "{}#[allow(dead_code)]\n{}pub enum {} {{\n{}{}}}\n",
        indentation_str,
        indentation_str,
        pascal_case(name),
        fields
            .iter()
            .map(|field| render_choice_field(imports, namespace, field, indentation + 1))
            .collect::<Vec<_>>()
            .join(""),
        indentation_str,
    )
}

// Render a field of a struct, including a trailing line break.
fn render_struct_field(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    field: &schema::Field,
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    format!(
        "{}{}: {},\n",
        indentation_str,
        snake_case(&field.name),
        render_type(imports, namespace, &field.r#type),
    )
}

// Render a field of a choice, including a trailing line break.
fn render_choice_field(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    field: &schema::Field,
    indentation: u64,
) -> String {
    let indentation_str = (0..indentation).map(|_| INDENTATION).collect::<String>();

    format!(
        "{}{}({}),\n",
        indentation_str,
        pascal_case(&field.name),
        render_type(imports, namespace, &field.r#type),
    )
}

// Render a type with no line breaks.
fn render_type(
    imports: &BTreeMap<String, schema::Namespace>,
    namespace: &schema::Namespace,
    r#type: &schema::Type,
) -> String {
    let type_namespace = schema::Namespace {
        components: if let Some(import) = &r#type.import {
            imports[import].components.clone()
        } else {
            vec!["self".to_owned()]
        },
    };

    let (relative_type_namespace, ancestors) = relativize_namespace(&type_namespace, namespace);

    let mut components = vec![];

    if ancestors == 0 {
        components.push("self".to_owned());
    } else {
        for _ in 0..ancestors {
            components.push("super".to_owned());
        }
    }

    components.extend(
        relative_type_namespace
            .components
            .iter()
            .map(|component| snake_case(component)),
    );

    components.push(pascal_case(&r#type.name));

    components.join("::")
}

#[cfg(test)]
mod tests {
    use crate::{
        generate_rust::generate, parser::parse, schema, tokenizer::tokenize, validator::validate,
    };
    use std::{collections::BTreeMap, path::Path};

    #[test]
    fn generate_empty() {
        let path = Path::new("foo.t").to_owned();
        let contents = "".to_owned();
        let tokens = tokenize(&path, &contents).unwrap();
        let schema = parse(&path, &contents, &tokens).unwrap();
        let mut schemas = BTreeMap::new();
        schemas.insert(
            schema::Namespace {
                components: vec!["foo".to_owned()],
            },
            (schema, path, contents),
        );
        validate(&schemas).unwrap();

        assert_eq!(
            generate(schemas),
            "\
                pub mod foo {\n\
                }\n\
            ",
        );
    }
}
