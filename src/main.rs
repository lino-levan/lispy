use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod ast;
mod interpreter;
mod tokenizer;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a lispy file
    Run { file: PathBuf },

    /// Run a lispy file
    Repl,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file }) => {
            let contents =
                fs::read_to_string(file).expect("Should have been able to read the file");
            let tokens = tokenizer::tokenize(contents);
            let tree = ast::generate(&tokens);

            for ast in tree {
                interpreter::run(ast);
            }
        }
        Some(Commands::Repl) => {
            let mut input = String::new();

            loop {
                print!("lispy> ");
                stdout().flush().unwrap();

                stdin().read_line(&mut input).expect("Failed to read line");

                let tokens = tokenizer::tokenize(input.clone());
                let tree = ast::generate(&tokens);

                for ast in tree {
                    interpreter::run(ast);
                }

                input.clear();
            }
        }
        None => {}
    }
}
