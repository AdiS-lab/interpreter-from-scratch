#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct Parser{
    tokens: Vec<String>,
    current: usize
}

// wrap var state inside interpreter so for now the whole program would reference
//the same variables

#[derive(Debug, Clone)]
enum Declr{
    VarDeclr(String, Stmt),
    FunDeclr(String, Vec<String>, Stmt),
    Reg(Stmt)
}

#[derive(Debug, Clone)]
enum Stmt{
    Print(Box<Stmt>),
    Other(Expr),
    Block(Vec<Declr>),
    IfChain(Expr, Box<Stmt>, Box<Stmt>),
    WhileStmt(Expr, Box<Stmt>),
    ForStmt(Box<Declr>, Box<Stmt>, Expr, Box<Stmt>),
    FunStmt(String, Vec<Expr>)
}

#[derive(Debug, Clone)]                                                                                                                                                   
enum Expr{
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    Assign(String, Box<Expr>),
    Operand(Box<Expr>, String, Box<Expr>)
}
#[derive(Debug, Clone)]                                                                                             
enum Lit{
    String(String),
    Bool(bool),
    Nil,
    F64(f64),
    Id(String),
    DeclrFn(Vec<String>, Box<Stmt>),
    NativeFn(String)
}

impl std::fmt::Display for Lit {            
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
          match self {
              Lit::F64(n) => write!(f, "{}", n),                                                                                                                                                                                                                                                        
              Lit::Bool(b) => write!(f, "{}", b),
              Lit::String(s) => write!(f, "{}", s),    
              Lit::Id(s) => write!(f, "{}", s),    
              Lit::Nil => write!(f, "nil"), 
              Lit::DeclrFn(v, s) => write!(f, "hello"), // change
              Lit::NativeFn(s) => write!(f, "hi")                                                                                                                                                                                                                                                         
          }
      }
}

impl Parser{
    fn block(&mut self) -> Result<Vec<Declr>, String>{
        let res: Vec<Declr> = self.declaration()?;
        let p: String = self.peek();
        if p.as_str() != "RIGHT_BRACE"{
            return Err("make sure to close block".to_string())
        }
        self.consume();
        return Ok(res);
    }

    fn fun_declr(&mut self) -> Result<Declr, String>{
        self.consume(); // fn
        let function_id: String = self.consume();
        self.consume(); // left paren
        let mut parameters: Vec<String> = Vec::new();
        while self.peek() != "RIGHT_PAREN" {
            if self.peek() != "COMMA"{
                let parameter: String = self.consume();
                parameters.push(parameter);
            }else{
                self.consume();
            }
        }
        self.consume(); // right paren
        let block_stmt: Stmt = self.statement()?;
        return Ok(Declr::FunDeclr(function_id, parameters, block_stmt))
    }

    fn var_declr(&mut self) -> Result<Declr, String>{
        let vars: String = self.consume();
        let id: String = self.consume();
        let operator: String = self.consume(); 
        match operator.as_str() {
            "=" =>  return Ok(Declr::VarDeclr(id, self.statement()?)),
            ";" => return Ok(Declr::VarDeclr(id, Stmt::Other(Expr::Literal(Lit::Nil)))),
            _ => return Err("bad syntax on var".to_string())
        }
    }

    fn declaration(&mut self)-> Result<Vec<Declr>, String> { 
        let mut tk_type: String = self.peek();
        let mut d: Vec<Declr> = Vec::<Declr>::new();
        while tk_type != "EOF" && tk_type != "RIGHT_BRACE"{
            if matches!(tk_type.as_str(), "VAR"){
                d.push(self.var_declr()?);
            }else if matches!(tk_type.as_str(), "FUN"){
                d.push(self.fun_declr()?)
            }else{
                let right: Stmt = self.statement()?;
                d.push(Declr::Reg(right));
            }
            tk_type = self.peek();
        }
        return Ok(d);
    }

