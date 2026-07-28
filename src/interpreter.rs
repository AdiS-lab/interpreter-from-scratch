use crate::types::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Interpreter {
    pub scope: Vec<HashMap<String, Lit>>,
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: Expr) -> Result<Lit, String> {
          println!("=== SCOPES ({}) ===", self.scope.len());                                                                                                    
        for (i, scope) in self.scope.iter().enumerate() {
            let keys: Vec<_> = scope.iter().map(|(k, v)| format!("{k}={v:?}")).collect();                                                                            
            println!("  [{}] {}", i, keys.join(", "));                                                                                                             
        }
        println!("===");

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
                let res: Lit = self.evaluate(*expr)?; // total + 1
                let iter: std::iter::Rev<std::slice::IterMut<'_, HashMap<String, Lit>>> = self.scope.iter_mut().rev();
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
            Expr::Call(id, args) => {
                let call_type: Lit = self.search_state(id)?;
                if let Lit::NativeFn(fn_name) = call_type{
                    match fn_name.as_str(){
                        "clock" => return Ok(Lit::F64(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as f64)),
                        _=> return Err("function not found".to_string())
                    }
                }else if let Lit::DefineFn(_, params, block_stmt) = call_type{
                    let mut i = 0;
                    self.scope.push(HashMap::new());
                    if params.len() != args.len(){
                        return Err("mismatching args and params".to_string())
                    }
                    while i < params.len(){
                        let lit = self.evaluate(args[i].clone())?;
                        self.scope.last_mut().unwrap().insert(params[i].clone(), lit);
                        i+=1;
                    }
                    // let Stmt::Block(v ) = *block_stmt.clone() else { return Err("make sure to add braces or an arrow".to_string())};
                    let val: Lit = ex_reg(*block_stmt, self)?;
                    self.scope.pop();
                    let Lit::Nil = val else{return Ok(val)};
                    return Ok(Lit::Nil)
                }else{
                    return Err("function not found".to_string())
                }
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


pub fn execute(list: Vec<Declr>, interpreter: &mut Interpreter) -> Result<Lit, String> {
    for declaration in list{
        if let Declr::VarDeclr(id, stmt) = declaration {
            ex_var(id, stmt, interpreter)?;
        }else if let Declr::FunDeclr(id, parameters, stmt) = declaration{
            add_function(id, parameters, stmt, interpreter)
        }else if let Declr::Reg(stmt) = declaration{
            let val: Lit = ex_reg(stmt, interpreter)?; // check for nil
            let Lit::Nil = val else{return Ok(val)};
        }
    };
    return Ok(Lit::Nil)
}

pub fn ex_reg(stmt: Stmt, interpreter: &mut Interpreter)->Result<Lit, String>{
    if let Stmt::Print(expr) = stmt{
        let val: Lit = interpreter.evaluate(expr)?;
        println!("{}", val);
    }else if let Stmt::Block(list) = stmt{
        interpreter.scope.push(HashMap::new());
        let val: Lit = execute(list, interpreter)?;
        interpreter.scope.pop();
        let Lit::Nil = val else{return Ok(val)};

    }else if let Stmt::Other(expr) = stmt{ // only hits here on implicit returns
        let val: Lit = interpreter.evaluate(expr)?;

    }else if let Stmt::ReturnStmt(expr) = stmt{ // only hits here on returns
        return interpreter.evaluate(expr);

    }else if let Stmt::IfChain(conditional, then_stmt, else_stmt) = stmt{ 
        let val: Lit = interpreter.evaluate(conditional)?;
        let b: bool = interpreter.is_truthy(val.clone());
        if b{
            ex_reg(*then_stmt, interpreter)?;  
        }else{
            ex_reg(*else_stmt, interpreter)?;  
        }

    }else if let Stmt::WhileStmt(conditional, stmt) = stmt{
        let mut res = interpreter.evaluate(conditional.clone())?;

        while interpreter.is_truthy(res) { 
            ex_reg(*stmt.clone(), interpreter)?;
            res = interpreter.evaluate(conditional.clone())?;
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
            ex_reg(*stmt.clone(), interpreter)?; 
            match incr{
                Expr::Literal(Lit::Nil) => {},
                _=> { interpreter.evaluate(incr.clone())?; }
            }
            val = interpreter.evaluate(condition.clone())?;
        };
    }
    return Ok(Lit::Nil)
}

pub fn ex_var(id: String, stmt: Stmt, interpreter: &mut Interpreter) -> Result<(), String>{
  if let Stmt::Other(expr) = stmt { 
    let val: Lit = interpreter.evaluate(expr)?;
    interpreter.scope.last_mut().unwrap().insert(id, val);
  }
  return Ok(())
}

pub fn add_function(id: String, parameters: Vec<String>, stmt: Stmt, interpreter: &mut Interpreter){
    interpreter.scope.last_mut().unwrap().insert(id.clone(), Lit::DefineFn(id, parameters, Box::new(stmt)));
}



