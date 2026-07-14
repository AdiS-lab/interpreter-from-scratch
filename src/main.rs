#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;
use std::collections::HashMap;



fn tokenize(file_contents: String) -> String {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
    let mut result = String::new();

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
                '(' => result.push_str("LEFT_PAREN ( null,"),
                ')' => result.push_str("RIGHT_PAREN ) null,"),
                '{' => result.push_str("LEFT_BRACE {{ null,"),
                '}'=> result.push_str("RIGHT_BRACE }} null,"),
                '.' => result.push_str("DOT . null,"),
                ',' => result.push_str("COMMA , null,"),
                '+' => result.push_str("PLUS + null,"),
                '*'=> result.push_str("STAR * null,"),
                '-' => result.push_str("MINUS - null,"),
                '/' => {
                    if str_iter.peek() == Some(&'/'){  
                        while let Some(new_ch) = str_iter.next(){
                            if new_ch == '\n'{
                                new_line+=1;
                                break;
                            }
                        };
                    }else{  
                        result.push_str("SLASH / null,");
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
                        result.push_str(&format!("[line {}] Error: Unterminated string.,", new_line));
                    }else{
                        result.push_str(&format!("STRING {} {},", lexeme, literal));
                    }
                },
                ';' => result.push_str("SEMICOLON ; null,"),
                '=' => {
                    if str_iter.peek() == Some(&'='){ // does NOT consume the next value. & finds address of equal, * refrences the address created by &
                        let _: Option<char> = str_iter.next();
                        result.push_str("EQUAL_EQUAL == null,");
                    }else{  
                        result.push_str("EQUAL = null,");
                    }
                },
                '!' => {
                    if str_iter.peek() == Some(&'='){ 
                        let _: Option<char> = str_iter.next();
                        result.push_str("BANG_EQUAL != null,");
                    }else{  
                        result.push_str("BANG ! null,");
                    }
                }
                '>' => {
                    if str_iter.peek() == Some(&'='){ 
                        let _: Option<char> = str_iter.next();
                        result.push_str("GREATER_EQUAL >= null,");
                    }else{  
                        result.push_str("GREATER > null,");
                    }
                },
                '<' => {
                    if str_iter.peek() == Some(&'='){ // automatically derefences equal, so chars are compared. 
                        let _: Option<char> = str_iter.next();
                        result.push_str("LESS_EQUAL <= null,");
                    }else{  
                        result.push_str("LESS < null,");
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
                            result.push_str(&format!("NUMBER {} {:?},", literal, value));
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
                            result.push_str(&format!("{} {} null,", reference, identifier));
                        }else{
                            result.push_str(&format!("IDENTIFIER {} null,", identifier));
                        }
                    }else{
                        err_exists = true;
                        result.push_str(&format!("[line {}] Error: Unexpected character: {},", new_line, ch));
                    }
                }
            } // match ends. 
        }
        result.push_str("EOF  null");
        if err_exists{
            return result
            // return ExitCode::from(65);
        }
    } else {
        result.push_str("EOF  null");
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
        "tokenize" =>{
             // You can use print statements as follows for debugging, they'll be visible when running tests.

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
                        while let Some(new_ch) = str_iter.next(){
                            if new_ch == '\n'{
                                new_line+=1;
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
                        eprintln!( "[line {}] Error: Unterminated string.", new_line);
                    }else{
                        println!( "STRING {} {}", lexeme, literal);
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
                            println!( "NUMBER {} {:?}", literal, value);
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
                            println!( "{} {} null", reference, identifier);
                        }else{
                            println!( "IDENTIFIER {} null", identifier);
                        }
                    }else{
                        err_exists = true;
                        eprintln!( "[line {}] Error: Unexpected character: {}", new_line, ch);
                    }
                }
            } // match ends. 
        }
        println!("EOF  null");
        if err_exists{
            return ExitCode::from(65);
        }
    } else {
        println!("EOF  null");
    }// marking the end of file
     // want to return the normal. 
    
    return ExitCode::from(0);
        },"parse" => { // iterator. 
            let tokenStr = tokenize(file_contents);  
            let tokens: Vec<String>= tokenStr.split(",").map(|s| s.to_string()).collect(); // ["NUMBER etc ", "BRACKET {{"]
            let ind_tokens: Vec<&str> = tokens[0].split(" ").collect(); // gets first val 
            if let Some(&tk_type) = ind_tokens.get(0){
                match tk_type[0]{
                    "NUMBER" => println!("{}", tk_type[2]),
                    _ => println!("{}", tk_type[1])
                };
            };
            return ExitCode::from(0)
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}

// h