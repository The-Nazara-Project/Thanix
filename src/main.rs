use clap::Parser;

/// The argument that Thanix expects to get given via the cli.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Path to the input yaml you want to use.
    #[arg(short, long)]
    input_file: Option<String>
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
        "{} \n(c) The Nezara Project. (github.com/The-Nezara/Project)\n
        Licensed under the terms of the MIT-License.\n\
        Check github.com/The-Nazara-Project/Thanix/LICENSE for more info.\n",
        ascii_art
    );

}
