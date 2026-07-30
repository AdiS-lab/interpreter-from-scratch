use crate::types::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::statements::*;

pub struct Interpreter {
    pub scope: Vec<HashMap<String, Lit>>,
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: Expr) -> Result<Lit, String> {
        match expr{
            Expr::Literal(lit) => {
                if let Lit::Id(s) = lit {
                   return self.search_state(s)
                }
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
                let iter  = self.scope.iter_mut().rev();
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
            Expr::Call(id_expr, args) => {
                let call_type = self.evaluate(*id_expr)?;

                if let Lit::NativeFn(fn_name) = call_type{
                    match fn_name.as_str(){
                        "clock" => return Ok(Lit::F64(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as f64)),
                        _=> return Err("function not found".to_string())
                    }
                }else if let Lit::DefineFn(_, params, block_stmt, temp_scope) = call_type{


                    let mut i = 0;
                    if params.len() != args.len() {return Err("arguments do not match parameters".to_string())}
                    let real_scope = self.scope.clone();
                    self.scope = temp_scope;
                    self.scope.push(HashMap::new());

                    while i < params.len(){
                        let lit = self.evaluate(args[i].clone())?;  // Call --> [id, [expr1, expr2]]
                        self.scope.last_mut().unwrap().insert(params[i].clone(), lit); // DefineFn --> [id, ["arg1", "arg2"], blockStmt]
                        i+=1;
                    }
                    // inserting vars into the same scope as func. 

                    let val: Lit = ex_reg(*block_stmt, self)?;
                    self.scope.pop();
                    
                   if self.scope.len() > real_scope.len() {
                        self.scope = self.scope[..real_scope.len()].to_vec();
                    } else if self.scope.len() < real_scope.len() {
                        self.scope = [&self.scope[..], &real_scope[self.scope.len()..]].concat();
                    }

                    if let Lit::Return(return_res) = val { return Ok(*return_res); }
                    return Ok(Lit::Nil)
                }else{
                    return Err("function not found".to_string())
                }
            },   
        }
    }

    pub fn is_truthy(&mut self, lit: Lit) -> bool{
        if let Lit::Nil = lit{
            return false
        }
        if let Lit::Bool(false) = lit{
            return false
        }
        return true
    }

    fn search_state(&mut self, key: String)-> Result<Lit, String>{
        let iter= self.scope.iter().rev();
        for val in iter {
            if val.contains_key(&key){
                return Ok(val[&key].clone())
            };
        };
        return Err(format!("{} not found", key))
    }
}