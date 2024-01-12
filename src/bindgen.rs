use serde::Deserialize;
use serde_yaml::{Number, Value};
use std::{collections::HashMap, fs::File, io::Write};

// Type names.
const TYPE_DATETIME: &'static str = "DateTime";
const TYPE_URI: &'static str = "Uri";

#[derive(Debug, Deserialize)]
struct Schema {
    #[serde(rename = "openapi")]
    open_api: String,
    info: SchemaInfo,
    paths: HashMap<String, Path>,
    components: ComponentSchemas,
}

#[derive(Debug, Deserialize)]
struct SchemaInfo {
    title: String,
    version: String,
    license: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Path {
    get: Option<PathOp>,
    post: Option<PathOp>,
    put: Option<PathOp>,
    patch: Option<PathOp>,
    delete: Option<PathOp>,
}

#[derive(Debug, Deserialize)]
struct PathOp {
    #[serde(rename = "operationId")]
    operation_id: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct ComponentSchemas {
    schemas: HashMap<String, Component>,
    #[serde(rename = "securitySchemes")]
    security_schemes: HashMap<String, Component>,
}

#[derive(Debug, Deserialize)]
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
}

/// Makes a comment out of a given string.
fn make_comment<'a>(input: String) -> String {
    return input
        .split("\n")
        .map(|x| format!("/// {}\n", x))
        .collect::<Vec<_>>()
        .concat();
}

/// Generates the Rust bindings from a file.
pub fn gen(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let yaml: Schema = serde_yaml::from_str(&input).unwrap();

    // For every struct.
    for (name, comp) in &yaml.components.schemas {
        // Create and open the output file.
        let mut output = File::create(format!("output/{}.rs", name)).unwrap();

        // Contains enums that were defined by this struct.
        let mut enums = HashMap::new();

        // Prepend slashes to all lines in the documentation string.
        let desc = make_comment(
            comp.description
                .clone()
                .unwrap_or("No description available".to_owned()),
        );

        // Write description.
        output.write(desc.as_bytes()).unwrap();

        // Write name.
        output.write(b"struct ").unwrap();
        output.write(name.as_bytes()).unwrap();
        output.write(b" {\n").unwrap();

        // For every struct field.
        for (prop_name, prop) in &comp.properties {
            // Get type or skip this property if none given.
            let t = match &prop.typ {
                Some(val) => val.as_str(),
                None => continue,
            };
            // Resolve the type from YAML to a Rust type.
            let mut resolved_type = match t {
                // "string" can mean either a plain or formatted string or an enum declaration.
                "string" => match &prop.format {
                    Some(x) => match x.as_str() {
                        "uri" => TYPE_URI,
                        "date-time" => TYPE_DATETIME,
                        _ => match &prop.enumeration {
                            // Handle inline enum declaration.
                            // TODO: Name mangling.
                            Some(y) => {
                                enums.insert(prop_name.clone(), y.clone());
                                println!("Added {}", &prop_name);
                                continue;
                            }
                            None => "String",
                        },
                    },
                    None => continue,
                },
                // "integer" usually means i64.
                "integer" => match &prop.format {
                    Some(val) => match val.as_str() {
                        _ => "i64",
                    },
                    None => "i64",
                },
                // "number" usually means f64.
                "number" => match &prop.format {
                    Some(val) => match val.as_str() {
                        _ => "f64",
                    },
                    None => "f64",
                },
                "boolean" => "bool",
                // TODO
                "array" => "Vec<?>",
                _ => "?",
            };

            // If a property is nullable, convert the field type to an Option<T>.
            let fmt = format!("Option<{}>", resolved_type);
            if prop.nullable.unwrap_or(false) {
                resolved_type = fmt.as_str();
            }

            // Escape field names if they are Rust keywords.
            let name = match prop_name.as_str() {
                "type" => "r#type",
                _ => &prop_name,
            };

            let desc = prop
                .description
                .clone()
                .unwrap_or("No description available.".to_owned());
            output.write(b"\t").unwrap();
            output.write(make_comment(desc).as_bytes()).unwrap();
            output
                .write(format!("\t{}: {},\n", name, resolved_type).as_bytes())
                .unwrap();
        }
        output.write(b"}\n\n").unwrap();

        // Write the generated enums.
        for (name, vars) in &enums {
            output
                .write(format!("enum {} {{\n", name).as_bytes())
                .unwrap();
            for var in vars {
                output.write(var.as_bytes()).unwrap();
            }
            output.write(b"}\n\n").unwrap();
        }
    }
}
