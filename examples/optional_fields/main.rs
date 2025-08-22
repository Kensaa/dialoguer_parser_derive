use dialoguer_parser_derive::DialoguerParser;
// Clap's macros need to be in scope
use clap::{arg, command, Parser};

#[derive(Debug, DialoguerParser)]
#[command(name = "myapp", version = "1.0")]
struct Cli {
    #[arg(long)]
    #[prompt = "What is arg1 ?"]
    arg1: String,

    #[arg(long)]
    arg2: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("arg1 : {}, arg2 : {:?}", cli.arg1, cli.arg2);
}
