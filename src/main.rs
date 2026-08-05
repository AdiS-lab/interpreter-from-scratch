use std::env;
use std::fs;
use std::process::ExitCode;

mod helpers;
mod interpreter;
mod parser;
mod statements;
mod tokenizer;
mod types;

use interpreter::Interpreter;
use parser::{Parser, parse};
use statements::execute;
use tokenizer::tokenize;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::from(1);
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let (tokens, err) = tokenize(file_contents);
            for token in &tokens {
                println!("{}", token)
            }

            if !err.is_empty() {
                for err_message in err {
                    eprintln!("{}", err_message)
                }
                return ExitCode::from(65);
            };

            return ExitCode::from(0);
        }
        "parse" => {
            let (tokens, _) = tokenize(file_contents);
            if tokens.len() == 1 {
                return ExitCode::from(65);
            }

            let mut parser: Parser = Parser {
                tokens,
                curr_pos: 0,
            };
            match parser.equality() {
                Ok(tree) => {
                    let ast_tree: String = parse(tree);
                    println!("{}", ast_tree);
                }
                Err(error) => {
                    eprintln!("{}", error);
                    return ExitCode::from(65);
                }
            };
            return ExitCode::from(0);
        }
        "evaluate" => {
            let (tokens, _) = tokenize(file_contents);
            let mut parser = Parser {
                tokens,
                curr_pos: 0,
            };
            match parser.equality() {
                Ok(tree) => {
                    let mut interpreter: Interpreter = Interpreter::new();
                    match interpreter.evaluate(tree) {
                        Ok(ast_tree) => println!("{}", ast_tree),
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::from(70);
                        }
                    };
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65);
                }
            };
            return ExitCode::from(0);
        }
        "run" => {
            let (tokens, _) = tokenize(file_contents);
            let mut parser = Parser {
                tokens,
                curr_pos: 0,
            };
            match parser.declaration() {
                Ok(tree) => {
                    let mut interpreter: Interpreter = Interpreter::new();
                    // dbg!("interpreter initialized");
                    match execute(tree, &mut interpreter) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::from(70);
                        }
                    }
                }

                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65);
                }
            };
            return ExitCode::from(0);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1);
        }
    }
}
