use crate::ast::{Expr, Program, Stmt};
use std::fmt::Error;

pub fn generate_arm64(program: &Program) -> Result<String, Error> {
    let function = &program.function;
    let mut output = String::new();
    use std::fmt::Write;

    writeln!(output, ".globl _{}", function.name)?;
    writeln!(output, "_{}:", function.name)?;

    writeln!(output, "sub	sp, sp, #16")?; // reserve stack space

    match &function.body {
        Stmt::Return(expr) => {
            match expr {
                Expr::Const(n) => {
                    writeln!(output, "mov	w0, #{}", n)?; // return value
                }
            }
        }
    }

    writeln!(output, "add	sp, sp, #16")?; // clean up stack

    writeln!(output, "ret")?;

    Ok(output)
}
