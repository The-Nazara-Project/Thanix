mod bindgen;
mod pathgen;
mod structgen;
mod util;

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
    /// Enable Workaround mode.
    /// Creates opinionated NetBox API client.
    /// Can help with unsanitary response data crashing deserialization by making API object fields optional, even though
    /// the YAML might state otherwise.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    workaround: bool,
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
        Some(file) => bindgen::generate(file, args.output, args.workaround),
        None => println!("Error: You need to provide a YAML schema to generate from."),
    }
}
