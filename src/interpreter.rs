use crate::statements::{ex_reg, new_scope};
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
        // dbg!(&self.scope);
        match expr {
            Expr::Literal(lit) => {
                if let Lit::Id(s) = lit {
                    return self.search_state(s);
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
                //false or false or true =>   false or true
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
                        }
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
                    } //eval args before switching to bind any updated variables

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
                    // inserting vars into the same scope as func.

                    let val: Lit = ex_reg(*block_stmt, self)?;
                    self.scope.pop();
                    self.scope = real_scope;
                    if let Lit::Return(return_res) = val {
                        return Ok(*return_res);
                    }
                    return Ok(Lit::Nil);
                } else {
                    return Err("function not found".to_string());
                }
            }
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

    fn search_state(&mut self, key: String) -> Result<Lit, String> {
        let iter = self.scope.iter().rev();
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
        return Interpreter {
            scope: vec![Rc::new(RefCell::new(scope))],
        };
    }
}
