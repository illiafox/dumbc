use crate::ast::Expr::BinOp;
use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use std::fmt;
use std::fmt::Write;

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
        BinOp(op, lhs, rhs) => {
            generate_expr(output, lhs)?;
            writeln!(output, "str\tw0, [sp, #-16]!")?; // push w0 on stack

            generate_expr(output, rhs)?;
            writeln!(output, "ldr\tw1, [sp], #16")?; // pop previous w0 result into w1

            // w0 - result of evaluating rhs
            // w1 - result of evaluating lhs

            match op {
                BinaryOp::Add => writeln!(output, "add\tw0, w1, w0")?,
                BinaryOp::Sub => writeln!(output, "sub\tw0, w1, w0")?,
                BinaryOp::Multiply => writeln!(output, "mul\tw0, w1, w0")?,
                BinaryOp::Divide => writeln!(output, "sdiv\tw0, w1, w0")?,
            }
        }
    }

    Ok(())
}

pub fn generate(program: &Program, platform: &str) -> Result<String, Box<dyn std::error::Error>> {
    let function = &program.function;
    let mut output = String::new();
    use std::fmt::Write;

    let prefix = match platform {
        "macos" => "_", // macOS (Mach-O), label _main
        "linux" => "",  // Linux (ELF), label main
        _ => return Err(format!("Unsupported platform {platform}").into()),
    };
    writeln!(output, ".global {}main", prefix)?;
    writeln!(output, "{}main:", prefix)?;
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
