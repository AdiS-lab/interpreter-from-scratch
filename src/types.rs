use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = Rc<RefCell<HashMap<String, Rc<RefCell<Lit>>>>>;
pub type Arr = Vec<Rc<RefCell<Lit>>>;
pub type Val = Rc<RefCell<Lit>>;

#[derive(Debug, Clone)]
pub enum Declr {
    VarDeclr(String, Stmt),
    FunDeclr(String, Vec<String>, Stmt),
    Reg(Stmt),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Other(Expr),
    Block(Vec<Declr>),
    IfChain(Expr, Box<Stmt>, Box<Stmt>),
    WhileStmt(Expr, Box<Stmt>),
    ForStmt(Box<Declr>, Box<Stmt>, Expr, Box<Stmt>),
    ReturnStmt(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    AssignVar(String, Box<Expr>),
    Operand(Box<Expr>, String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Arr(Box<Expr>, Box<Expr>), // <id, index>
    AssignArr(Box<Expr>, Box<Expr>),
}
#[derive(Debug, Clone)]
pub enum Lit {
    String(String),
    Bool(bool),
    Nil,
    F64(f64),
    Id(String),
    DefineFn(String, Vec<String>, Box<Stmt>, Vec<Env>),
    NativeFn(String),
    Return(Box<Lit>),
    Arr(Arr),
    //final has id inside so parse this and then get a string, and then bubble this up
}

#[derive(Debug, Clone)]
pub enum ErrorHandler {
    IndexOutOfBounds {
        message: String,
        index: usize,
        arr: Val,
    },
    InvalidSyntax {
        message: String,
        token: String,
    },
    NotFound {
        message: String,
        token: String,
    },
}

impl std::fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorHandler::IndexOutOfBounds {
                message,
                index,
                arr,
            } => {
                write!(
                    f,
                    "IndexOutOfBounds: {} (index: {}, arr {:?})",
                    message, index, arr
                )
            }
            ErrorHandler::InvalidSyntax { message, token } => {
                write!(f, "InvalidSyntax: {} {}", message, token)
            }
            ErrorHandler::NotFound { message, token } => {
                write!(f, "NotFound: {} {}", message, token)
            }
        }
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lit::F64(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::String(s) => write!(f, "{}", s),
            Lit::Id(s) => write!(f, "{}", s),
            Lit::Nil => write!(f, "nil"),
            Lit::DefineFn(s, _, _, _) => write!(f, "<fn {}>", s),
            Lit::NativeFn(s) => write!(f, "<fn {}>", s),
            Lit::Return(e) => write!(f, "{:?}", e),
            Lit::Arr(v) => {
                for i in v{
                    println!("{}", i.borrow().clone());
                }
                return write!(f, "");
            }
        }
    }
}

pub fn default() -> Rc<RefCell<Lit>> {
    return Rc::new(RefCell::new(Lit::Nil));
}