    fn statement(&mut self) -> Result<Stmt, String>{
        let tk_type: String = self.peek();
        if matches!(tk_type.as_str(), "PRINT"){
            self.consume();
            let res: Stmt = self.statement()?;
            if self.consume() != ";"{
                return Err("line [1] make sure to include semicolon!".to_string()) 
            }
            return Ok(Stmt::Print(Box::new(res)));
        }else if matches!(tk_type.as_str(), "LEFT_BRACE") {
            self.consume(); // consume { 
            let res: Vec<Declr> = self.block()?; // call back 
            return Ok(Stmt::Block(res))
        }else if matches!(tk_type.as_str(), "IF"){
            self.consume();
            let open: String = self.consume(); 
            let condition: Expr = self.assignment()?;
            let close: String = self.consume();
            let then_st: Stmt = self.statement()?; 
            let mut else_st: Stmt = Stmt::Other(Expr::Literal(Lit::Nil));
            if self.peek() == "ELSE"{
                self.consume();
                else_st = self.statement()?;
            }
            return Ok( Stmt::IfChain(condition, Box::new(then_st), Box::new(else_st)) ) // should be an expr

        }else if matches!(tk_type.as_str(), "WHILE"){
            self.consume();
            let open: String = self.consume(); 
            let condition: Expr = self.assignment()?;
            let close: String = self.consume();
            let repeat = self.statement()?;
            return Ok(Stmt::WhileStmt(condition, Box::new(repeat)))
        }else if matches!(tk_type.as_str(), "FOR"){
            self.consume();
            let open: String = self.consume();  // ( var i = 1; i < 2; i + 1)
            let mut start: Declr = Declr::Reg(Stmt::Other(Expr::Literal(Lit::Nil)));

            if self.peek() == "VAR"{
                start = self.var_declr()?;
            }else if self.peek() == "SEMICOLON"{
                self.consume();
            }else{
                start = Declr::Reg(self.statement()?);
            }

            let range: Stmt = self.statement()?;
            let mut incr: Expr = Expr::Literal(Lit::Nil);
            if self.peek() != "RIGHT_PAREN"{
                incr = self.assignment()?;
            }

            let close: String = self.consume();
            let repeat: Stmt = self.statement()?;


            return Ok(Stmt::ForStmt(Box::new(start), Box::new(range), incr, Box::new(repeat))) 
        }else if matches!(tk_type.as_str(), "IDENTIFIER"){
            let id: String = self.consume();
            let mut arguments: Vec<Expr> = Vec::new();
            if self.peek() == "LEFT_PAREN"{
                self.consume();
                while self.peek()!="RIGHT_PAREN" {
                    if self.peek()!="COMMA"{
                        let expr: Expr = self.literal()?;
                        arguments.push(expr);
                    }else{
                        self.consume();
                    }
                };
                return Ok(Stmt::FunStmt(id,arguments))
            }else{
                return Err("function has to be callable".to_string())
            }
        }else{
            let result: Expr = self.assignment()?; // this is for var declaratiions.
            // println!("{:?}", result);
            if self.consume() != ";"{
                return Err("line [1] make sure to include semicolon!".to_string()) 
            }
            return Ok(Stmt::Other(result));
         }
    } 

    fn assignment(&mut self) -> Result<Expr, String> {
        let left: Expr = self.operands()?; 
        let tk_type = self.peek(); 
        if matches!(tk_type.as_str(), "EQUAL"){ 
            let operator: String = self.consume(); 
            if let Expr::Literal(Lit:: Id(id)) = left{
                let right = self.assignment()?;
                return Ok(Expr::Assign(id, Box::new(right))) 
            }
            return Err("make sure to include equal if re-defining identifier".to_string())
        }
        return Ok(left); 
    }

