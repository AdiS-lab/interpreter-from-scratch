#[derive(Debug, Clone)]
pub enum Declr{
    VarDeclr(String, Stmt),
    FunDeclr(String, Vec<String>, Stmt),
    Reg(Stmt)
}

#[derive(Debug, Clone)]
pub enum Stmt{
    Print(Box<Stmt>),
    Other(Expr),
    Block(Vec<Declr>),
    IfChain(Expr, Box<Stmt>, Box<Stmt>),
    WhileStmt(Expr, Box<Stmt>),
    ForStmt(Box<Declr>, Box<Stmt>, Expr, Box<Stmt>),
    FunStmt(String, Vec<Expr>)
}

#[derive(Debug, Clone)]                                                                                                                                                   
pub enum Expr{
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    Assign(String, Box<Expr>),
    Operand(Box<Expr>, String, Box<Expr>)
}
#[derive(Debug, Clone)]                                                                                             
pub enum Lit{
    String(String),
    Bool(bool),
    Nil,
    F64(f64),
    Id(String),
    DeclrFn(Vec<String>, Box<Stmt>),
    NativeFn(String)
}

impl std::fmt::Display for Lit {            
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
          match self {
              Lit::F64(n) => write!(f, "{}", n),                                                                                                                                                                                                                                                        
              Lit::Bool(b) => write!(f, "{}", b),
              Lit::String(s) => write!(f, "{}", s),    
              Lit::Id(s) => write!(f, "{}", s),    
              Lit::Nil => write!(f, "nil"), 
              _ => write!(f, "complex literal")                                                                                                                                                                                                                                                       
          }
      }
}