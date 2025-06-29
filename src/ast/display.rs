use crate::ast::{BinaryOp, Expr, Function, Program, Stmt, UnaryOp};
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(n) => write!(f, "Int<{}>", n),
            Expr::UnOp(op, expr) => write!(f, "{}{}", op, expr),
            Expr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitNot => "~",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BinaryOp::Multiply => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Add => "+",
            BinaryOp::Divide => "/",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Return(expr) => {
                writeln!(f, "return {}", expr)
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "function int {}:", self.name)?;
        writeln!(f, "    params: ()")?;
        writeln!(f, "    body:\n        {}", self.body)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.function)
    }
}
