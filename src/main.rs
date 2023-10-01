use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use glob::glob;

mod ast;
mod fmt;
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
    Run {
        file: PathBuf,
    },

    /// Run a lispy file
    Repl,

    // Format all lispy files
    Fmt,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file }) => {
            let contents =
                fs::read_to_string(file).expect("Should have been able to read the file");
            let tokens = tokenizer::tokenize(&contents);
            let tree = ast::generate(&tokens);

            let mut state = interpreter::State::new();

            for ast in tree {
                interpreter::run(ast, &mut state);
            }
        }
        Some(Commands::Repl) => {
            let mut input = String::new();
            let mut state = interpreter::State::new();

            loop {
                print!("lispy> ");
                stdout().flush().unwrap();

                stdin().read_line(&mut input).expect("Failed to read line");

                let tokens = tokenizer::tokenize(&input);
                let tree = ast::generate(&tokens);

                let print = ast::Ast::Operation {
                    operator: "print".to_string(),
                    operands: tree,
                };

                interpreter::run(print, &mut state);

                input.clear();
            }
        }
        Some(Commands::Fmt) => {
            for entry in glob("**/*.l").expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        let contents = fs::read_to_string(path.clone())
                            .expect("Should have been able to read the file");
                        let tokens = tokenizer::tokenize(&contents);
                        let tree = ast::generate(&tokens);

                        let mut result = String::new();

                        for ast in tree {
                            result.push_str(fmt::format(ast).as_str());
                            result.push('\n');
                        }

                        if contents == result {
                            continue;
                        }

                        println!("[fmt] {}", path.display());

                        match fs::write(path.clone(), result) {
                            Ok(_) => {}
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        None => {}
    }
}
