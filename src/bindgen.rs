use convert_case::{Case, Casing};
use serde::Deserialize;
use serde_yaml::{Number, Value};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

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
    operation_id: Option<String>,
    description: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    parameters: Vec<Parameter>,
    responses: Option<HashMap<String, Response>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Parameter {
    #[serde(rename = "in")]
    input: String,
    name: String,
    schema: Option<Value>,
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

/// Create all necessary structures and directories for the crate.
/// Requires `output` folder to exist.
///
/// # Arguments
///
/// - `output_name: &str` - The name of the output library given by the CLI. Default `output/`.
///
/// # Panics
///
/// This function panics when the `output/` directory does not exist.
fn create_lib_dir(output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting repackaging into crate...");
    let source_files = ["paths.rs", "types.rs", "util.rs"];

    if fs::metadata(output_name).is_err() {
        panic!("Fatal: Output directory does not exist!");
    }

    std::env::set_current_dir(output_name)?;

    for src_file in &source_files {
        if fs::metadata(src_file).is_err() {
            panic!("Source file {} does not exist!", src_file);
        }
    }

    let src_dir = "src";
    fs::create_dir_all(src_dir)?;

    for src_file in source_files {
        let dest_path = format!("{}/{}", src_dir, src_file);
        fs::rename(src_file, &dest_path)?;
    }

    let mut lib_file = fs::File::create(format!("{}/lib.rs", src_dir))?;
    write!(lib_file, "{}", include_str!("templates/lib.rs.template"))?;

    let cargo_toml = format!(include_str!("templates/Cargo.toml.template"), output_name);

    fs::write("Cargo.toml", cargo_toml)?;

    let readme_contents = r#"
# Readme

This output was automatically generated by `Thanix` (github.com/The-Nazara-Project/Thanix).
"#;

    fs::write("README.md", readme_contents)?;

    println!("Output successfully repackaged!");
    Ok(())
}

/// Makes a comment out of a given string.
fn make_comment(input: String, indent: usize) -> String {
    return input
        .split('\n')
        .map(|x| format!("{}/// {}\n", "\t".repeat(indent), x))
        .collect::<Vec<_>>()
        .concat();
}

fn make_fn_name_from_path(input: &str) -> String {
    input.replace("/api/", "").replace('/', "_")
}

/// Replaces reserved keywords in an input string for use in Rust.
fn fix_keywords(input: &str) -> String {
    input
        .replace("type", "typ")
        .replace("struct", "structure")
        .replace("fn", "func")
}

fn pathop_to_string(path: &str, input: &PathOp, method: &str) -> String {
    // Create a new struct for the query parameters.
    let fn_struct_params = input
        .parameters
        // Filter out only the query inputs.
        .iter()
        .filter(|x| x.input == "query")
        .enumerate()
        .map(|(s, p)| {
            format!(
                "\t{}: {}{}\n",
                fix_keywords(&p.name),
                get_inner_type(p.schema.as_ref().unwrap().clone(), false),
                if s < &input.parameters.len() - 1 {
                    ","
                } else {
                    ""
                }
            )
        })
        .collect::<String>();
    let fn_name = input
        .operation_id
        .clone()
        .unwrap_or(make_fn_name_from_path(&path));
    let fn_struct_name = fn_name.to_case(Case::Pascal) + "Query";
    let fn_struct = format!("#[derive(Debug, Serialize, Deserialize)]\npub struct {fn_struct_name} {{\n{fn_struct_params}}}");
    let comment = make_comment(input.description.clone().unwrap(), 0);
    let mut path_args = input
        .parameters
        // Filter out only the path inputs.
        .iter()
        .filter(|x| x.input == "path")
        .enumerate()
        .map(|(s, p)| {
            format!(
                "{}: {}{}",
                fix_keywords(&p.name),
                get_inner_type(p.schema.as_ref().unwrap().clone(), false),
                if s < &input.parameters.len() - 1 {
                    ","
                } else {
                    ""
                }
            )
        })
        .collect::<String>();
    if !path_args.is_empty() {
        path_args = ", ".to_owned() + &path_args;
    }
    return format!(
        include_str!("templates/path.template"),
        fn_struct, comment, fn_name, fn_struct_name, path_args, method, path
    );
}

fn get_inner_type(items: Value, append_vec: bool) -> String {
    // Get inner type of the array.
    let inner_type = match items.get("$ref") {
        // Struct type
        Some(y) => y.as_str().unwrap().replace("#/components/schemas/", ""),
        // Normal type
        None => match items.get("type") {
            Some(y) => match y.as_str().unwrap() {
                "integer" => match items.get("format") {
                    Some(x) => match x.as_str().unwrap() {
                        "int8" => "i8".to_owned(),
                        "int16" => "i16".to_owned(),
                        "int32" => "i32".to_owned(),
                        _ => "i64".to_owned(),
                    },
                    None => "i64".to_owned(),
                },
                "number" => "f64".to_owned(),
                "string" => match items.get("format") {
                    Some(x) => match x.as_str().unwrap() {
                        "uri" => "Url".to_owned(),
                        _ => "String".to_owned(),
                    },
                    None => "String".to_owned(),
                },
                "boolean" => "bool".to_owned(),
                "object" => "String".to_owned(),
                "array" => get_inner_type(
                    match items.get("items") {
                        Some(z) => z.clone(),
                        None => panic!("array is missing items section!"),
                    },
                    true,
                ),
                _ => panic!("unhandled type!"),
            },
            // We don't know what this is so assume a JSON object.
            None => "String".to_owned(),
        },
    };
    if append_vec {
        let fmt = format!("Vec<{inner_type}>");
        return fmt.clone();
    }
    inner_type
}

/// Executes a closure if the Option contains a Some value.
fn if_some<F: FnOnce(&T), T>(this: Option<T>, func: F) {
    if let Some(ref x) = this {
        func(x);
    }
}

/// Generates the Rust bindings from a file.
pub fn gen(input_path: impl AsRef<std::path::Path>, output_name: String) {
    // Parse the schema.
    let input = std::fs::read_to_string(input_path).unwrap();
    let yaml: Schema = serde_yaml::from_str(&input).unwrap();

    // Generate output folder.
    _ = std::fs::create_dir(&output_name);

    // Create and open the output file for structs.
    let mut types_file = File::create(output_name.clone() + "/types.rs").unwrap();
    types_file
        .write_all(include_str!("templates/usings.template").as_bytes())
        .unwrap();

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
                        "uri" => "Url".to_owned(),
                        _ => "String".to_owned(),
                    },
                    None => "String".to_owned(),
                },
                "integer" => "i64".to_owned(),
                "number" => "f64".to_owned(),
                "boolean" => "bool".to_owned(),
                "array" => get_inner_type(prop.items.as_ref().unwrap().clone(), true),
                "object" => "String".to_owned(),
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

    // Create and open the output file for paths.
    let mut paths_file = File::create(output_name.clone() + "/paths.rs").unwrap();

    paths_file
        .write_all(include_str!("templates/usings.template").as_bytes())
        .unwrap();

    // For every path.
    for (name, path) in &yaml.paths {
        if_some(path.get.as_ref(), |op| {
            paths_file
                .write_all(pathop_to_string(name, op, "get").as_bytes())
                .unwrap()
        });
        if_some(path.put.as_ref(), |op| {
            paths_file
                .write_all(pathop_to_string(name, op, "put").as_bytes())
                .unwrap()
        });
        if_some(path.post.as_ref(), |op| {
            paths_file
                .write_all(pathop_to_string(name, op, "post").as_bytes())
                .unwrap()
        });
        if_some(path.patch.as_ref(), |op| {
            paths_file
                .write_all(pathop_to_string(name, op, "patch").as_bytes())
                .unwrap()
        });
        if_some(path.delete.as_ref(), |op| {
            paths_file
                .write_all(pathop_to_string(name, op, "delete").as_bytes())
                .unwrap()
        });
    }
    fs::write(
        output_name.clone() + "/util.rs",
        include_str!("templates/util.rs.template").as_bytes(),
    )
    .unwrap();
    create_lib_dir(&output_name).unwrap();
}