    fn operands(&mut self) -> Result<Expr, String>{
        let mut left = self.equality()?;
        let mut tk_type = self.peek();

        while matches!(tk_type.as_str(), "OR" | "AND"){ // false or true or true
            let operator: String = self.consume();
            let right = self.equality()?;
            left = Expr::Operand(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        }
        return Ok(left)
    }  

    fn equality(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison()?;
        let mut tk_type = self.peek(); 

        while matches!(tk_type.as_str(), "BANG_EQUAL" | "EQUAL_EQUAL"){
            let operator = self.consume();
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        }
        return Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, String>{
        let mut left = self.add()?;
        let mut tk_type = self.peek();

        while matches! (tk_type.as_str(), "GREATER_EQUAL" | "GREATER" |  "LESS" | "LESS_EQUAL"){
            let operator = self.consume();
            let right = self.add()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        };
        return Ok(left)  
    }

    fn add(&mut self) -> Result<Expr, String>{
        let mut left = self.mult()?; 
        let mut tk_type = self.peek();
        while matches!(tk_type.as_str(), "PLUS" | "MINUS"){ 
            let operator = self.consume();
            let right = self.mult()?; 
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            tk_type = self.peek();
        }
        return Ok(left)
    }

    fn mult(&mut self) -> Result<Expr, String>{
        let mut left = self.unary()?;
        let mut tk_type = self.peek();

        while matches!(tk_type.as_str(), "STAR" | "SLASH"){
            let operator = self.consume(); 
            let right = self.unary()?; 
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
            let unary = Expr::Unary(operator, Box::new(right));
            return Ok(unary)
        }
        let result = self.literal()?;
        return Ok(result)
    }

    fn literal(&mut self) -> Result<Expr, String> { 
        let tk_type = self.peek();
        let curr_tok = self.tokens[self.current].to_string();
        let tk_arr: Vec<&str> = curr_tok.split(" ").collect(); 
        if matches!(tk_type.as_str(), "NUMBER"){
            let f: f64 =  self.consume().parse().unwrap();
            return Ok(Expr::Literal(Lit::F64(f)))
        }else if matches!(tk_type.as_str(), "IDENTIFIER"){
            let s: String = self.consume();
            return Ok(Expr::Literal(Lit::Id(s)))
        }else if matches!(tk_type.as_str(), "STRING"){
            let s: String =  curr_tok.split('"').nth(1).unwrap().to_string(); 
            self.current += 1;
            return Ok(Expr::Literal(Lit::String(s)))
        }else if matches!(tk_type.as_str(), "TRUE" | "FALSE"){
            let b: bool = self.consume().parse().unwrap();
            return Ok(Expr::Literal(Lit::Bool(b)))
        }else if matches!(tk_type.as_str(), "NIL"){
            let s: String= self.consume();
            return Ok(Expr::Literal(Lit::Nil))
        }else if matches!(tk_type.as_str(), "BANG" | "MINUS"){
            let result: Expr = self.unary()?;
            return Ok(result)
        }else if matches!(tk_type.as_str(), "LEFT_PAREN"){
            _ = self.consume(); // consumes (
            let right = self.assignment()?;
            let right_paren = self.consume(); 
            if right_paren != ")"{
                return Err(format!("[line 1] Error at '{}': close parantheses.", right_paren)) 
            }
            return Ok(Expr::Grouping(Box::new(right)))    
        }else if matches!(tk_type.as_str(), "SEMICOLON"){
            return Err("line[1] missing some requirement".to_string())
        }else{
            return Err( format!("[line 1] Error at '{}': Expect expression.", self.consume() )) 
        } 
    }
    fn consume(&mut self) -> String {
        let curr_tok = self.tokens[self.current].to_string();
        let tk_arr: Vec<&str> = curr_tok.split(" ").collect(); 
        let &tk_type = tk_arr.get(0).unwrap();
        self.current += 1;
        return tk_arr.get(1).unwrap_or(&"").to_string() 
    }
    fn peek(&mut self) -> String {
        let curr_tok = self.tokens[self.current].to_string(); 
        let words: Vec<&str> = curr_tok.split(" ").collect();
        let curr_type = words.get(0).unwrap_or(&"").to_string(); 
        return curr_type
    }
}
fn tokenize(file_contents: String) -> (Vec<String>, Vec<String>) {
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
        ("print", "PRINT"),
        ("var", "VAR"),
    ]);
    let mut str_iter = file_contents.chars().peekable();
    let mut new_line: i32 = 1; //  have to do something with this that allows the next thing to see it
    let mut result:Vec<String> = Vec::new();
    let mut eresult:Vec<String> = Vec::new();


