use clap::Parser;

/// A simple CLI tool
#[derive(Parser)]
struct Cli {
    /// Name of the person to greet
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}