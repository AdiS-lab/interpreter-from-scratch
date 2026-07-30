
use crate::types::*;
use std::collections::{HashMap};
use crate::Interpreter;


pub fn execute(list: Vec<Declr>, interpreter: &mut Interpreter) -> Result<Lit, String> {
    for declaration in list{
        if let Declr::VarDeclr(id, stmt) = declaration {
            ex_var(id, stmt, interpreter)?;
        }else if let Declr::FunDeclr(id, parameters, stmt) = declaration{
            add_function(id, parameters, stmt, interpreter)
        }else if let Declr::Reg(stmt) = declaration{
            let val: Lit = ex_reg(stmt, interpreter)?; // check for nil
            if matches!(val, Lit::Return(_)) { return Ok(val); }
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
        if matches!(val, Lit::Return(_)) { return Ok(val); }

    }else if let Stmt::Other(expr) = stmt{ // only hits here on implicit returns
        let val: Lit = interpreter.evaluate(expr)?;

    }else if let Stmt::ReturnStmt(expr) = stmt{ // only hits here on returns
        let return_val: Lit = interpreter.evaluate(expr)?;
        return Ok(Lit::Return(Box::new(return_val)));

    }else if let Stmt::IfChain(conditional, then_stmt, else_stmt) = stmt{ 
        let val: Lit = interpreter.evaluate(conditional)?;
        let b: bool = interpreter.is_truthy(val.clone());
        if b{
            let val = ex_reg(*then_stmt, interpreter)?;
            if matches!(val, Lit::Return(_)) { return Ok(val); }
        }else{
            let val = ex_reg(*else_stmt, interpreter)?; 
            if matches!(val, Lit::Return(_)) { return Ok(val); }

        }

    }else if let Stmt::WhileStmt(conditional, stmt) = stmt{
        let mut res = interpreter.evaluate(conditional.clone())?;

        while interpreter.is_truthy(res) { 
            let val = ex_reg(*stmt.clone(), interpreter)?;
            if matches!(val, Lit::Return(_)) { return Ok(val); } 
            res = interpreter.evaluate(conditional.clone())?;
        };
        
    }else if let Stmt::ForStmt(var_init,range, incr, stmt) = stmt{
        if let Declr::VarDeclr(id, val) = *var_init{
            ex_var(id.clone(), val, interpreter)?; // create var with num
        }else if let Declr::Reg(stmt) = *var_init{
            ex_reg(stmt, interpreter)?; // asssignment
        }
        let condition = if let Stmt::Other(c) = *range { c } else { Expr::Literal(Lit::Bool(true)) }; // condition
        let mut val = interpreter.evaluate(condition.clone())?; // range

        while interpreter.is_truthy(val){
            let res = ex_reg(*stmt.clone(), interpreter)?; 
            if matches!(res, Lit::Return(_)) { return Ok(res); }
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
    // if parameters.len() = 0{

    // }else {
        interpreter.scope.last_mut().unwrap().insert(id.clone(), Lit::DefineFn(id, parameters, Box::new(stmt) ));
    // }
}