#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;
use std::collections::HashMap;


fn tokenize(file_contents: String) -> Vec<&'static str> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let mut result = Vec::new();
    result.push("hi");

    let res_words = HashMap::from([
        ("and", "AND" ),
        ("class", "CLASS"),
        ("else", "ELSE"),
        ("false", "FALSE" ),
        ("for", "FOR"),
        ("fun", "FUN"),
        ("if", "IF" ),
        ("nil", "NIL"),
        ("or", "OR" ),
        ("print", "PRINT"),
        ("return", "RETURN"), 
        ("super", "SUPER"),
        ("this", "THIS"),
        ("true", "TRUE"),
        ("var", "VAR"),
        ("while", "WHILE"),
    ]);
    let mut err_exists = false;
    let mut str_iter = file_contents.chars().peekable();
    let mut new_line = 1;

    if !file_contents.is_empty() {
        while let Some(ch) = str_iter.next() { // Option<char>
            match ch {
                '(' => result.push("LEFT_PAREN ( null"),
                ')' => result.push("RIGHT_PAREN ) null"),
                '{' => result.push("LEFT_BRACE {{ null"),
                '}'=> result.push("RIGHT_BRACE }} null"),
                '.' => result.push("DOT . null"),
                ',' => result.push("COMMA , null"),
                '+' => result.push("PLUS + null"),
                '*'=> result.push("STAR * null"),
                '-' => result.push("MINUS - null"),
                '/' => {
                    if str_iter.peek() == Some(&'/'){  
                        while let Some(new_ch) = str_iter.next(){
                            if new_ch == '\n'{
                                new_line+=1;
                                break;
                            }
                        };
                    }else{  
                        result.push!("SLASH / null");
                    }
                },
                '"' => {
                    let mut lexeme = '"'.to_string(); // takes &temp and creates new mem add with modifiable string
                    let mut literal = String::new();
                
                    while let Some(new_ch) = str_iter.next(){
                        if new_ch == '"' {
                            lexeme.push(new_ch);
                            break;
                        };
                        lexeme.push(new_ch);// "abcd...
                        literal.push(new_ch);//abcd... 
                    }; // once reaching None, will have the strings. 
                    if !lexeme.ends_with('"'){
                        err_exists = true;
                        result.push("[line {}] Error: Unterminated string.", new_line);
                    }else{
                        result.push("STRING {} {}", lexeme, literal);
                    }
                },
                ';' => result.push("SEMICOLON ; null"),
                '=' => {
                    if str_iter.peek() == Some(&'='){ // does NOT consume the next value. & finds address of equal, * refrences the address created by &
                        let _: Option<char> = str_iter.next();
                        result.push("EQUAL_EQUAL == null");
                    }else{  
                        result.push("EQUAL = null");
                    }
                },
                '!' => {
                    if str_iter.peek() == Some(&'='){ 
                        let _: Option<char> = str_iter.next();
                        result.push("BANG_EQUAL != null");
                    }else{  
                        result.push("BANG ! null");
                    }
                }
                '>' => {
                    if str_iter.peek() == Some(&'='){ 
                        let _: Option<char> = str_iter.next();
                        result.push("GREATER_EQUAL >= null");
                    }else{  
                        result.push("GREATER > null");
                    }
                },
                '<' => {
                    if str_iter.peek() == Some(&'='){ // automatically derefences equal, so chars are compared. 
                        let _: Option<char> = str_iter.next();
                        result.push("LESS_EQUAL <= null");
                    }else{  
                        result.push("LESS < null");
                    }
                },
                ' ' | '\t' =>{},
                '\n' =>{
                    new_line+=1;
                },
                _ => {
                    if ch.is_digit(10){ // finding numbers
                        let mut literal = ch.to_string();
                        while let Some(new_ch) = str_iter.peek(){ //Option<&char>
                            if new_ch.is_digit(10) || *new_ch == '.'{
                                literal.push(*new_ch);
                                let _ : Option<char> = str_iter.next();
                            }else{
                                break;
                            }
                        };
                        if let Ok(value) = literal.parse::<f64>(){
                            result.push("NUMBER {} {:?}", literal, value);
                        };
                        
                    }else if ch == '_' || ch.is_ascii_alphabetic(){ //creating identifiers
                        let mut identifier = ch.to_string();
                        while let Some(new_ch) = str_iter.peek(){
                            if !new_ch.is_digit(10) && !(*new_ch == '_') && !new_ch.is_ascii_alphabetic(){
                                break;
                            }
                            identifier.push(*new_ch);
                            let _ : Option<char> = str_iter.next();
                        };

                        if res_words.contains_key(&*identifier){
                            let reference = res_words[&*identifier];
                            result.push("{} {} null", reference, identifier);
                        }else{
                            result.push("IDENTIFIER {} null", identifier);
                        }
                    }else{
                        err_exists = true;
                        result.push("[line {}] Error: Unexpected character: {}", new_line, ch);
                    }
                }
            } // match ends. 
        }
        result.push("EOF  null");
        if err_exists{
            return result
            // return ExitCode::from(65);
        }
    } else {
        result.push("EOF  null");
    }// marking the end of file
     // want to return the normal. 
    
    return result;
    // return ExitCode::from(0);
}


fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::from(1);
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| { // same as catching error
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "parse" => { // iterator. 
            let tokens = tokenize(file_contents);  
            println!("{:?}", tokens);
            return ExitCode::from(1)
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}

// h