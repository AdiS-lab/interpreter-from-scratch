#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;
use std::collections::HashMap;


struct Parser{
    tokens: Vec<String>,
    current: usize
}
// enum is saying anything of this defined type is allowed. 
// when populating 


#[derive(Debug)]                                                                                                                                                   
enum Expr{
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
}
#[derive(Debug)]                                                                                             
enum Lit{
    String(String),
    Bool(bool),
    Nil,
    F64(f64),
    I32(i32)
}


// Some v None => wrapping data in an an enum such that one can discern if that data is there or not. if not there 
// would manifest in...
// Ok v Errs

impl Parser{
    fn equality(&mut self) -> Result<Expr, String> {
        let left = self.comparison()?;
        let tk_type = self.peek(); // &str
        if matches!(tk_type.as_str(), "BANG_EQUAL" | "EQUAL_EQUAL"){
            let operator = self.consume(); // operator = String
            let right = self.comparison()?; // loop
            // return Ok(format!("({} {} {})", operator, left, right));
            return Ok(Expr::Binary(Box::new(left), operator, Box::new(right)))
        }else{
            // return Ok(left)
            return Ok(left)
        };
    }

    fn comparison(&mut self) -> Result<Expr, String>{
        let mut left = self.add()?;
        let mut tk_type = self.peek();

        while matches! (tk_type.as_str(), "GREATER_EQUAL" | "GREATER" |  "LESS" | "LESS_EQUAL"){
            let operator = self.consume();
            let right = self.add()?;
            // built_str = format!("({} {} {})", operator, built_str, right);
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        };
        // return Ok(built_str)
        return Ok(left)
        
        
    }

    fn add(&mut self) -> Result<Expr, String>{
        let mut left = self.mult()?; // starts as left
        let mut tk_type = self.peek();
        while matches!(tk_type.as_str(), "PLUS" | "MINUS"){ 
            let operator = self.consume();
            let right = self.mult()?; 
            // built_str = format!("({} {} {})", operator, built_str, right); // if mult (/ (* 3 2 ) 5)
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        }
        return Ok(left)
    }

