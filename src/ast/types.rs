#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // !
    BitNot, // ~
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(i32),
    UnOp(UnaryOp, Box<Expr>),
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