    if !file_contents.is_empty() {
        while let Some(ch) = str_iter.next() { // Option<char>
            match ch {
                '(' => result.push("LEFT_PAREN ( null".to_string()),
                ')' => result.push("RIGHT_PAREN ) null".to_string()),
                '{' => result.push( "LEFT_BRACE { null".to_string()),
                '}'=> result.push("RIGHT_BRACE } null".to_string()),
                '.' => result.push("DOT . null".to_string()),
                ',' => result.push("COMMA , null".to_string()),
                '+' => result.push("PLUS + null".to_string()),
                '*'=> result.push("STAR * null".to_string()),
                '-' => result.push("MINUS - null".to_string()),
                '/' => {
                    if str_iter.peek() == Some(&'/'){
                        while let Some(new_ch) = str_iter.next(){
                            if new_ch == '\n'{
                                new_line+=1;
                                break;
                            }
                        };
                    }else{  
                        result.push("SLASH / null".to_string());
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
                    }; 
                    if !lexeme.ends_with('"'){
                        eresult.push(format!("[line {}] Error: Unterminated string.", new_line));
                    }else{
                         result.push(format!("STRING {} {}", lexeme, literal)); 
                    }
                },
                ';' => result.push("SEMICOLON ; null".to_string()),
                '=' => {
                    if str_iter.peek() == Some(&'='){ // does NOT consume the next value. & finds address of equal, * refrences the address created by &
                        let _: Option<char> = str_iter.next();
                        result.push("EQUAL_EQUAL == null".to_string());
                    }else{  
                        result.push("EQUAL = null".to_string());
                    }
                },
                '!' => {
                    if str_iter.peek() == Some(&'='){ 
                        let _: Option<char> = str_iter.next();
                        result.push("BANG_EQUAL != null".to_string());
                    }else{  
                        result.push("BANG ! null".to_string());
                    }
                }
                '>' => {
                    if str_iter.peek() == Some(&'='){ 
                        let _: Option<char> = str_iter.next();
                        result.push("GREATER_EQUAL >= null".to_string());
                    }else{  
                        result.push("GREATER > null".to_string());
                    }
                },
                '<' => {
                    if str_iter.peek() == Some(&'='){ // automatically derefences equal, so chars are compared. 
                        let _: Option<char> = str_iter.next();
                        result.push("LESS_EQUAL <= null".to_string());
                    }else{  
                        result.push("LESS < null".to_string());
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
                            result.push(format!("NUMBER {} {:?}", literal, value));
                        };
                        
                    }else if ch == '_' || ch.is_ascii_alphabetic(){ //strings
                        let mut identifier = ch.to_string();
                        while let Some(new_ch) = str_iter.peek(){
                            if !new_ch.is_digit(10) && !(*new_ch == '_') && !new_ch.is_ascii_alphabetic(){
                                break;
                            }
                            identifier.push(*new_ch);
                            let _ : Option<char> = str_iter.next();
                        }; // typically pushes identifer in, but if we want print then want to get our string afger 


                        if res_words.contains_key(&*identifier){
                            let reference = res_words[&*identifier];
                            result.push(format!("{} {} null", reference, identifier)); 
                        }else{
                            result.push(format!("IDENTIFIER {} null", identifier));
                        }
                    }else{
                        eresult.push(format!("[line {}] Error: Unexpected character: {}", new_line, ch));
                    }
                }
            } // match ends. 
        }// while ends
        result.push("EOF  null".to_string()); 
        return (result, eresult)
    }// if ends
    result.push("EOF  null".to_string());
    return (result, eresult)
}


