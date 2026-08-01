use crate::Interpreter;
use crate::types::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn execute(list: Vec<Declr>, interpreter: &mut Interpreter) -> Result<Lit, String> {
    for declaration in list {
        if let Declr::VarDeclr(id, value) = declaration {
            create_variable(id, value, interpreter)?;
            
        } else if let Declr::FunDeclr(id, parameters, stmt) = declaration {
            dbg!("executing function declaration");
            add_function(id, parameters, stmt, interpreter)

        } else if let Declr::Reg(stmt) = declaration {
            let val: Lit = execute_stmt(stmt, interpreter)?; // check for nil
            if matches!(val, Lit::Return(_)) {
                return Ok(val);
            }

        }
    }
    return Ok(Lit::Nil);
}

pub fn execute_stmt(stmt: Stmt, interpreter: &mut Interpreter) -> Result<Lit, String> {
    if let Stmt::Print(expr ) = stmt {
        let val: Lit = interpreter.evaluate(expr)?;
        // dbg!("printg out {}...", val.clone());
        println!("{}", val);
    } else if let Stmt::Block(list) = stmt {
        interpreter
            .scope
            .push(new_scope());
        let val: Lit = execute(list, interpreter)?;
        interpreter.scope.pop();
        if matches!(val, Lit::Return(_)) {
            return Ok(val);
        }
    } else if let Stmt::Other(expr) = stmt {
        // only hits here on implicit returns
        interpreter.evaluate(expr)?;
    } else if let Stmt::ReturnStmt(expr) = stmt {
        // only hits here on returns
        let return_val: Lit = interpreter.evaluate(expr)?;
        return Ok(Lit::Return(Box::new(return_val)));
    } else if let Stmt::IfChain(conditional, then_stmt, else_stmt) = stmt {
        let val: Lit = interpreter.evaluate(conditional)?;
        let b: bool = interpreter.is_truthy(val.clone());
        if b {
            let val = execute_stmt(*then_stmt, interpreter)?;
            if matches!(val, Lit::Return(_)) {
                return Ok(val);
            }
        } else {
            let val = execute_stmt(*else_stmt, interpreter)?;
            if matches!(val, Lit::Return(_)) {
                return Ok(val);
            }
        }
    } else if let Stmt::WhileStmt(conditional, stmt) = stmt {
        let mut res = interpreter.evaluate(conditional.clone())?;

        while interpreter.is_truthy(res) {
            let val = execute_stmt(*stmt.clone(), interpreter)?;
            if matches!(val, Lit::Return(_)) {
                return Ok(val);
            }
            res = interpreter.evaluate(conditional.clone())?;
        }
    } else if let Stmt::ForStmt(var_init, range, incr, stmt) = stmt {
        execute(vec![*var_init,], interpreter)?;
        let condition: Expr = if let Stmt::Other(c) = *range {
            c
        } else {
            Expr::Literal(Lit::Bool(true))
        };
        
        let mut loop_condition = interpreter.evaluate(condition.clone())?;
        while interpreter.is_truthy(loop_condition) {
            let res: Lit = execute_stmt(*stmt.clone(), interpreter)?;
            if matches!(res, Lit::Return(_)) {
                return Ok(res);
            }
            match incr {
                Expr::Literal(Lit::Nil) => {}
                _ => {
                    println!("makign it into evaluation");
                    interpreter.evaluate(incr.clone())?;
                }
            }
            loop_condition = interpreter.evaluate(condition.clone())?;
        }
    }
    return Ok(Lit::Nil);
}

pub fn create_variable(id: String, stmt: Stmt, interpreter: &mut Interpreter) -> Result<(), String> {
    if let Stmt::Other(expr) = stmt {
        let val: Lit = interpreter.evaluate(expr)?;
        interpreter
            .scope
            .last()
            .unwrap()
            .borrow_mut()
            .insert(id, val);
    }
    return Ok(());
}

pub fn add_function(
    id: String,
    parameters: Vec<String>,
    stmt: Stmt,
    interpreter: &mut Interpreter,
) {
    let temp_scope: Vec<Env> = interpreter.scope.clone();
    let val: Lit = Lit::DefineFn(id.clone(), parameters, Box::new(stmt), temp_scope);
    // interpreter.scope.borrow_mut().unwrap().insert(id.clone(), Lit::DefineFn(id, parameters, Box::new(stmt), temp_scope));
    interpreter
        .scope
        .last()
        .unwrap()
        .borrow_mut()
        .insert(id, val);

    dbg!("adding function to current scope...");
}

pub fn new_scope() -> Env {
    return Rc::new(RefCell::new(HashMap::new()))
}