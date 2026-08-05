use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser {
    pub tokens: Vec<String>,
    pub curr_pos: usize,
}

impl Parser {
    fn block(&mut self) -> Result<Vec<Declr>, ErrorHandler> {
        self.expect("{")?;
        let res: Vec<Declr> = self.declaration()?;
        self.expect("}")?;
        return Ok(res);
    }

    fn fun_declr(&mut self) -> Result<Declr, ErrorHandler> {
        self.expect("fun")?;
        let function_id: String = self.consume();
        self.expect("(")?;
        let mut parameters: Vec<String> = Vec::new();

        if self.peek() == "RIGHT_PAREN" {
            self.expect(")")?;
        } else if self.peek() == "IDENTIFIER" {
            parameters.push(self.consume());
            while self.peek() == "COMMA" {
                self.expect(",")?;
                if self.peek() != "IDENTIFIER" {
                    return Err(ErrorHandler::InvalidSyntax {
                        message: "invalid syntax on id".to_string(),
                        token: String::new(),
                    });
                }
                parameters.push(self.consume());
            }
            self.expect(")")?;
        } else {
            return Err(ErrorHandler::InvalidSyntax {
                message: format!("wrong syntax on function {}", function_id),
                token: function_id,
            });
        }

        if self.peek() != "LEFT_BRACE" {
            return Err(ErrorHandler::InvalidSyntax {
                message: "make sure to include braces or arrow".to_string(),
                token: function_id,
            });
        }

        let body: Stmt = self.statement()?;
        return Ok(Declr::FunDeclr(function_id, parameters, body));
    }

    fn var_declr(&mut self) -> Result<Declr, ErrorHandler> {
        self.expect("var")?;
        let id: String = self.consume();
        let operator: String = self.consume();
        match operator.as_str() {
            "=" => return Ok(Declr::VarDeclr(id, self.statement()?)),
            ";" => return Ok(Declr::VarDeclr(id, Stmt::Other(Expr::Literal(Lit::Nil)))),
            _ => {
                return Err(ErrorHandler::InvalidSyntax {
                    message: "bad syntax on var".to_string(),
                    token: String::new(),
                });
            }
        }
    }

    pub fn declaration(&mut self) -> Result<Vec<Declr>, ErrorHandler> {
        let mut tk_type: String = self.peek();
        let mut d: Vec<Declr> = Vec::<Declr>::new();
        while tk_type != "EOF" && tk_type != "RIGHT_BRACE" {
            if matches!(tk_type.as_str(), "VAR") {
                d.push(self.var_declr()?);
            } else if matches!(tk_type.as_str(), "FUN") {
                d.push(self.fun_declr()?);
            } else {
                let body: Stmt = self.statement()?;
                d.push(Declr::Reg(body));
            }
            tk_type = self.peek();
        }
        return Ok(d);
    }

    fn statement(&mut self) -> Result<Stmt, ErrorHandler> {
        let tk_type: String = self.peek();
        if matches!(tk_type.as_str(), "PRINT") {
            self.expect("print")?;
            let message: Expr = self.assignment()?;
            self.expect(";")?;
            return Ok(Stmt::Print(message));
        } else if matches!(tk_type.as_str(), "LEFT_BRACE") {
            let body: Vec<Declr> = self.block()?; // call back 
            return Ok(Stmt::Block(body));
        } else if matches!(tk_type.as_str(), "IF") {
            self.expect("if")?;
            self.expect("(")?;
            let condition: Expr = self.assignment()?;
            self.expect(")")?;

            let then_st: Stmt = self.statement()?;
            let mut else_st: Stmt = Stmt::Other(Expr::Literal(Lit::Nil));

            if self.peek() == "ELSE" {
                self.consume();
                else_st = self.statement()?;
            }
            return Ok(Stmt::IfChain(
                condition,
                Box::new(then_st),
                Box::new(else_st),
            ));
        } else if matches!(tk_type.as_str(), "WHILE") {
            self.expect("while")?;
            self.expect("(")?;
            let condition: Expr = self.assignment()?;
            self.expect(")")?;

            let body = self.statement()?;
            return Ok(Stmt::WhileStmt(condition, Box::new(body)));
        } else if matches!(tk_type.as_str(), "FOR") {
            self.expect("for")?;
            self.expect("(")?;
            let mut start_val: Declr = Declr::Reg(Stmt::Other(Expr::Literal(Lit::Nil)));

            if self.peek() == "VAR" {
                start_val = self.var_declr()?;
            } else if self.peek() == "SEMICOLON" {
                self.expect(";")?; // take this out of the conditionals afterwards
            } else {
                start_val = Declr::Reg(self.statement()?);
            }

            let conditional: Stmt = self.statement()?;
            let mut incr: Expr = Expr::Literal(Lit::Nil);
            if self.peek() != "RIGHT_PAREN" {
                incr = self.assignment()?;
            }

            self.expect(")")?;
            let body: Stmt = self.statement()?;
            return Ok(Stmt::ForStmt(
                Box::new(start_val),
                Box::new(conditional),
                incr,
                Box::new(body),
            ));
        } else if matches!(tk_type.as_str(), "RETURN") {
            self.expect("return")?;
            if self.peek() == "SEMICOLON" {
                self.consume();
                return Ok(Stmt::ReturnStmt(Expr::Literal(Lit::Nil)));
            } else {
                let expr = self.assignment()?;
                self.expect(";")?;
                return Ok(Stmt::ReturnStmt(expr));
            }
        } else {
            let result: Expr = self.assignment()?; // this is for calling vars or functions
            self.expect(";")?;
            return Ok(Stmt::Other(result));
        }
    }

