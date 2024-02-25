use crate::pathgen;
use crate::structgen;
use openapiv3::Schema;
use openapiv3::SchemaKind;
use openapiv3::Type;
use openapiv3::{OpenAPI, ReferenceOr};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

/// Generate Rust bindings from an OpenAPI schema.
pub fn gen(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) {
    // Parse the schema.
    let input = fs::read_to_string(input_path).unwrap();
    let api: OpenAPI = serde_yaml::from_str(&input).unwrap();

    // Populate the output directory.
    let output_path = output_path.as_ref();
    create_lib_dir(output_path).unwrap();

    // Create and open the output file for structs.
    let mut types_file = File::create(output_path.join("src/").join("types.rs")).unwrap();
    // TODO: We don't really need these.
    //write!(types_file, "{}", include_str!("templates/usings.template")).unwrap();

    // For every component.
    for (name, schema) in &api.components.unwrap().schemas {
        let s = match schema {
            ReferenceOr::Item(x) => x,
            _ => continue,
        };
        // Generate struct and write it to file.
        if let Some(structure) = structgen::gen(name, s) {
            types_file.write_all(structure.as_bytes()).unwrap();
        }
    }

    // Create and open the output file for paths.
    let mut paths_file = File::create(output_path.join("src/").join("paths.rs")).unwrap();
    write!(paths_file, "{}", include_str!("templates/usings.template")).unwrap();

    // For every path.
    for (name, path) in &api.paths.paths {
        let p = match path {
            ReferenceOr::Item(x) => x,
            _ => continue,
        };
        // Generate paths and write to file.
        if let Some(paths) = pathgen::gen(name, p) {
            paths_file.write_all(paths.as_bytes()).unwrap();
        }
    }
}

/// Create all necessary structures and directories for the crate.
/// Requires `output` folder to exist.
///
/// # Arguments
///
/// - `output_name: &Path` - The name of the output library given by the CLI. Default `output`.
fn create_lib_dir(output_path: &Path) -> io::Result<()> {
    println!("Starting repackaging into crate...");

    // Create the output folder.
    _ = fs::create_dir(output_path);

    // Create the "src" subdirectory.
    let src_dir = output_path.join("src/");
    fs::create_dir_all(&src_dir)?;

    // Create the "src/util.rs" file.
    fs::write(
        &src_dir.join("util.rs"),
        include_str!("templates/util.rs.template"),
    )?;

    // Create the "src/lib.rs" file.
    fs::write(
        &src_dir.join("lib.rs"),
        include_str!("templates/lib.rs.template"),
    )?;

    // Create the "Cargo.toml" file.
    let mut cargo_file = fs::File::create(output_path.join("Cargo.toml"))?;
    write!(
        cargo_file,
        include_str!("templates/Cargo.toml.template"),
        // In case the user provides a relative path, use the last directory
        // as the crate name.
        output_path.file_name().unwrap().to_string_lossy()
    )?;

    // Create the "README.md" file.
    fs::write(
        output_path.join("README.md"),
        include_str!("templates/README.md.template"),
    )?;

    println!("Output successfully repackaged!");
    Ok(())
}

/// Makes a comment out of a given string.
pub fn make_comment(input: Option<String>, indent: usize) -> String {
    match input {
        Some(x) => x
            .split('\n')
            .map(|x| format!("{}/// {}\n", "\t".repeat(indent), x))
            .collect::<Vec<_>>()
            .concat(),
        None => String::new(),
    }
}

pub fn type_to_string(ty: &ReferenceOr<Schema>) -> String {
    match ty {
        // If the type is a reference, just extract the component name.
        ReferenceOr::Reference { reference } => reference.replace("#/components/schemas/", ""),
        ReferenceOr::Item(item) => {
            let mut base = match &item.schema_kind {
                SchemaKind::Type(t) => match t {
                    Type::String(_) => "String".to_owned(),
                    Type::Number(_) => "f64".to_owned(),
                    Type::Integer(_) => "i64".to_owned(),
                    // JSON object, but Rust has no easy way to support this, so just ask for a string.
                    Type::Object(_) => "String".to_owned(),
                    Type::Boolean(_) => "bool".to_owned(),
                    Type::Array(x) => {
                        let items = x.items.as_ref().unwrap().clone().unbox();
                        format!("Vec<{}>", type_to_string(&items))
                    }
                },
                // Very likely a JSON object.
                _ => "String".to_owned(),
            };
            // If property is nullable, we treat it as an optional argument.
            if item.schema_data.nullable {
                base = format!("Option<{}>", base);
            }
            base
        }
    }
}
