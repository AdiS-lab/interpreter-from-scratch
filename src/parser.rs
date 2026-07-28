use crate::types::*;

pub struct Parser{
    pub tokens: Vec<String>,
    pub current: usize
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

    pub fn declaration(&mut self)-> Result<Vec<Declr>, String> { 
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
            println!("{:?}", res);
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

    pub fn equality(&mut self) -> Result<Expr, String> {
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

pub fn parse(val: Expr) -> String {
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