    fn assignment(&mut self) -> Result<Expr, ErrorHandler> {
        let left: Expr = self.operands()?;
        let tk_type = self.peek();
        if matches!(tk_type.as_str(), "EQUAL") {
            self.expect("=")?;
            let right = self.assignment()?;
            if let Expr::Literal(Lit::Id(id)) = left {
                return Ok(Expr::AssignVar(id, Box::new(right)));
            } else if let Expr::Arr(_, _) = left {
                return Ok(Expr::AssignArr(Box::new(left), Box::new(right)));
            }
            return Err(ErrorHandler::InvalidSyntax {
                message: "not a valid assignable".to_string(),
                token: String::new(),
            });
        }
        return Ok(left);
    }

    fn operands(&mut self) -> Result<Expr, ErrorHandler> {
        let mut left = self.equality()?;

        while matches!(self.peek().as_str(), "OR" | "AND") {
            let operator: String = self.consume();
            let right = self.equality()?;
            left = Expr::Operand(Box::new(left), operator, Box::new(right));
        }
        return Ok(left);
    }

    pub fn equality(&mut self) -> Result<Expr, ErrorHandler> {
        let mut left = self.comparison()?;

        while matches!(self.peek().as_str(), "BANG_EQUAL" | "EQUAL_EQUAL") {
            let operator = self.consume();
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }

        return Ok(left);
    }

