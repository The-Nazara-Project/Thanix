use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml::Value;

fn main() {
    let manifest_path = Path::new("Cargo.toml");

    let content =
        std::fs::read_to_string(manifest_path).expect("Unable to read manifest to string.");
    let parsed_toml: Value = content.parse().expect("Failed to parse toml!");

    let version = parsed_toml["package"]["version"]
        .as_str()
        .unwrap_or("0.0.0");

    let version_file_path = Path::new("src").join("version.rs");
    let mut file = File::create(version_file_path).expect("Unable to create version.rs!");
    writeln!(
        file,
        "#[allow(dead_code)]\npub const VERSION: &str = \"{}\";",
        version
    )
    .expect("Unable to write to file!");
}

