#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;


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
                        // result.push_str(&format!("STRING {} {},", lexeme, literal));
                         result.push_str(&format!("STRING {} {},", lexeme, literal)); // literal. 
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
                            // result.push_str(&format!("{} {} null,", reference, identifier));
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

fn equality(it: &mut Peekable<Iter<String>>) -> String {
    let left = comparison(it);
    let tk_type = peekAhead(it); // &str

    if matches!(tk_type, "BANG_EQUAL" | "EQUAL_EQUAL"){
        let operator = consume(it); // operator = String
        let right = comparison(it); // loop
        return format!("({} {} {})", operator, left, right);
    }else{
        return left
    }
}

fn comparison(it: &mut Peekable<Iter<String>>) -> String{
    let left = add(it);
    let tk_type = peekAhead(it);

    if matches! (tk_type, "GREATER_EQUAL" | "GREATER" |  "LESS" | "LESS_EQUAL"){
        let operator = consume(it);
        let right = comparison(it);
        return format!("({} {} {})", operator, left, right)
    };
    return left
}

fn add(it: &mut Peekable<Iter<String>>) -> String{
    let mut built_str = mult(it); // starts as left
    let mut tk_type = peekAhead(it);
    while matches!(tk_type, "PLUS" | "MINUS"){ 
        let operator = consume(it);
        let right = mult(it);
        built_str.push_str(&format!("({} {} {})", operator, built_str, right)); // if mult (/ (* 3 2 ) 5)
        tk_type = peekAhead(it);
    }
    return built_str
}

fn mult(it: &mut Peekable<Iter<String>>) -> String{
    let mut built_str = unary(it); //  num or String
    let mut tk_type = peekAhead(it);

    while matches!(tk_type, "STAR" | "SLASH"){
        let operator = consume(it); // * 
        let right = unary(it); // num or String
        built_str.push_str(&format!("({} {} {})", operator, built_str, right));
        tk_type = peekAhead(it);
    }
    return built_str
}

fn unary(it: &mut Peekable<Iter<String>>) -> String{
    let mut tk_type = peekAhead(it);
    if matches!(tk_type, "MINUS" | "BANG"){
        let mut build_str = String::new();
        while matches!(tk_type, "MINUS" | "BANG"){
            let operator = consume(it);    
            let right = literal(it);
            build_str.push_str(&format!("({} {})", operator, right)); // should be ! then ! then true
            tk_type = peekAhead(it);
        } 
        return build_str
    }
    return literal(it)
}

fn literal(it: &mut Peekable<Iter<String>>) -> String{
    let tk_type = peekAhead(it);
    if matches!(tk_type, "NUMBER" | "TRUE" | "FALSE" |  "NIL" |  "STRING"){
        return consume(it)
    }else if matches!(tk_type, "BANG" | "MINUS"){
        return unary(it)
    }else if matches!(tk_type, "RIGHT_PAREN"){
        return String::new()
    }else if matches!(tk_type, "LEFT_PAREN"){
        let middle = "(group ";
        _ = consume(it); // consumes (
        let right = equality(it); // gets String, will throw inside if no ending
        _ = consume(it); // consume )
        return format!("{} {})", middle, right)
    }else if matches!(tk_type, "EOF"){
        return String::new()
    }else{
        return String::new()
    }
}  

fn consume(it: &mut Peekable<Iter<String>>) -> String {
    let current = it.next().unwrap(); // &String
    let tk_arr: Vec<&str> = current.split(" ").collect(); // Vec<&str>
    let &tk_type = tk_arr.get(0).unwrap();

    if tk_type == "STRING"{
        return current.split('"').nth(1).unwrap().to_string() // &str --> String
    }else if tk_type == "NUMBER"{
        return tk_arr.get(2).unwrap_or(&"").to_string() // &str --> String
    }else{
        return tk_arr.get(1).unwrap_or(&"").to_string() // &str --> String
    }
}

fn peekAhead<'a>(it: &mut Peekable<Iter<'a, String>>) -> &'a str {
    if let Some(&word) = it.peek(){
        let words: Vec<&str> = word.split(" ").collect(); // splits String into &str
        let new_type = *words.get(0).unwrap_or(&""); // Option<&&str> --> &str
        return new_type // iterator   is Vec<String> Option<&String> compared to Some(&String)
    }else{
        return ""
    }
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
            let tokenStr = tokenize(file_contents); // NUMBER 50 50.0, EOF null
            let tokens: Vec<String>= tokenStr.split(",").map(|s| s.to_string()).collect(); // ["NUMBER 50 50.0 ", "EOF null"]
            let mut token_iter = tokens.iter().peekable();
            let result = equality(&mut token_iter); // pass this inside 
            println!("{}", result);
            return ExitCode::from(0)
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}