    fn comparison(&mut self) -> Result<Expr, ErrorHandler> {
        let mut left = self.add()?;

        while matches!(
            self.peek().as_str(),
            "GREATER_EQUAL" | "GREATER" | "LESS" | "LESS_EQUAL"
        ) {
            let operator = self.consume();
            let right = self.add()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        return Ok(left);
    }

    fn add(&mut self) -> Result<Expr, ErrorHandler> {
        let mut left = self.mult()?;
        while matches!(self.peek().as_str(), "PLUS" | "MINUS") {
            let operator = self.consume();
            let right = self.mult()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        return Ok(left);
    }

    fn mult(&mut self) -> Result<Expr, ErrorHandler> {
        let mut left = self.unary()?;

        while matches!(self.peek().as_str(), "STAR" | "SLASH") {
            let operator = self.consume();
            let right = self.unary()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        return Ok(left);
    }

    fn unary(&mut self) -> Result<Expr, ErrorHandler> {
        let tk_type = self.peek();
        if matches!(tk_type.as_str(), "MINUS" | "BANG") {
            let operator = self.consume();
            let right = self.arr_call()?;
            let unary = Expr::Unary(operator, Box::new(right));
            return Ok(unary);
        }
        let result = self.arr_call()?;
        return Ok(result);
    }

    fn arr_call(&mut self) -> Result<Expr, ErrorHandler> {
        let mut id: Expr = self.func_call()?;
        while self.peek() == "LEFT_SQUARE" {
            self.expect("[")?;
            let index: Expr = self.literal()?;
            self.expect("]")?;
            id = Expr::Arr(Box::new(id), Box::new(index));
        }
        return Ok(id);
    }

    fn func_call(&mut self) -> Result<Expr, ErrorHandler> {
        let mut left: Expr = self.literal()?;
        while self.peek() == "LEFT_PAREN" {
            self.expect("(")?;

            let mut arguments: Vec<Expr> = Vec::new();
            while self.peek() != "RIGHT_PAREN" {
                if self.peek() != "COMMA" {
                    let argument: Expr = self.assignment()?;
                    arguments.push(argument);
                } else {
                    self.consume();
                }
            }

            left = Expr::Call(Box::new(left), arguments);
            self.expect(")")?;
        }
        return Ok(left);
    }

    fn literal(&mut self) -> Result<Expr, ErrorHandler> {
        let tk_type = self.peek();
        let curr_tok = self.tokens[self.curr_pos].to_string();

        if matches!(tk_type.as_str(), "NUMBER") {
            let f: f64 = self.consume().parse().unwrap();
            return Ok(Expr::Literal(Lit::F64(f)));
        } else if matches!(tk_type.as_str(), "IDENTIFIER") {
            let id: String = self.consume();
            return Ok(Expr::Literal(Lit::Id(id)));
        } else if matches!(tk_type.as_str(), "LEFT_SQUARE") {
            self.expect("[")?;
            let mut arr: Arr = Vec::new();

            // THESE ARE THE EXACT SAME

            if !self.is_next_primitive() {
                return {
                    Err(ErrorHandler::InvalidSyntax {
                        message: "have to include index!".to_string(),
                        token: String::new(),
                    })
                };
            }
            let Expr::Literal(l) = self.literal()? else {
                return Err(ErrorHandler::InvalidSyntax {
                    message: "invalid argument type".to_string(),
                    token: String::new(),
                });
            };
            arr.push(Rc::new(RefCell::new(l)));

            while self.peek() == "COMMA" {
                self.expect(",")?;
                if !self.is_next_primitive() {
                    return Err(ErrorHandler::InvalidSyntax {
                        message: "invalid argument arr".to_string(),
                        token: String::new(),
                    });
                }
                let Expr::Literal(l) = self.literal()? else {
                    return Err(ErrorHandler::InvalidSyntax {
                        message: "bad value".to_string(),
                        token: String::new(),
                    });
                };
                arr.push(Rc::new(RefCell::new(l)));
            }

            self.expect("]")?;
            return Ok(Expr::Literal(Lit::Arr(arr)));
        } else if matches!(tk_type.as_str(), "STRING") {
            let s: String = curr_tok.split('"').nth(1).unwrap().to_string();
            self.curr_pos += 1;
            return Ok(Expr::Literal(Lit::String(s)));
        } else if matches!(tk_type.as_str(), "TRUE" | "FALSE") {
            let b: bool = self.consume().parse().unwrap();
            return Ok(Expr::Literal(Lit::Bool(b)));
        } else if matches!(tk_type.as_str(), "NIL") {
            self.expect("nil")?;
            return Ok(Expr::Literal(Lit::Nil));
        } else if matches!(tk_type.as_str(), "BANG" | "MINUS") {
            let result: Expr = self.unary()?;
            return Ok(result);
        } else if matches!(tk_type.as_str(), "LEFT_PAREN") {
            self.expect("(")?; // consumes (
            let body = self.assignment()?;
            self.expect(")")?;
            return Ok(Expr::Grouping(Box::new(body)));
        } else if matches!(tk_type.as_str(), "SEMICOLON") {
            return Err(ErrorHandler::InvalidSyntax {
                message: "line [1] missing some requirement".to_string(),
                token: String::new(),
            });
        } else {
            return Err(ErrorHandler::InvalidSyntax {
                message: "[ERROR] expect expression".to_string(),
                token: self.consume(),
            });
        }
    }

    fn consume(&mut self) -> String {
        let curr_tok = self.tokens[self.curr_pos].to_string();
        let tk_arr: Vec<&str> = curr_tok.split(" ").collect();
        self.curr_pos += 1;
        // println!("{:?}", tk_arr.get(1).unwrap_or(&"").to_string());
        return tk_arr.get(1).unwrap_or(&"").to_string();
    }

    fn peek(&mut self) -> String {
        let curr_tok = self.tokens[self.curr_pos].to_string();
        let words: Vec<&str> = curr_tok.split(" ").collect();
        let curr_type = words.get(0).unwrap_or(&"").to_string();
        return curr_type;
    }

    fn expect(&mut self, check: &str) -> Result<String, ErrorHandler> {
        let tk = self.consume();
        if tk.as_str() != check {
            return Err(ErrorHandler::InvalidSyntax {
                message: "invalid syntax".to_string(),
                token: check.to_string(),
            });
        }
        return Ok(tk);
    }

    fn is_next_primitive(&mut self) -> bool {
        if matches!(
            self.peek().as_str(),
            "NUMBER" | "IDENTIFIER" | "STRING" | "TRUE" | "FALSE" | "NIL" | "LEFT_SQUARE"
        ) {
            return true;
        }
        return false;
    }
}

pub fn parse(val: Expr) -> String {
    match val {
        Expr::Literal(lit) => {
            if let Lit::F64(f) = lit {
                return format!("{:?}", f);
            } else if let Lit::String(s) = lit {
                return format!("{}", s);
            } else if let Lit::Bool(b) = lit {
                return format!("{}", b);
            } else if let Lit::Nil = lit {
                return "nil".to_string();
            }
        }
        Expr::Binary(l, o, r) => return format!("({} {} {})", o, parse(*l), parse(*r)),
        Expr::Unary(l, r) => return format!("({} {})", l, parse(*r)),
        Expr::Grouping(l) => return format!("(group {})", parse(*l)),
        Expr::AssignVar(s, expr) => return format!("({}{})", s, parse(*expr)),
        Expr::AssignArr(val, right) => return format!("{:?} {:?}", parse(*val), parse(*right)),
        Expr::Operand(l, o, r) => return format!("({} {} {})", o, parse(*l), parse(*r)),
        Expr::Call(id, args) => return format!("({:?} {:?})", parse(*id), args),
        Expr::Arr(id, index) => return format!("{:?} {}", parse(*id), parse(*index)),
    };
    return "".to_string();
}
