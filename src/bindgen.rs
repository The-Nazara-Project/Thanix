use serde::Deserialize;
use serde_yaml::{Number, Value};
use std::{collections::HashMap, fs::File, io::Write};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Schema {
    #[serde(rename = "openapi")]
    open_api: String,
    info: SchemaInfo,
    paths: HashMap<String, Path>,
    components: ComponentSchemas,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SchemaInfo {
    title: String,
    version: String,
    license: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Path {
    get: Option<PathOp>,
    post: Option<PathOp>,
    put: Option<PathOp>,
    patch: Option<PathOp>,
    delete: Option<PathOp>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PathOp {
    #[serde(rename = "operationId")]
    operation_id: String,
    description: String,
    parameters: Option<Vec<Response>>,
    responses: HashMap<String, Response>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Response {}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ComponentSchemas {
    schemas: HashMap<String, Component>,
    #[serde(rename = "securitySchemes")]
    security_schemes: HashMap<String, Component>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Component {
    #[serde(rename = "type")]
    typ: String,
    description: Option<String>,
    #[serde(default)]
    properties: HashMap<String, Property>,
    #[serde(default)]
    required: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
struct Property {
    #[serde(rename = "type")]
    typ: Option<String>,
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    format: Option<String>,
    description: Option<String>,
    #[serde(rename = "minLength")]
    min_length: Option<Number>,
    #[serde(rename = "maxLength")]
    max_length: Option<Number>,
    #[serde(rename = "enum")]
    enumeration: Option<Vec<String>>,
    nullable: Option<bool>,
    properties: Option<Value>,
    items: Option<Value>,
    #[serde(rename = "allOf")]
    all_of: Option<Vec<Value>>,
}

/// Makes a comment out of a given string.
fn make_comment(input: String, indent: usize) -> String {
    return input
        .split('\n')
        .map(|x| format!("{}/// {}\n", "\t".repeat(indent), x))
        .collect::<Vec<_>>()
        .concat();
}

fn get_inner_type(items: Value) -> String {
    // Get inner type of the array.
    let inner_type = match items.get("$ref") {
        // Struct type
        Some(y) => y.as_str().unwrap().replace("#/components/schemas/", ""),
        // Normal type
        None => match items.get("type") {
            Some(y) => match y.as_str().unwrap() {
                "integer" => "i64".to_owned(),
                "number" => "f64".to_owned(),
                "string" => match items.get("format") {
                    Some(x) => match x.as_str().unwrap() {
                        "uri" => "Uri".to_owned(),
                        "date-time" => "DateTime".to_owned(),
                        _ => "String".to_owned(),
                    },
                    None => "String".to_owned(),
                },
                "boolean" => "bool".to_owned(),
                "object" => "Json".to_owned(),
                "array" => get_inner_type(match items.get("items") {
                    Some(z) => z.clone(),
                    None => panic!("array is missing items section!"),
                }),
                _ => panic!("unhandled type!"),
            },
            // We don't know what this is so assume a JSON object.
            None => "Json".to_owned(),
        },
    };
    let fmt = format!("Vec<{inner_type}>");
    fmt.clone()
}

/// Generates the Rust bindings from a file.
pub fn gen(input_path: impl AsRef<std::path::Path>) {
    // Parse the schema.
    let input = std::fs::read_to_string(input_path).unwrap();
    let yaml: Schema = serde_yaml::from_str(&input).unwrap();

    // Generate output folder.
    _ = std::fs::create_dir("output/");

    // Create and open the output file for structs.
    let mut types_file = File::create("output/types.rs").unwrap();

    // For every struct.
    for (name, comp) in &yaml.components.schemas {
        // Keep a record of all written fields for a constructor.
        let mut fields = Vec::new();
        // Prepend slashes to all lines in the documentation string.
        let desc = match comp.description.clone() {
            Some(d) => make_comment(d, 0),
            None => String::new(),
        };
        // Write description.
        types_file.write_all(desc.as_bytes()).unwrap();
        // Write name.
        types_file.write_all(b"pub struct ").unwrap();
        types_file.write_all(name.as_bytes()).unwrap();
        types_file.write_all(b" {\n").unwrap();
        // For every struct field.
        for (prop_name, prop) in &comp.properties {
            // Get the type of this field from YAML.
            let yaml_type = match prop.typ.as_ref() {
                Some(val) => val.as_str(),
                None => continue,
            };

            let mut type_result = match yaml_type {
                // "string" can mean either a plain or formatted string or an enum declaration.
                "string" => match &prop.format {
                    Some(x) => match x.as_str() {
                        "uri" => "Uri".to_owned(),
                        "date-time" => "DateTime".to_owned(),
                        _ => "String".to_owned(),
                    },
                    None => "String".to_owned(),
                },
                "integer" => "i64".to_owned(),
                "number" => "f64".to_owned(),
                "boolean" => "bool".to_owned(),
                "array" => get_inner_type(prop.items.as_ref().unwrap().clone()),
                "object" => "Json".to_owned(),
                _ => todo!(),
            };

            // Wrap type in an Option<T> if nullable.
            if prop.nullable.unwrap_or(false) {
                type_result = format!("Option<{type_result}>");
            }

            // Escape field names if they are Rust keywords.
            let name = match prop_name.as_str() {
                "type" => "r#type",
                _ => prop_name,
            };

            // Prepend slashes to all lines in the documentation string.
            if let Some(d) = prop.description.as_ref() {
                types_file
                    .write_all(make_comment(d.clone(), 1).as_bytes())
                    .unwrap();
            };
            // Write the field to file.
            types_file
                .write_all(format!("\t{}: {},\n", name, type_result).as_bytes())
                .unwrap();
            fields.push((name, type_result));
        }
        types_file.write_all(b"}\n\n").unwrap();
    }
}
