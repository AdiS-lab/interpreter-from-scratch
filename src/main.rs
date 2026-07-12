#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;

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
            let mut str_iter = file_contents.chars().peekable();
            if !file_contents.is_empty() {
                while let Some(ch) = str_iter.next() { // automatically creates an iteratable
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
                        '/' => {
                            if str_iter.peek() == Some(&'/'){  
                                while let Some(newCh) = str_iter.next(){

                                };
                            }else{  
                                println!("SLASH / null");
                            }
                        },
                        ';' => println!("SEMICOLON ; null"),
                        '=' => {
                            if str_iter.peek() == Some(&'='){ // does NOT consume the next value. & finds address of equal, * refrences the address created by &
                                let _: Option<char> = str_iter.next();
                                println!("EQUAL_EQUAL == null");
                            }else{  
                                println!("EQUAL = null");
                            }
                        },
                        '!' => {
                            if str_iter.peek() == Some(&'='){ 
                                let _: Option<char> = str_iter.next();
                                println!("BANG_EQUAL != null");
                            }else{  
                                println!("BANG ! null");
                            }
                        }
                        '>' => {
                            if str_iter.peek() == Some(&'='){ 
                                let _: Option<char> = str_iter.next();
                                println!("GREATER_EQUAL >= null");
                            }else{  
                                println!("GREATER > null");
                            }
                        },
                        '<' => {
                            println!("made it into < operator");
                            if str_iter.peek() == Some(&'='){
                                let _: Option<char> = str_iter.next();
                                println!("LESS_EQUAL <= null");
                            }else if str_iter.peek() == Some(&'|'){
                                println!("made it to correct check");
                                while str_iter.next() != Some('>'){}
                            }
                            else{  
                                println!("LESS < null");
                            }
                        },
                        '!' => {
                            println!("made it into pipe");
                        }
                        _ => {
                            println!(ch);
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
