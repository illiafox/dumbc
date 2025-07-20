use crate::ast::Expr::{Assign, BinOp, Conditional, Const, FunCall, UnOp, Var};
use crate::ast::{BinaryOp, Expr, UnaryOp};
use std::error::Error;

pub fn evaluate_compile_time_expr(expr: &Expr) -> Result<i32, Box<dyn Error>> {
    match expr {
        Const(n) => Ok(*n),

        Var(name) => {
            Err(format!("variables not supported in compile-time evaluation: {name}").into())
        }

        UnOp(op, inner) => {
            let inner_val = evaluate_compile_time_expr(inner)?;
            match op {
                UnaryOp::Neg => Ok(-inner_val),
                UnaryOp::BitNot => Ok(!inner_val),
                UnaryOp::Not => Ok((inner_val == 0) as i32),
            }
        }

        BinOp(op, lhs, rhs) => {
            let l_val = evaluate_compile_time_expr(lhs)?;
            let r_val = evaluate_compile_time_expr(rhs)?;
            match op {
                BinaryOp::Add => Ok(l_val + r_val),
                BinaryOp::Sub => Ok(l_val - r_val),
                BinaryOp::Multiply => Ok(l_val * r_val),
                BinaryOp::And => Ok(l_val & r_val),
                BinaryOp::Or => Ok(l_val | r_val),
                BinaryOp::Xor => Ok(l_val ^ r_val),
                BinaryOp::LogicalAnd => Ok(((l_val != 0) && (r_val != 0)) as i32),
                BinaryOp::LogicalOr => Ok(((l_val != 0) || (r_val != 0)) as i32),
                BinaryOp::Equal => Ok((l_val == r_val) as i32),
                BinaryOp::NotEqual => Ok((l_val != r_val) as i32),
                BinaryOp::Less => Ok((l_val < r_val) as i32),
                BinaryOp::LessEqual => Ok((l_val <= r_val) as i32),
                BinaryOp::Greater => Ok((l_val > r_val) as i32),
                BinaryOp::GreaterEqual => Ok((l_val >= r_val) as i32),
                BinaryOp::Divide => {
                    if r_val == 0 {
                        return Err("division by zero in constant expression".into());
                    }
                    Ok(l_val.wrapping_div(r_val))
                }
                BinaryOp::Modulo => {
                    if r_val == 0 {
                        return Err("modulo by zero in constant expression".into());
                    }
                    Ok(l_val.wrapping_rem(r_val))
                }
                BinaryOp::ShiftLeft => Ok(l_val.wrapping_shl(r_val as u32)),
                BinaryOp::ShiftRight => Ok(((l_val as u32) >> (r_val & 31)) as i32),
            }
        }

        Assign(_, _) => Err("assignment is not allowed in compile-time expressions".into()),

        Conditional { cond, then, els } => {
            let cond_val = evaluate_compile_time_expr(cond)?;
            if cond_val != 0 {
                evaluate_compile_time_expr(then)
            } else {
                evaluate_compile_time_expr(els)
            }
        }

        FunCall { .. } => Err("function calls are not allowed in compile-time expressions".into()),
    }
}
