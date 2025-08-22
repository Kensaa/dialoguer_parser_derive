use dialoguer_parser_derive::DialoguerParser;
// Clap's macros need to be in scope
use clap::{arg, command, Parser};

#[derive(Debug, DialoguerParser)]
#[command(name = "myapp", version = "1.0")]
struct Cli {
    #[arg(short, long)]
    #[prompt = "What is your name?"]
    name: String,

    #[arg(short, long)]
    #[prompt = "How old are you?"]
    age: u32,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello {}, age {}", cli.name, cli.age);
}
