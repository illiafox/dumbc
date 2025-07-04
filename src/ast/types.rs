#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    /// Unary negation operation "-"
    Neg,
    /// Unary logical `NOT` operation "!"
    Not,
    /// Unary bitwise `NOT` operation "~"
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    /// Binary addition operation "+"
    Add,
    /// Binary substraction operator "-"
    Sub,
    /// Binary multiplication operation "*"
    Multiply,
    /// Binary division operation "/"
    Divide,

    /// Binary logical "greater than" operation ">"
    Greater,
    /// Binary logical "less than" operation "<"
    Less,

    /// Binary logical "greater or equal" operation ">="
    GreaterEqual,
    /// Binary logical "less or equal" operation "<="
    LessEqual,

    /// Binary logical equality operation "=="
    Equal,
    /// Binary logical inequality operation "!="
    NotEqual,

    /// Binary logical `AND` operation "&&"
    LogicalAnd,
    /// Binary logical `OR` operation "||"
    LogicalOr,

    /// Binary bitwise `OR` operation "|"
    Or,
    /// Binary bitwise `AND` operation "&"
    And,
    /// Binary bitwise `XOR` operation "^"
    Xor,
    /// Binary right shift operation ">>"
    ShiftRight,
    /// Binary left shift operation "<<"
    ShiftLeft,
    /// Binary modulo operation "%"
    Modulo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// `int` literal expression
    Const(i32),
    /// Unary operation expression
    UnOp(UnaryOp, Box<Expr>),
    /// Binary operation expression
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
    /// Variable expression
    Var(String),
    /// Variable assignment expression
    Assign(String, Box<Expr>),
    /// Ternary expression (cond ? then : else)
    Conditional {
        cond: Box<Expr>,
        then: Box<Expr>,
        els: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// `return expr;` statement
    Return(Expr),
    /// Arbitrary expression statement
    Expr(Expr),
    /// "if-then- optional else" block
    If {
        cond: Expr,
        then: Box<Statement>,
        els: Option<Box<Statement>>,
    },
    /// Lexical scope, enclosed in braces (`{}`).
    Compound(Vec<BlockItem>), // added

    /// Print the value of the [Expr] as an `int`
    Bingus(Expr),
}

/// Item of a [`Statement::Compound`]
#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    /// Arbitrary statement
    Stmt(Statement),
    /// Variable declaration with optional initial value.
    Decl(String, Option<Expr>),
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
