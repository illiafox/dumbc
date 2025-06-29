use crate::ast::{Expr, Program, Stmt, UnaryOp};
use std::fmt;
use std::fmt::{Error, Write};

pub fn generate_expr(output: &mut dyn Write, expr: &Expr) -> fmt::Result {
    match expr {
        Expr::Const(n) => {
            writeln!(output, "mov\tw0, #{}", n)?;
        }
        Expr::UnOp(op, inner) => {
            generate_expr(output, inner)?; // recursively evaluate into w0

            match op {
                UnaryOp::Neg => writeln!(output, "neg\tw0, w0")?,
                UnaryOp::BitNot => writeln!(output, "mvn\tw0, w0")?,
                UnaryOp::Not => {
                    // sets condition flags
                    writeln!(output, "cmp\tw0, #0")?;
                    // clear w0
                    writeln!(output, "mov\tw0, #0")?;
                    // set w0 = 1 if w0 was equal to 0
                    writeln!(output, "cset\tw0, eq")?;
                }
            }
        }
    }

    Ok(())
}

pub fn generate(program: &Program) -> Result<String, Error> {
    let function = &program.function;
    let mut output = String::new();
    use std::fmt::Write;

    writeln!(output, ".globl _{}", function.name)?;
    writeln!(output, "_{}:", function.name)?;

    writeln!(output, "sub	sp, sp, #16")?; // reserve stack space

    match &function.body {
        Stmt::Return(expr) => {
            // no extra handling for return
            // as long as generate_expr ends with w0 containing the correct result,
            // the ret instruction will return it
            generate_expr(&mut output, expr)?;
        }
    }

    writeln!(output, "add	sp, sp, #16")?; // clean up stack

    writeln!(output, "ret")?;

    Ok(output)
}
