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
            let mut newLine = 1;

            if !file_contents.is_empty() {
                while let Some(ch) = str_iter.next() { // Option<char>
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
                                    if newCh == '\n'{
                                        newLine+=1;
                                        break;
                                    }
                                };
                            }else{  
                                println!("SLASH / null");
                            }
                        },
                        '"' => {
                            let mut lexeme = '"'.to_string(); // takes &temp and creates new mem add with modifiable string
                            let mut literal = String::new();
                        
                            while let Some(newCh) = str_iter.next(){
                                if newCh == '"' {
                                    lexeme.push(newCh);
                                    break;
                                };
                                lexeme.push(newCh);// "abcd...
                                literal.push(newCh);//abcd... 
                            }; // once reaching None, will have the strings. 
                            if !lexeme.ends_with('"'){
                                err_exists = true;
                                eprintln!("[line {}] Error: Unterminated string.", newLine);
                            }else{
                                println!("STRING {} {}", lexeme, literal);
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
                            if str_iter.peek() == Some(&'='){ // automatically derefences equal, so chars are compared. 
                                let _: Option<char> = str_iter.next();
                                println!("LESS_EQUAL <= null");
                            }else{  
                                println!("LESS < null");
                            }
                        },
                        ' ' | '\t' =>{},
                        '\n' =>{
                            newLine+=1;
                        },
                        _ => {
                            if ch.is_digit(10){
                                let mut literal = ch.to_string();
                                while let Some(val) = str_iter.peek(){ //Option<&char>
                                    if val.is_digit(10) || *val == '.'{
                                        let str_iter.next().unwrap();
                                        literal.push(newCh);
                                    }else{
                                        break;
                                    }
                                };
                                if let Ok(value) = literal.parse::<f64>(){
                                    println!("NUMBER {} {}", literal, value);
                                };
                            }else{
                                err_exists = true;
                                eprintln!("[line {}] Error: Unexpected character: {}", newLine, ch);
                            }
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
