mod bindgen;

use clap::Parser;

/// The argument that Thanix expects to get given via the cli.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Path to a YAML schema file.
    #[arg(short, long)]
    input_file: Option<String>,
    /// Name of the output package (Default 'output')
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let ascii_art = r#"
    ████████╗██╗  ██╗ █████╗ ███╗   ██╗██╗██╗  ██╗
    ╚══██╔══╝██║  ██║██╔══██╗████╗  ██║██║╚██╗██╔╝
       ██║   ███████║███████║██╔██╗ ██║██║ ╚███╔╝
       ██║   ██╔══██║██╔══██║██║╚██╗██║██║ ██╔██╗
       ██║   ██║  ██║██║  ██║██║ ╚████║██║██╔╝ ██╗
       ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝
                                                  "#;

    // Welcome Message
    println!(
        "{} \n(c) The Nazara Project. (github.com/The-Nazara-Project)\n
        Licensed under the terms of the GPL-v3.0-License.\n\
        Check github.com/The-Nazara-Project/Thanix/LICENSE for more info.\n",
        ascii_art
    );

    match args.input_file {
        Some(file) => bindgen::gen(file, args.name.unwrap_or("output".to_owned())),
        None => println!("Error: You need to provide a YAML schema to generate from."),
    }
}
