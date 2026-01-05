#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Type {
    I32,
    F32,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    NumberF32(f32),
    Identifier(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Let(String, Expr),
    Const(String, Expr),
    Assign(String, Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(
        Option<Box<Stmt>>,
        Option<Expr>,
        Option<Box<Stmt>>,
        Box<Stmt>,
    ),
    Block(Vec<Stmt>),
    Return(Expr),
    Break,
    Continue,
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub param_types: Option<Vec<Type>>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub line: usize,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub top_level: Vec<Stmt>,
}
