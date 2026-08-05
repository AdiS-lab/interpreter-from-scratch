use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;


pub fn parse_number(number: Lit) -> Result<f64, ErrorHandler> {
    let Lit::F64(n) = number else { return Err(ErrorHandler::InvalidSyntax{message: "not expected number".to_string(), token: String::new()}) };
    return Ok(n);
}

pub fn wrap_lit_ref_cell(lit: Lit) -> Rc<RefCell<Lit>> {
    return Rc::new(RefCell::new(lit))
}

pub fn unwrap_lit_ref_cell(ref_cell: Rc<RefCell<Lit>>) -> Lit {
    return ref_cell.borrow().clone()
}

pub fn parse_string(string: Lit) -> Result<String, ErrorHandler>{
    let Lit::String(s) = string else { return Err(ErrorHandler::InvalidSyntax{message: "invalid string".to_string(), token: String::new()}) };
    return Ok(s)
}