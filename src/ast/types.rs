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

    Or,
    And,
    Xor,
    ShiftRight,
    ShiftLeft,
    Modulo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(i32),
    UnOp(UnaryOp, Box<Expr>),
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>), // the three expressions are the condition, 'if' expression and 'else' expression, respectively
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return(Expr),
    Expr(Expr),
    If(Expr, Box<Statement>, Option<Box<Statement>>), // condition, if branch, else branch (optional)

    Bingus(Expr), // print expr int val
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Declare(String, Option<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    Stmt(Statement),
    Decl(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub block_items: Vec<BlockItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub function: Function,
}
