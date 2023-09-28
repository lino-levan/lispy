use std::fs;
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
        None => {}
    }
}
