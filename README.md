# dialoguer_parser_derive

A custom derive macro that lets you build interactive CLI tools using both clap and dialoguer

It works like `#[derive(Parser)]` from `clap`, but automatically prompts the user for any missing arguments using `dialoguer`

## Exemple

```rust
use dialoguer_parser_derive::DialoguerParser;
// Clap's macros need to be in scope
use clap::{Parser, arg, command};

#[derive(Debug, DialoguerParser)]
#[command(name = "myapp", version = "1.0")]
struct Cli {
    #[arg(short, long)]
    #[prompt="What is your name?"]
    name: String,

    #[arg(short, long)]
    #[prompt="How old are you?"]
    age: u32,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello {}, age {}", cli.name, cli.age);
}
```
If --name or --age is not passed via CLI, the user is prompted for it interactively.

## Required Dependencies
In your project, add:
```Toml
[dependencies]
clap = { version = "4.5.40", features = ["derive"] }
dialoguer = "0.11.0"
dialoguer_parser_derive = { git = "https://github.com/Kensaa/dialoguer_parser_derive.git" }
```
You must import clap and dialoguer yourself. This crate only generates the glue code.

## Limitations
- Only works on structs with named fields (no tuple structs or enums)
- All fields must be non-`Option<T>` — prompting is used instead of optional values
- Prompts use dialoguer::Input (no support yet for password, select, confirm, etc), so that implies that only single value fields 
- This crate is very much experimental and was made for my personnal use so it's probably not working very well if not at all