fn parse(val: Expr) -> String {
    match val{
        Expr::Literal(lit) => {
            if let Lit::F64(f) = lit { 
            return format!("{:?}", f);
            }else if let Lit::String(s) = lit{
            return format!("{}", s);
            }else if let Lit::Bool(b) = lit{
            return format!("{}", b);
            }else if let Lit::Nil = lit{
                return "nil".to_string()
            }
        },
        Expr::Binary(l , o, r) =>return format!("({} {} {})", o, parse(*l), parse(*r)),
        Expr::Unary(l, r) =>return format!("({} {})", l, parse(*r)),
        Expr::Grouping(l) =>return format!("(group {})", parse(*l)),
        Expr::Assign(s, expr) => return format!("({}{})",s, parse(*expr)),
        Expr::Operand(l, o, r) => return format!("({}{}{})", o, parse(*l), parse(*r))
    };    
    return "".to_string()
}

struct Interpreter {
    scope: Vec<HashMap<String, Lit>>,
}


impl Interpreter {
    fn evaluate(&mut self, expr: Expr) -> Result<Lit, String> {
        match expr{
            Expr::Literal(lit) => {
                if let Lit::Id(s) = lit {
                   return self.search_state(s)
                };
                return Ok(lit)
            },
            Expr::Binary(l , o, r) =>{
                let left: Lit = self.evaluate(*l)?; // always unpack
                let right: Lit = self.evaluate(*r)?;
                if matches!(o.as_str(), | "*" | "/" |"-" |">" | "<" | ">=" | "<="){
                    if let Lit::F64(n) = left && let Lit::F64(n2) = right{ 
                        match o.as_str(){
                            "*" => return Ok(Lit::F64(n*n2)),
                            "/"=> return Ok(Lit::F64(n/n2)),
                            "-" => return Ok(Lit::F64(n-n2)),
                            ">" => return Ok(Lit::Bool(n>n2)),
                            "<"=>  return Ok(Lit::Bool(n<n2)),
                            ">="=>  return Ok(Lit::Bool(n>=n2)),
                            "<=" =>  return Ok(Lit::Bool(n<=n2)),
                            _ => return Ok(Lit::Nil)
                        }
                    }else{
                        return Err("Operands must be numbers".to_string())
                    }
                }else{
                    match o.as_str(){
                        "+" => {
                            if let Lit::F64(n) = left && let Lit::F64(n2) = right{ 
                                return Ok(Lit::F64(n+n2))
                            }else if let Lit::String(s) = left && let Lit::String(s2) = right{
                                return Ok(Lit::String(format!("{}{}",s,s2)))
                            }
                            return Err("Operands must be strings/numbers".to_string())
                        },
                        "==" => { // case would be anything
                            if let Lit::F64(n) = left && let Lit::F64(n2) = right{
                                return Ok(Lit::Bool(n==n2))
                            }else if let Lit::Bool(b) = left && let Lit::Bool(b2) = right{
                                return Ok(Lit::Bool(b==b2))
                            }else if let Lit::String(s) = left && let Lit::String(s2) = right{
                                return Ok(Lit::Bool(s==s2))
                            }
                            return Ok(Lit::Bool(false))
                        },
                        "!="=>  {
                            if let Lit::F64(n) = left && let Lit::F64(n2) = right{
                                return Ok(Lit::Bool(n!=n2))
                            }else if let Lit::Bool(b) = left && let Lit::Bool(b2) = right{
                                return Ok(Lit::Bool(b!=b2))
                            }else if let Lit::String(s) = left && let Lit::String(s2) = right{
                                return Ok(Lit::Bool(s!=s2))
                            }
                            return Ok(Lit::Bool(true))
                        },
                        _ => return Ok(Lit::Nil)
                    }
                }
            },
            Expr::Unary(l, r) => { 
                let right = self.evaluate(*r)?;
                match l.as_str(){
                    "!"=> {
                        if let Lit::Bool(b) = right{
                            return Ok(Lit::Bool(!b))
                        }else if let Lit::Nil = right{
                            return Ok(Lit::Bool(true))
                        }
                        return Ok(Lit::Nil)
                    },
                    "-" => {
                        if let Lit::F64(f) = right{
                            return Ok(Lit::F64(-1.0 * f))
                        }
                        return Err("line[1] Operand must be a number.".to_string())
                    },
                    _=> return Ok(Lit::Nil)
                }
            }
            Expr::Grouping(l) => {
                return self.evaluate(*l)
            },
            Expr::Assign(k, expr) => { 
                let res = self.evaluate(*expr)?; 
                let iter = self.scope.iter_mut().rev();
                for vars in iter{
                    if vars.contains_key(&k){
                        vars.insert(k, res.clone()); 
                        return Ok(res)
                    };
                };
                return Err(format!("not found {}", k))
            },
            Expr::Operand(l, o , r) =>{ //false or false or true =>   false or true
                let left = self.evaluate(*l)?;
                let b = self.is_truthy(left.clone());
                if o == "and" && !b{
                    return Ok(left)
                }
                if o == "or" && b{
                    return Ok(left);
                }
                return self.evaluate(*r);
            },
        }   
    }

