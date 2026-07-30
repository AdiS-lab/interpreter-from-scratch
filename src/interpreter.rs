use crate::types::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::statements::*;

pub struct Interpreter {
    pub scope: Vec<HashMap<String, Lit>>,
    pub current_scope: usize
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: Expr) -> Result<Lit, String> {
        // println!("=== SCOPES ({}) ===", self.scope.len());                                                                                                    
        // for (i, scope) in self.scope.iter().enumerate() {
        //     let keys: Vec<_> = scope.iter().map(|(k, v)| format!("{k}={v:?}")).collect();                                                                            
        //     println!("  [{}] {}", i, keys.join("           "));                                                                                                             
        // }
        // println!("===");

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
                }else if let Lit::DefineFn(_, params, block_stmt, index) = call_type{
                    let mut i = 0;
                    let new_index = index + 1;
                    self.current_scope = new_index;
                    self.scope.insert(new_index, HashMap::new()); // make a new scope right after index

                    if params.len() != args.len(){
                        return Err("mismatching args and params".to_string())
                    }
                    // add variables to the new scope right after index
                    while i < params.len(){
                        let lit = self.evaluate(args[i].clone())?;  // Call --> [id, [expr1, expr2]]
                        self.scope[self.current_scope].insert(params[i].clone(), lit); // DefineFn --> [id, ["arg1", "arg2"], blockStmt]
                        i+=1;
                    }


                    let val: Lit = ex_reg(*block_stmt, self)?;
                    self.scope.remove(self.current_scope);

                    self.current_scope = self.scope.len()-1;
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
        for i in (0..= self.current_scope).rev() {
            let val: HashMap<String, Lit> = self.scope[i].clone();
            if val.contains_key(&key){
                return Ok(val[&key].clone())
            };
        };
        return Err(format!("{} not found", key))
    }
}