    fn mult(&mut self) -> Result<Expr, String>{
        let mut left = self.unary()?; //  num or String
        let mut tk_type = self.peek();

        while matches!(tk_type.as_str(), "STAR" | "SLASH"){
            let operator = self.consume(); // * 
            let right = self.unary()?; // num or String
            // built_str = format!("({} {} {})", operator, built_str, right);
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        }
        return Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String>{
        let tk_type = self.peek();
        if matches!(tk_type.as_str(), "MINUS" | "BANG"){
            let operator = self.consume();
            let right = self.literal()?;
            // let mut build_str = String::new();
            let unary = Expr::Unary(operator, Box::new(right));
            return Ok(unary)
        }
        // while matches!(tk_type.as_str(), "MINUS" | "BANG"){
        //     let operator = self.consume();     
        //     let right = self.literal()?; // if error will just propogate up. if not then return an  so can unwrap right. 
        //     // build_str.push_str(&format!("({} {})", operator, right)); // should be ! then ! then true
        //     left = Expr::Unary(operator, new(right))
        //     tk_type = self.peek();
        // } 
        // // return build_str
        // return left
        let result = self.literal()?;
        return Ok(result)
    }

    // has to be a Result, and then unary will catch immediately through question mark. 
    fn literal(&mut self) -> Result<Expr, String> { 
        let tk_type = self.peek();

        let curr_tok = self.tokens[self.current].to_string();
        let tk_arr: Vec<&str> = curr_tok.split(" ").collect(); // Vec<&str>

        if matches!(tk_type.as_str(), "NUMBER" | "NIL"){
            if self.tokens.len() > 2 {  
                let f: f64 = tk_arr.get(2).unwrap_or(&"").to_string().parse().unwrap();
                self.current += 1;
                return Ok(Expr::Literal(Lit::F64(f)))
            }else{
                let i: i32 = self.consume().parse().unwrap();
                return Ok(Expr::Literal(Lit::I32(i)))
            };
        }else if matches!(tk_type.as_str(), "STRING"){
            let s =  curr_tok.split('"').nth(1).unwrap().to_string(); // &str --> String
            self.current += 1;
            return Ok(Expr::Literal(Lit::String(s)))

        }else if matches!(tk_type.as_str(), "TRUE" | "FALSE"){
            let b: bool = self.consume().parse().unwrap();
            return Ok(Expr::Literal(Lit::Bool(b)))

        }else if matches!(tk_type.as_str(), "BANG" | "MINUS"){
            let result = self.unary()?;
            return Ok(result) // will  be a Unary expr
        }else if matches!(tk_type.as_str(), "LEFT_PAREN"){
            // let middle = "(group";
            _ = self.consume(); // consumes (
            let right = self.equality()?;
            let curr = self.consume(); 
            // return Ok(format!("{} {})", middle, right))
            return Ok(Expr::Grouping(Box::new(right)))    
        }else{
            //assuming that will never be PAST EOF
            return Err( format!("[line 1] Error at '{}': Expect expression.", self.consume() )) // when reaching end 
        }
    } 

    fn consume(&mut self) -> String {
        let curr_tok = self.tokens[self.current].to_string();
        let tk_arr: Vec<&str> = curr_tok.split(" ").collect(); // Vec<&str>
        let &tk_type = tk_arr.get(0).unwrap();
        self.current += 1;
        return tk_arr.get(1).unwrap_or(&"").to_string() // &str --> String
    }
    fn peek(&mut self) -> String {
        let curr_tok = self.tokens[self.current].to_string(); 
        let words: Vec<&str> = curr_tok.split(" ").collect();
        let curr_type = words.get(0).unwrap_or(&"").to_string(); 
        return curr_type
    }
}




fn tokenize(file_contents: String) -> (String, String) {
    let mut result = String::new();
    let mut eresult = String::new();

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
    let mut str_iter = file_contents.chars().peekable();
    let mut new_line = 1; //  have to do something with this that allows the next thing to see it

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
                        eresult.push_str(&format!("[line {}] Error: Unterminated string.,", new_line));
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
                        eresult.push_str(&format!("[line {}] Error: Unexpected character: {},", new_line, ch));
                    }
                }
            } // match ends. 
        }// while ends
        result.push_str("EOF  null"); 
        return (result, eresult)
    }// if ends
    result.push_str("EOF null");
    return (result, eresult)
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
            let (token_str, err_str) = tokenize(file_contents);
            let tokens: Vec<String>= token_str.split(",").map(|s| s.to_string()).collect(); 
        
            for i in &tokens{
                println!("{}", i);
            };

            if !err_str.is_empty() {
                let etokens: Vec<String>= err_str.split(",").map(|s| s.to_string()).collect();
                for j in &etokens{
                   eprintln!("{}", j);
                };
                return ExitCode::from(65)
            };

            if token_str.len() == 1{
                return ExitCode::from(1)
            }

            return ExitCode::from(0)
           
        },"parse" => {
            let (token_str, err_str) = tokenize(file_contents); 
            let tokens: Vec<String>= token_str.split(",").map(|s| s.to_string()).collect(); 
            if token_str.len() == 1{
                return ExitCode::from(65)
            }
            let mut parser = Parser{tokens, current: 0};
            let result = match parser.equality(){
                Ok(val) => println!("{:?}", val),
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65)
                }  
            };
            return ExitCode::from(0)
        }, "evaluate" =>{
            let (token_str, err_str) = tokenize(file_contents); // NUMBER 50 50.0, EOF null
            let tokens: Vec<String>= token_str.split(",").map(|s| s.to_string()).collect(); // ["NUMBER 50 50.0 ", "EOF null"]
            let mut parser = Parser{tokens, current: 0};
            let result = match parser.equality(){ 
                Ok(val) => { // what is a moved value, w
                    match val{
                        Expr::Literal(lit) => {
                            if let Lit::F64(f) = lit {
                                println!("{:?}", f);
                            }else if let Lit::I32(i) = lit {
                                println!("{}", i);
                            }
                        },
                        Expr::Binary(l, o, r) => println!("{:?}", l),
                        Expr::Unary(l, o) => println!("{:?}", l),
                        Expr::Grouping(l) => println!("{:?}", l),
                    };
                },
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65) 
                } 
            };
            return ExitCode::from(0)
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}