    fn is_truthy(&mut self, lit: Lit) -> bool{
        if let Lit::Nil = lit{
            return false
        }
        if let Lit::Bool(false) = lit{
            return false
        }
        return true
    }

    fn search_state(&mut self, key: String)-> Result<Lit, String>{
        let iter = self.scope.iter_mut().rev();
            for vars in iter{
                if vars.contains_key(&key){
                    return Ok(vars[&key].clone())
                };
            };
        return Err(format!("{} not found", key))
    }
}







fn execute(list: Vec<Declr>, interpreter: &mut Interpreter) -> Result<(), String> {
    // println!("{:?}", list);
    for declaration in list{
        if let Declr::VarDeclr(id, stmt) = declaration { // whether declaration for now HAS to be a simple expr
            ex_var(id, stmt, interpreter)?;
        }else if let Declr::FunDeclr(id, parameters, stmt) = declaration{
            add_function(id, parameters, stmt, interpreter)
        }else if let Declr::Reg(stmt) = declaration{
            ex_reg(stmt, interpreter)?;
        }
    };
    return Ok(())
}

fn ex_reg(stmt: Stmt, interpreter: &mut Interpreter)->Result<(), String>{
    if let Stmt::Print(print_statement) = stmt{
        if let Stmt::Other(expr) = *print_statement{
            let val: Lit = interpreter.evaluate(expr)?;
            println!("{}", val);
        }else{
            ex_reg(*print_statement, interpreter)?;
        }
    }else if let Stmt::Other(expr) = stmt{ 
        let val: Lit = interpreter.evaluate(expr)?;

    }else if let Stmt::Block(list) = stmt{
        interpreter.scope.push(HashMap::new());
        execute(list, interpreter)?;
        interpreter.scope.pop(); 

    }else if let Stmt::IfChain(conditional, then_stmt, else_stmt) = stmt{ 
        let val: Lit = interpreter.evaluate(conditional)?;
        let b: bool = interpreter.is_truthy(val.clone()); // if lit ain't that then true
        if b{
            ex_reg(*then_stmt, interpreter)?;  
        }else{
            ex_reg(*else_stmt, interpreter)?;  
        }

    }else if let Stmt::WhileStmt(c, stmt) = stmt{
        let mut res = interpreter.evaluate(c.clone())?;
        while interpreter.is_truthy(res) { 
            ex_reg(*stmt.clone(), interpreter)?;
            res = interpreter.evaluate(c.clone())?;

        };
        
    }else if let Stmt::ForStmt(var_init,range, incr, stmt) = stmt{
        if let Declr::VarDeclr(id, val) = *var_init{
            ex_var(id.clone(), val, interpreter)?; // create var with num
        }else if let Declr::Reg(stmt) = *var_init{
            ex_reg(stmt, interpreter)?;
        }
        let condition = if let Stmt::Other(c) = *range { c } else { Expr::Literal(Lit::Bool(true)) }; // condition
        let mut val = interpreter.evaluate(condition.clone())?; // range

        while interpreter.is_truthy(val){
            // println!("{:?}", interpreter.scope);
            ex_reg(*stmt.clone(), interpreter)?; 
            match incr{
                Expr::Literal(Lit::Nil) => {},
                _=> { interpreter.evaluate(incr.clone())?; }
            }
            val = interpreter.evaluate(condition.clone())?;
            // println!("{:?}",val);
        };
    }else if let Stmt::FunStmt(id, arguments) = stmt{
        let res: Lit = interpreter.search_state(id)?;
        if let Lit::DeclrFn(parameters, stmt) = res{ 
            let mut i = 0;
            while i < parameters.len(){
                ex_var(parameters[i].clone(), Stmt::Other(arguments[i].clone()), interpreter)?; // define vars in new scope
                i += 1;
            };
            ex_reg(*stmt, interpreter)?;
        }else if let Lit::NativeFn(s) = res{
            match s.as_str(){
                "clock" => {
                    println!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                    return Ok(())
                },
                _=> return Err("function not defined".to_string())
            }
        }
        else{
            return Err("function not defined".to_string())
        } 
    }
    return Ok(())
}

