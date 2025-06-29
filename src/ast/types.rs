#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // !
    BitNot, // ~
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Multiply,
    Divide,

    Greater,
    Less,

    GreaterEqual,
    LessEqual,

    Equal,
    NotEqual,

    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(i32),
    UnOp(UnaryOp, Box<Expr>),
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Stmt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub function: Function,
}
