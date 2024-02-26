mod bindgen;
mod pathgen;
mod structgen;

use std::path::PathBuf;

use clap::Parser;

/// The argument that Thanix expects to get given via the cli.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "output")]
    output: PathBuf,
    /// Path to a YAML schema file.
    input: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    // Welcome Message
    println!(
        "{} \n(c) The Nazara Project. (github.com/The-Nazara-Project)\n
        Licensed under the terms of the GPL-v3.0-License.\n\
        Check github.com/The-Nazara-Project/Thanix/LICENSE for more info.\n",
        include_str!("templates/ascii_art.template")
    );

    match args.input {
        Some(file) => bindgen::gen(file, args.output),
        None => println!("Error: You need to provide a YAML schema to generate from."),
    }
}