fn ex_var(id: String, stmt: Stmt, interpreter: &mut Interpreter) -> Result<(), String>{
  if let Stmt::Other(expr) = stmt { 
    let val: Lit = interpreter.evaluate(expr)?;
    interpreter.scope.last_mut().unwrap().insert(id, val);
  }
  return Ok(())
}

fn add_function(id: String, parameters: Vec<String>, stmt: Stmt, interpreter: &mut Interpreter){
    interpreter.scope.last_mut().unwrap().insert(id, Lit::DeclrFn(parameters, Box::new(stmt)));
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
            let (tokens , err) = tokenize(file_contents);
            for i in &tokens{ // reference rather than borrowing it
                println!("{}", i);
            };
            if !err.is_empty() {
                for j in err{
                   eprintln!("{}", j);
                };
                return ExitCode::from(65)
            };
            return ExitCode::from(0)
           
        },"parse" => {
            let (tokens, err_str) = tokenize(file_contents); 
            if tokens.len() == 1{
                return ExitCode::from(65)
            }

            let mut parser = Parser{tokens, current: 0};
            let result = match parser.equality(){
                Ok(val) => {
                    let tree_str: String = parse(val);
                    println!("{}", tree_str);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65)
                }  
            };
            return ExitCode::from(0)
        },"evaluate" => {
            let (tokens, err_str) = tokenize(file_contents); // NUMBER 50 50.0, EOF null
            let mut parser = Parser{tokens, current: 0};
            let result = match parser.equality(){
                Ok(val) => { 
                    let mut interpreter: Interpreter = Interpreter{scope: vec![HashMap::new()]};
                    let res = match interpreter.evaluate(val){
                        Ok(val)=> println!("{}", val),
                        Err(err) =>{
                            eprintln!("{}", err); // print expression
                            return ExitCode::from(70)
                        }
                    };
                },
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65) 
                } 
            };
            return ExitCode::from(0)
        },"run"=>{
            let (tokens, err_str) = tokenize(file_contents); // NUMBER 50 50.0, EOF null
            let mut parser = Parser{tokens, current: 0};
            match parser.declaration(){
                Ok(val)=>{
                    let mut scope: HashMap<String, Lit> = HashMap::new();
                    scope.insert("clock".to_string(), Lit::NativeFn("clock".to_string()));
                    let mut interpreter: Interpreter = Interpreter{scope: vec![scope]};

                    match execute(val, &mut interpreter){
                        Ok(val) => {},
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::from(70)
                        }
                    }
                },
                Err(e)=>{
                    eprintln!("{}", e);
                    return ExitCode::from(65)
                }
            };
            return ExitCode::from(0)
        }, _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}
