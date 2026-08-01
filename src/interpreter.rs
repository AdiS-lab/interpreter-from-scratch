use crate::statements::{execute_stmt, new_scope};
use crate::types::{Lit, Expr, Env};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Interpreter {
    pub scope: Vec<Env>,
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: Expr) -> Result<Lit, String> {
        // println!("{:?}", &self.scope);
        match expr {
            Expr::Literal(lit) => {
                if let Lit::Id(s) = lit {
                    return self.search_scope(s);
                }
                return Ok(lit);
            }
            Expr::Binary(l, o, r) => {
                let left: Lit = self.evaluate(*l)?; // always unpack
                let right: Lit = self.evaluate(*r)?;
                if matches!(o.as_str(), |"*"| "/" | "-" | ">" | "<" | ">=" | "<=") {
                    if let Lit::F64(n) = left
                        && let Lit::F64(n2) = right
                    {
                        match o.as_str() {
                            "*" => return Ok(Lit::F64(n * n2)),
                            "/" => return Ok(Lit::F64(n / n2)),
                            "-" => return Ok(Lit::F64(n - n2)),
                            ">" => return Ok(Lit::Bool(n > n2)),
                            "<" => return Ok(Lit::Bool(n < n2)),
                            ">=" => return Ok(Lit::Bool(n >= n2)),
                            "<=" => return Ok(Lit::Bool(n <= n2)),
                            _ => return Ok(Lit::Nil),
                        }
                    } else {
                        return Err("Operands must be numbers".to_string());
                    }
                } else {
                    match o.as_str() {
                        "+" => {
                            if let Lit::F64(n) = left
                                && let Lit::F64(n2) = right
                            {
                                return Ok(Lit::F64(n + n2));
                            } else if let Lit::String(s) = left
                                && let Lit::String(s2) = right
                            {
                                return Ok(Lit::String(format!("{}{}", s, s2)));
                            }
                            return Err("Operands must be strings/numbers".to_string());
                        }
                        "==" => {
                            // case would be anything
                            if let Lit::F64(n) = left
                                && let Lit::F64(n2) = right
                            {
                                return Ok(Lit::Bool(n == n2));
                            } else if let Lit::Bool(b) = left
                                && let Lit::Bool(b2) = right
                            {
                                return Ok(Lit::Bool(b == b2));
                            } else if let Lit::String(s) = left
                                && let Lit::String(s2) = right
                            {
                                return Ok(Lit::Bool(s == s2));
                            }
                            return Ok(Lit::Bool(false));
                        }
                        "!=" => {
                            if let Lit::F64(n) = left
                                && let Lit::F64(n2) = right
                            {
                                return Ok(Lit::Bool(n != n2));
                            } else if let Lit::Bool(b) = left
                                && let Lit::Bool(b2) = right
                            {
                                return Ok(Lit::Bool(b != b2));
                            } else if let Lit::String(s) = left
                                && let Lit::String(s2) = right
                            {
                                return Ok(Lit::Bool(s != s2));
                            }
                            return Ok(Lit::Bool(true));
                        }
                        _ => return Ok(Lit::Nil),
                    }
                }
            }
            Expr::Unary(l, r) => {
                let right = self.evaluate(*r)?;
                match l.as_str() {
                    "!" => {
                        if let Lit::Bool(b) = right {
                            return Ok(Lit::Bool(!b));
                        } else if let Lit::Nil = right {
                            return Ok(Lit::Bool(true));
                        }
                        return Ok(Lit::Nil);
                    }
                    "-" => {
                        if let Lit::F64(f) = right {
                            return Ok(Lit::F64(-1.0 * f));
                        }
                        return Err("line[1] Operand must be a number.".to_string());
                    }
                    _ => return Ok(Lit::Nil),
                }
            }
            Expr::Grouping(l) => return self.evaluate(*l),
            Expr::Assign(k, expr) => {
                let res: Lit = self.evaluate(*expr)?; // total + 1
                let iter = self.scope.iter_mut().rev();
                for vars in iter {
                    if vars.borrow().contains_key(&k) {
                        vars.borrow_mut().insert(k, res.clone());
                        return Ok(res);
                    };
                }
                return Err(format!("not found {}", k));
            }
            Expr::Operand(l, o, r) => {
                let left = self.evaluate(*l)?;
                let b = self.is_truthy(left.clone());
                if o == "and" && !b {
                    return Ok(left);
                }
                if o == "or" && b {
                    return Ok(left);
                }
                return self.evaluate(*r);
            }
            Expr::Call(id_expr, args) => {
                let call_type = self.evaluate(*id_expr)?;

                if let Lit::NativeFn(fn_name) = call_type {
                    match fn_name.as_str() {
                        "clock" => {
                            return Ok(Lit::F64(
                                SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as f64,
                            ));
                        }, 
                        "cos" =>{
                            if args.len() != 1 { return Err("invalid cos syntax".to_string()) }
                            let degrees: Lit = self.evaluate(args.get(0).unwrap().clone())?;
                            let n = self.parse_number(degrees)?;
                            return Ok(Lit::F64(n.cos()))
                        }, 
                        "sin" => {
                            if args.len() != 1 { return Err("invalid cos syntax".to_string()) }
                            let degrees: Lit = self.evaluate(args.get(0).unwrap().clone())?;
                            let n = self.parse_number(degrees)?;
                            return Ok(Lit::F64(n.sin()))
                        },
                        _ => return Err("function not found".to_string()),
                    }
                } else if let Lit::DefineFn(_, params, block_stmt, temp_scope) = call_type {
                    dbg!("made it to DefineFn");
                    if params.len() != args.len() { 
                        return Err("arguments do not match parameters".to_string());
                    }

                    let mut arguments: Vec<Lit> = Vec::new();
                    for arg in &args {
                        arguments.push(self.evaluate(arg.clone())?); // Call --> [id, [expr1, expr2]]
                    }

                    let real_scope = self.scope.clone();
                    self.scope = temp_scope;
                    self.scope.push(new_scope());

                    for (i, _) in params.iter().enumerate() {
                        self.scope
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .insert(params[i].clone(), arguments[i].clone()); // DefineFn --> [id, ["arg1", "arg2"], blockStmt]
                    }

                    let val: Lit = execute_stmt(*block_stmt, self)?;
                    self.scope.pop();
                    self.scope = real_scope;
                    if let Lit::Return(return_res) = val {
                        return Ok(*return_res);
                    }
                    return Ok(Lit::Nil);
                } else {
                    return Err("function not found".to_string());
                }
            }, Expr::Arr(id, index) => {
                let lit_arr: Lit = self.evaluate(*id)?;
                let lit_index: Lit = self.evaluate(*index)?;
                let final_index: f64 = self.parse_number(lit_index)?;
                let arr: Vec<Expr> = self.parse_arr(lit_arr)?;
                
                match arr.get(final_index as usize){
                    Some(value) => return Ok(self.evaluate(value.clone())?),
                    None => return Err("could not find index!".to_string())
                }
            },      
        } 
      
    }

    pub fn is_truthy(&mut self, lit: Lit) -> bool {
        if let Lit::Nil = lit {
            return false;
        }
        if let Lit::Bool(false) = lit {
            return false;
        }
        return true;
    }

    fn parse_number(&mut self, number: Lit) -> Result<f64, String> { 
        let Lit::F64(n) = number else { return Err( "not expected number".to_string() )};
        return Ok(n);
    }
    
    fn parse_string(&mut self, string: Lit) -> Result<String, String>{
        let Lit::String(s ) = string else { return Err( "invalid string".to_string())};
        return Ok(s) 
    }

    fn parse_arr(&mut self, arr: Lit) -> Result<Vec<Expr>, String>{
         let Lit::Arr(a) = arr else { return Err ( "not an arr".to_string())};
         return Ok(a)
    }

    fn search_scope(&mut self, key: String) -> Result<Lit, String> {
        let iter: std::iter::Rev<std::slice::Iter<'_, Rc<RefCell<HashMap<String, Lit>>>>> = self.scope.iter().rev();
        for val in iter {
            if val.borrow().contains_key(&key) {
                return Ok(val.borrow()[&key].clone());
            };
        }
        return Err(format!("{} not found", key));
    }

    pub fn new() -> Interpreter {
        let mut scope: HashMap<String, Lit> = HashMap::new();
        scope.insert("clock".to_string(), Lit::NativeFn("clock".to_string()));
        scope.insert("cos".to_string(), Lit::NativeFn("cos".to_string()));
        scope.insert("sin".to_string(), Lit::NativeFn("sin".to_string()));
        scope.insert("canvas".to_string(), Lit::NativeFn("canvas".to_string()));

        return Interpreter {
            scope: vec![Rc::new(RefCell::new(scope))],
        };
    }







}
