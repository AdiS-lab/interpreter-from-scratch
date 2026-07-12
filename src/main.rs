#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;
use std::iter::peekable

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::from(1);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| { // same as catching error
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut err_exists = false;
            let mut iter = file_contents.iter().peekable() // iteratable just creates a tracking struct
            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                while let Some(ch) = iter.next() {
                    match ch {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}'=> println!("RIGHT_BRACE }} null"),
                        '.' => println!("DOT . null"),
                        ',' => println!("COMMA , null"),
                        '+' => println!("PLUS + null"),
                        '*'=> println!("STAR * null"),
                         '-' => println!("MINUS - null"),
                        '/'=> println!("SLASH / null"),
                        ';' => println!("SEMICOLON ; null"),
                        '=' => {
                            let Some(next) = iter.peek() // does NOT consume the next value because creates a copy essentially
                            if next == "="{
                                println!("EQUAL_EQUAL = null"),
                            }else{ 
                                println!("EQUAL = null")
                            }
                        }
                        _ => {
                            err_exists = true;
                            eprintln!("[line 1] Error: Unexpected character: {}", ch);
                        }
                    } 
                }
                println!("EOF  null");
                if err_exists{
                    return ExitCode::from(65);
                }
            } else {
                println!("EOF  null");
            }
            return ExitCode::from(0)
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}
