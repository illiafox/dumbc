use crate::ast::Expr::BinOp;
use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::generator::label::LabelGenerator;
use std::fmt;
use std::fmt::Write;

pub fn generate_expr(
    output: &mut dyn Write,
    expr: &Expr,
    labels: &mut LabelGenerator,
) -> fmt::Result {
    match expr {
        Expr::Const(n) => {
            writeln!(output, "mov\tw0, #{}", n)?;
        }
        Expr::UnOp(op, inner) => {
            generate_expr(output, inner, labels)?; // recursively evaluate into w0

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
        BinOp(BinaryOp::LogicalOr, lhs, rhs) => {
            let true_clause = labels.next("or_true");
            let end_clause = labels.next("or_end");

            generate_expr(output, lhs, labels)?; // result in w0

            writeln!(output, "cmp\tw0, #0")?; // check if lhs is true (non-zero)
            writeln!(output, "b.ne\t{}", true_clause)?; // if lhs != 0, short-circuit: result is true

            generate_expr(output, rhs, labels)?; // result in w0
            writeln!(output, "cmp\tw0, #0")?; // check if rhs is true (non-zero)
            writeln!(output, "cset\tw0, ne")?; // w0 = 1 if rhs != 0, else 0
            writeln!(output, "b\t{}", end_clause)?;

            writeln!(output, "{}:", true_clause)?;
            writeln!(output, "mov\tw0, #1")?; // result is 1
            writeln!(output, "{}:", end_clause)?;
        }
        BinOp(BinaryOp::LogicalAnd, lhs, rhs) => {
            let false_clause = labels.next("and_false");
            let end_clause = labels.next("and_end");

            generate_expr(output, lhs, labels)?; // result in w0

            writeln!(output, "cmp\tw0, #0")?; // check if lhs is false (zero)
            writeln!(output, "b.eq\t{}", false_clause)?; // if lhs == 0, short-circuit: result is false

            generate_expr(output, rhs, labels)?; // result in w0
            writeln!(output, "cmp\tw0, #0")?; // check if rhs is true (non-zero)
            writeln!(output, "cset\tw0, ne")?; // w0 = 1 if rhs != 0, else 0
            writeln!(output, "b\t{}", end_clause)?;

            writeln!(output, "{}:", false_clause)?;
            writeln!(output, "mov\tw0, #0")?; // result is 0
            writeln!(output, "{}:", end_clause)?;
        }
        BinOp(op, lhs, rhs) => {
            generate_expr(output, lhs, labels)?;
            writeln!(output, "str\tw0, [sp, #-16]!")?; // push w0 on stack

            generate_expr(output, rhs, labels)?;
            writeln!(output, "ldr\tw1, [sp], #16")?; // pop previous w0 result into w1

            // w0 - result of evaluating rhs
            // w1 - result of evaluating lhs

            match op {
                BinaryOp::Add => writeln!(output, "add\tw0, w1, w0")?,
                BinaryOp::Sub => writeln!(output, "sub\tw0, w1, w0")?,
                BinaryOp::Multiply => writeln!(output, "mul\tw0, w1, w0")?,
                BinaryOp::Divide => writeln!(output, "sdiv\tw0, w1, w0")?,

                BinaryOp::Equal => {
                    writeln!(output, "cmp\tw1, w0")?;
                    writeln!(output, "cset\tw0, eq")?;
                }
                BinaryOp::NotEqual => {
                    writeln!(output, "cmp\tw1, w0")?;
                    writeln!(output, "cset\tw0, ne")?;
                }
                BinaryOp::Less => {
                    writeln!(output, "cmp\tw1, w0")?;
                    writeln!(output, "cset\tw0, lt")?;
                }
                BinaryOp::LessEqual => {
                    writeln!(output, "cmp\tw1, w0")?;
                    writeln!(output, "cset\tw0, le")?;
                }
                BinaryOp::Greater => {
                    writeln!(output, "cmp\tw1, w0")?;
                    writeln!(output, "cset\tw0, gt")?;
                }
                BinaryOp::GreaterEqual => {
                    writeln!(output, "cmp\tw1, w0")?;
                    writeln!(output, "cset\tw0, ge")?;
                }

                op => panic!("op {op} is not supported"),
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

    let mut labels = LabelGenerator::new();

    match &function.body {
        Stmt::Return(expr) => {
            // no extra handling for return
            // as long as generate_expr ends with w0 containing the correct result,
            // the ret instruction will return it
            generate_expr(&mut output, expr, &mut labels)?;
        }
    }

    writeln!(output, "add	sp, sp, #16")?; // clean up stack

    writeln!(output, "ret")?;

    Ok(output)
}
