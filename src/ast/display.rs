use crate::ast::Declaration::Declare;
use crate::ast::{BinaryOp, BlockItem, Declaration, Expr, Function, Program, Statement, UnaryOp};
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(n) => write!(f, "Int<{}>", n),
            Expr::UnOp(op, expr) => write!(f, "{}{}", op, expr),
            Expr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Var(name) => write!(f, "(var {})", name),
            Expr::Assign(name, exp) => write!(f, "{} = {}", name, exp),
            Expr::Conditional { cond, then, els } => {
                write!(f, "({} ? {} : {})", cond, then, els)
            }

            Expr::FunCall { name, parameters } => {
                let param_strs: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                write!(f, "{}({})", name, param_strs.join(", "))
            }
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
            BinaryOp::Multiply => "*",
            BinaryOp::Sub => "-",
            BinaryOp::Add => "+",
            BinaryOp::Divide => "/",

            BinaryOp::Greater => ">",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::Less => "<",
            BinaryOp::LessEqual => "<=",

            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",

            BinaryOp::LogicalAnd => "&&",
            BinaryOp::LogicalOr => "||",

            BinaryOp::And => "&",
            BinaryOp::Or => "|",
            BinaryOp::Xor => "^",

            BinaryOp::ShiftLeft => "<<",
            BinaryOp::ShiftRight => ">>",

            BinaryOp::Modulo => "%",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(expr) => writeln!(f, "return {}", expr),
            Statement::Expr(expr) => {
                if let Some(expr) = expr {
                    writeln!(f, "{}", expr)
                } else {
                    writeln!(f, "null expr")
                }
            }
            Statement::Bingus(expr) => writeln!(f, "bingus {}", expr),
            Statement::If { cond, then, els } => {
                if let Some(else_expr) = els {
                    writeln!(f, "if {} {} {}", cond, then, else_expr)
                } else {
                    writeln!(f, "if {} {}", cond, then)
                }
            }
            Statement::Compound(block_items) => {
                writeln!(f, "{{")?;
                for block_item in block_items {
                    writeln!(f, "  {}", block_item)?;
                }
                writeln!(f, "}}")
            }

            Statement::For {
                init,
                cond,
                post,
                body,
            } => {
                writeln!(
                    f,
                    "for ({}; {}; {}) {{",
                    init.as_ref().map_or(String::new(), |e| format!("{}", e)),
                    cond,
                    post.as_ref().map_or(String::new(), |e| format!("{}", e)),
                )?;
                writeln!(f, "\t{body}")?;
                writeln!(f, "}}")
            }

            Statement::ForDecl {
                decl,
                cond,
                post,
                body,
            } => {
                writeln!(
                    f,
                    "for ({}; {}; {}) {{",
                    decl,
                    cond,
                    post.as_ref().map_or(String::new(), |e| format!("{}", e)),
                )?;
                writeln!(f, "\t{body}")?;
                writeln!(f, "}}")
            }

            Statement::While { cond, body } => {
                writeln!(f, "while ({cond}) {{")?;
                writeln!(f, "\t{body}")?;
                writeln!(f, "}}")
            }

            Statement::Do { body, cond } => {
                writeln!(f, "do {{")?;
                writeln!(f, "\t{body}")?;
                writeln!(f, "}} while ({cond})")
            }

            Statement::Break => writeln!(f, "break"),
            Statement::Continue => writeln!(f, "continue"),
        }
    }
}

impl fmt::Display for BlockItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockItem::Stmt(stmt) => writeln!(f, "stmt {}", stmt),
            BlockItem::Decl(decl) => writeln!(f, "decl {decl}"),
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declare(name, expr) => {
                if let Some(expr) = expr {
                    writeln!(f, "declare {} = {}", name, expr)
                } else {
                    writeln!(f, "declare {}", name)
                }
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "function int {}:", self.name)?;
        writeln!(f, "    params: ()")?;
        writeln!(f, "    body:\n        {:?}", self.block_items)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.toplevel_items)
    }
}
