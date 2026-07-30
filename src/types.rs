#[derive(Debug, Clone)]
pub enum Declr{
    VarDeclr(String, Stmt),
    FunDeclr(String, Vec<String>, Stmt),
    Reg(Stmt)
}

#[derive(Debug, Clone)]
pub enum Stmt{
    Print(Expr),
    Other(Expr),
    Block(Vec<Declr>),
    IfChain(Expr, Box<Stmt>, Box<Stmt>),
    WhileStmt(Expr, Box<Stmt>),
    ForStmt(Box<Declr>, Box<Stmt>, Expr, Box<Stmt>),
    ReturnStmt(Expr)
}

#[derive(Debug, Clone)]                                                                                                                                                   
pub enum Expr{
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    Assign(String, Box<Expr>),
    Operand(Box<Expr>, String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>)
}
#[derive(Debug, Clone)]                                                                                             
pub enum Lit{
    String(String),
    Bool(bool),
    Nil,
    F64(f64),
    Id(String),
    DefineFn(String, Vec<String>, Box<Stmt>, usize),
    NativeFn(String), // this would mean that have to parse string out when matching on this
    Return(Box<Lit>)
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
              Lit::Return(e) => write!(f, "{:?}", e)
          }
      }
}