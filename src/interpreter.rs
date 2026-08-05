use crate::helpers::{parse_number,parse_string, unwrap_lit_ref_cell, wrap_lit_ref_cell};
use crate::statements::{execute_stmt, new_scope};
use crate::types::{Arr, Env, ErrorHandler, Expr, Lit, Val, default};
use std::time::{SystemTime, UNIX_EPOCH};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::io::{self, Write};

pub struct Interpreter {
    pub scope: Vec<Env>,
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: Expr) -> Result<Lit, ErrorHandler> {
        match expr {
            Expr::Literal(lit) => {
                if let Lit::Id(s) = lit {
                    let lit = self.search_scope(s)?;
                    let value = unwrap_lit_ref_cell(lit);
                    return Ok(value);
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
                        return Err(ErrorHandler::InvalidSyntax {
                            message: "wrong operand".to_string(),
                            token: o,
                        });
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
                            return Err(ErrorHandler::InvalidSyntax {
                                message: "wrong operand".to_string(),
                                token: "o".to_string(),
                            });
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
                        return Err(ErrorHandler::InvalidSyntax {
                            message: "operand must be a number.".to_string(),
                            token: l,
                        });
                    }
                    _ => return Ok(Lit::Nil),
                }
            }

            Expr::Grouping(l) => return self.evaluate(*l),

            Expr::AssignVar(k, value) => {
                let lit_v: Lit = self.evaluate(*value)?;
                let iter = self.scope.iter_mut().rev();
                for vars in iter {
                    if vars.borrow().contains_key(&k) {
                        let v = wrap_lit_ref_cell(lit_v.clone());
                        vars.borrow_mut().insert(k, v);
                        return Ok(lit_v);
                    };
                }
                return Err(ErrorHandler::NotFound {
                    message: "variable not found".to_string(),
                    token: k,
                });
            }

            Expr::AssignArr(arr_expr, value) => {
                let replace_val = self.evaluate(*value)?;
                match self.evaluate_place(*arr_expr) {
                    Ok(val_to_replace) => {
                        *val_to_replace.borrow_mut() = replace_val;
                        return Ok(Lit::Nil);
                    }
                    Err(e) => {
                        if let ErrorHandler::IndexOutOfBounds {
                            message: _,
                            index,
                            arr,
                        } = e
                        {
                            let mut new_arr = arr.borrow_mut();
                            let Lit::Arr(a) = &mut *new_arr else {
                                return Err(ErrorHandler::NotFound {
                                    message: "arr not found".into(),
                                    token: "".into(),
                                });
                            };
                            if index >= a.len() {
                                for _ in a.len()..index {
                                    a.push(default())
                                }
                                a.push(wrap_lit_ref_cell(replace_val));
                                return Ok(Lit::Nil);
                            }

                            return Err(ErrorHandler::NotFound {
                                message: "".to_string(),
                                token: "".to_string(),
                            });
                        } else {
                            return Err(e);
                        }
                    }
                }
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
                                    .subsec_millis() as f64,
                            ));
                        }
                        "cos" => {
                            if args.len() != 1 {
                                return Err(ErrorHandler::InvalidSyntax {
                                    message: "invalid cos syntax".to_string(),
                                    token: fn_name,
                                });
                            }
                            let degrees: Lit = self.evaluate(args.get(0).unwrap().clone())?;
                            let n = parse_number(degrees)?;
                            return Ok(Lit::F64(n.cos()));
                        }
                        "sin" => {
                            if args.len() != 1 {
                                return Err(ErrorHandler::InvalidSyntax {
                                    message: "invalid sin syntax".to_string(),
                                    token: fn_name,
                                });
                            }
                            let degrees: Lit = self.evaluate(args.get(0).unwrap().clone())?;
                            let n = parse_number(degrees)?;
                            return Ok(Lit::F64(n.sin()));
                        }
                        "clear" => { 
                            print!("\x1b[H");
                            io::stdout().flush().ok();
                            return Ok(Lit::Nil)
                        }
                        "sleep" =>{
                            let n: Lit = self.evaluate(args.get(0).unwrap().clone())?;
                            let ms = parse_number(n)? as u64;
                            std::thread::sleep(std::time::Duration::from_millis(ms));
                            Ok(Lit::Nil)
                        }
                        "floor" =>{
                            let n = self.evaluate(args.get(0).unwrap().clone())?; 
                            let number: f64 = parse_number(n)?.floor();
                            return Ok(Lit::F64(number))
                        }
                        "write" => {
                            let s: Lit = self.evaluate(args.get(0).unwrap().clone())?;
                            print!("{}", parse_string(s)?);
                            return Ok(Lit::Nil);
                        }
                        "flush" => {
                            io::stdout().flush().ok();
                            return Ok(Lit::Nil)
                        }
                        _ => {
                            return Err(ErrorHandler::NotFound {
                                message: "function not found".to_string(),
                                token: fn_name,
                            });
                        }
                    }
                } else if let Lit::DefineFn(_, params, block_stmt, temp_scope) = call_type {
                    // dbg!("made it to DefineFn");
                    if params.len() != args.len() {
                        return Err(ErrorHandler::InvalidSyntax {
                            message: "argument length does not match parameters".to_string(),
                            token: String::new(),
                        });
                    }

                    let mut arguments: Vec<Lit> = Vec::new();
                    for arg in &args {
                        arguments.push(self.evaluate(arg.clone())?);
                    }

                    // swap scopes
                    let real_scope = self.scope.clone();
                    self.scope = temp_scope;
                    self.scope.push(new_scope());

                    //bind params to args
                    for (i, _) in params.iter().enumerate() {
                        self.insert_into_scope(params[i].clone(), arguments[i].clone());
                    }

                    let val: Lit = execute_stmt(*block_stmt, self)?;
                    self.scope.pop();
                    self.scope = real_scope;
                    if let Lit::Return(return_res) = val {
                        return Ok(*return_res);
                    }
                    return Ok(Lit::Nil);
                } else {
                    return Err(ErrorHandler::NotFound {
                        message: "function not found".to_string(),
                        token: format!("{}", call_type),
                    });
                }
            }

            Expr::Arr(_, _) => {
                let val_at_index = self.evaluate_place(expr)?;
                return Ok(val_at_index.borrow().clone());
            }
        }
    }

    fn evaluate_place(&mut self, expr: Expr) -> Result<Val, ErrorHandler> {
        match expr {
            Expr::Literal(lit) => {
                if let Lit::Id(name) = lit {
                    let v = self.search_scope(name)?;
                    return Ok(v);
                } else {
                    return Err(ErrorHandler::NotFound {
                        message: "not a valid id".to_string(),
                        token: String::new(),
                    });
                }
            }

            Expr::Arr(id, index) => {
                let cell: Val = self.evaluate_place(*id)?; // get cell (Rc(lit) containing arr)
                let i: Lit = self.evaluate(*index)?; // get index 
                let index: i32 = parse_number(i)? as i32;
                let borrowed: std::cell::Ref<'_, Lit> = cell.borrow(); // borrow so create another pointer --> Rc(lit)

                let arr: &Arr = match &*borrowed {
                    Lit::Arr(a) => a,
                    _ => {
                        return Err(ErrorHandler::InvalidSyntax {
                            message: "arr not found".to_string(),
                            token: "".into(),
                        });
                    }
                };

                match arr.get(index as usize).cloned() {
                    Some(val_at_index) => return Ok(val_at_index),
                    None => {
                        return Err(ErrorHandler::IndexOutOfBounds {
                            message: "index out of bounds".to_string(),
                            arr: cell.clone(),
                            index: index as usize,
                        });
                    }
                };
            }
            _ => Err(ErrorHandler::InvalidSyntax {
                message: "no assignable value".to_string(),
                token: format!("{:?}", expr),
            }),
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

    pub fn insert_into_scope(&mut self, k: String, v: Lit) {
        let val: Rc<RefCell<Lit>> = wrap_lit_ref_cell(v);
        self.scope.last().unwrap().borrow_mut().insert(k, val);
    }

    pub fn new() -> Interpreter {
        let mut scope: HashMap<String, Rc<RefCell<Lit>>> = HashMap::new();
        let names = vec!["clock", "cos", "sin", "clear", "sleep", "floor", "write", "flush"].into_iter();
        for name in names {
            let v = Rc::new(RefCell::new(Lit::NativeFn(name.to_string())));
            scope.insert(name.to_string(), v);
        }
        return Interpreter {
            scope: vec![Rc::new(RefCell::new(scope))],
        };
    }

    fn search_scope(&mut self, key: String) -> Result<Rc<RefCell<Lit>>, ErrorHandler> {
        let iter: std::iter::Rev<std::slice::Iter<'_, Env>> = self.scope.iter().rev();
        for val in iter {
            if val.borrow().contains_key(&key) {
                return Ok(val.borrow()[&key].clone());
            };
        }
        return Err(ErrorHandler::NotFound {
            message: "not found in scope".to_string(),
            token: key,
        });
    }
}
