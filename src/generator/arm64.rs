use crate::ast::BlockItem::{Decl, Stmt};
use crate::ast::Declaration::Declare;
use crate::ast::Expr::{Assign, BinOp, Conditional};
use crate::ast::{BinaryOp, BlockItem, Expr, Program, Statement, UnaryOp};
use crate::generator::allocator::{Allocator, Variable};
use crate::generator::label::LabelGenerator;
use std::error::Error;
use std::fmt;
use std::fmt::Write;

impl Variable {
    pub fn emit_store_in_w0(&self, output: &mut dyn Write) -> fmt::Result {
        match self {
            Variable::Register(reg) => {
                writeln!(output, "mov\tw0, {}", reg)
            }
            Variable::Stack(offset) => {
                writeln!(output, "ldr\tw0, [x29, #{:+}]", offset)
            }
        }
    }

    pub fn emit_store_from_w0(&self, output: &mut dyn Write) -> fmt::Result {
        match self {
            Variable::Register(reg) => {
                writeln!(output, "mov\t{}, w0", reg)
            }
            Variable::Stack(offset) => {
                writeln!(output, "str\tw0, [x29, #{:+}]", offset)
            }
        }
    }
}

struct Generator<'a> {
    output: &'a mut dyn Write,
    labels: &'a mut LabelGenerator,
    allocator: &'a mut Allocator,
    epilogue: String,
}

fn generate_expr(g: &mut Generator, expr: &Expr) -> Result<(), Box<dyn Error>> {
    match expr {
        Expr::Const(n) => {
            writeln!(g.output, "mov\tw0, #{}", n)?;
        }
        Expr::Var(name) => {
            let var = g
                .allocator
                .get(name)
                .ok_or_else(|| format!("variable {} not found", name))?;
            var.emit_store_in_w0(g.output)?
        }
        Expr::UnOp(op, inner) => {
            generate_expr(g, inner)?; // recursively evaluate into w0

            match op {
                UnaryOp::Neg => writeln!(g.output, "neg\tw0, w0")?,
                UnaryOp::BitNot => writeln!(g.output, "mvn\tw0, w0")?,
                UnaryOp::Not => {
                    // sets condition flags
                    writeln!(g.output, "cmp\tw0, #0")?;
                    // clear w0
                    writeln!(g.output, "mov\tw0, #0")?;
                    // set w0 = 1 if w0 was equal to 0
                    writeln!(g.output, "cset\tw0, eq")?;
                }
            }
        }
        BinOp(BinaryOp::LogicalOr, lhs, rhs) => {
            let true_clause = g.labels.next("or_true");
            let end_clause = g.labels.next("or_end");

            generate_expr(g, lhs)?; // result in w0

            writeln!(g.output, "cmp\tw0, #0")?; // check if lhs is true (non-zero)
            writeln!(g.output, "b.ne\t{}", true_clause)?; // if lhs != 0, short-circuit: result is true

            generate_expr(g, rhs)?; // result in w0
            writeln!(g.output, "cmp\tw0, #0")?; // check if rhs is true (non-zero)
            writeln!(g.output, "cset\tw0, ne")?; // w0 = 1 if rhs != 0, else 0
            writeln!(g.output, "b\t{}", end_clause)?;

            writeln!(g.output, "{}:", true_clause)?;
            writeln!(g.output, "mov\tw0, #1")?; // result is 1
            writeln!(g.output, "{}:", end_clause)?;
        }
        BinOp(BinaryOp::LogicalAnd, lhs, rhs) => {
            let false_clause = g.labels.next("and_false");
            let end_clause = g.labels.next("and_end");

            generate_expr(g, lhs)?; // result in w0

            writeln!(g.output, "cmp\tw0, #0")?; // check if lhs is false (zero)
            writeln!(g.output, "b.eq\t{}", false_clause)?; // if lhs == 0, short-circuit: result is false

            generate_expr(g, rhs)?; // result in w0
            writeln!(g.output, "cmp\tw0, #0")?; // check if rhs is true (non-zero)
            writeln!(g.output, "cset\tw0, ne")?; // w0 = 1 if rhs != 0, else 0
            writeln!(g.output, "b\t{}", end_clause)?;

            writeln!(g.output, "{}:", false_clause)?;
            writeln!(g.output, "mov\tw0, #0")?; // result is 0
            writeln!(g.output, "{}:", end_clause)?;
        }
        BinOp(op, lhs, rhs) => {
            generate_expr(g, lhs)?;
            writeln!(g.output, "str\tw0, [sp, #-16]!")?; // push lhs (keep 16-byte align)

            generate_expr(g, rhs)?;
            writeln!(g.output, "ldr\tw1, [sp], #16")?; /* lhs → w1 */

            // w0 - result of evaluating rhs
            // w1 - result of evaluating lhs

            match op {
                BinaryOp::Add => writeln!(g.output, "add\tw0, w1, w0")?,
                BinaryOp::Sub => writeln!(g.output, "sub\tw0, w1, w0")?,
                BinaryOp::Multiply => writeln!(g.output, "mul\tw0, w1, w0")?,
                BinaryOp::Divide => writeln!(g.output, "sdiv\tw0, w1, w0")?,
                BinaryOp::And => writeln!(g.output, "and\tw0, w1, w0")?,
                BinaryOp::Or => writeln!(g.output, "orr\tw0, w1, w0")?,
                BinaryOp::Xor => writeln!(g.output, "eor\tw0, w1, w0")?,
                BinaryOp::ShiftLeft => writeln!(g.output, "lsl\tw0, w1, w0")?,
                BinaryOp::ShiftRight => writeln!(g.output, "lsr\tw0, w1, w0")?,

                BinaryOp::Modulo => {
                    // USES w2 register
                    writeln!(g.output, "udiv\tw2, w1, w0")?; // w2 = lhs / rhs
                    writeln!(g.output, "msub\tw0, w2, w0, w1")?; // w0 = lhs - w2 * rhs
                }

                BinaryOp::Equal => {
                    writeln!(g.output, "cmp\tw1, w0")?;
                    writeln!(g.output, "cset\tw0, eq")?;
                }
                BinaryOp::NotEqual => {
                    writeln!(g.output, "cmp\tw1, w0")?;
                    writeln!(g.output, "cset\tw0, ne")?;
                }
                BinaryOp::Less => {
                    writeln!(g.output, "cmp\tw1, w0")?;
                    writeln!(g.output, "cset\tw0, lt")?;
                }
                BinaryOp::LessEqual => {
                    writeln!(g.output, "cmp\tw1, w0")?;
                    writeln!(g.output, "cset\tw0, le")?;
                }
                BinaryOp::Greater => {
                    writeln!(g.output, "cmp\tw1, w0")?;
                    writeln!(g.output, "cset\tw0, gt")?;
                }
                BinaryOp::GreaterEqual => {
                    writeln!(g.output, "cmp\tw1, w0")?;
                    writeln!(g.output, "cset\tw0, ge")?;
                }

                op => panic!("op {op} is not supported"),
            }
        }
        Assign(name, expr) => {
            let var = {
                g.allocator
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("assignment to undeclared variable '{}'", name))?
            };
            generate_expr(g, expr)?;
            var.emit_store_from_w0(g.output)?
        } // op => panic!("op {op} is not supported"),

        Conditional(cond, then, els) => {
            let else_label = g.labels.next("_else");
            let post_conditional = g.labels.next("_post_conditional");

            generate_expr(g, cond)?; // evaluate cond (e1)
            writeln!(g.output, "cmp\tw0, #0")?; // compare e1 cond zero
            writeln!(g.output, "beq\t{}", else_label)?; // if e1 == 0 (false), jump to else (e3)

            generate_expr(g, then)?; // evaluate e2
            writeln!(g.output, "b\t{}", post_conditional)?; // skip e3

            writeln!(g.output, "{}:", else_label)?;
            generate_expr(g, els)?; // evaluate else (e3)

            writeln!(g.output, "{}:", post_conditional)?;
        }
    }

    Ok(())
}

fn generate_stmt(g: &mut Generator, stmt: &Statement) -> Result<(), Box<dyn Error>> {
    match stmt {
        Statement::Expr(e) => generate_expr(g, e),
        Statement::Return(r) => {
            generate_expr(g, r)?;
            writeln!(g.output, "b\t{}", g.epilogue).map_err(Into::into)
        }
        Statement::Bingus(expr) => {
            generate_expr(g, expr)?;
            writeln!(g.output, "bl\tbingus")?;
            Ok(())
        }
        Statement::If(cond, then, els) => {
            let else_label = g.labels.next("_else");
            let post_conditional = g.labels.next("_post_conditional");

            generate_expr(g, cond)?; // evaluate cond (e1)
            writeln!(g.output, "cmp\tw0, #0")?; // compare e1 cond zero
            writeln!(g.output, "beq\t{}", else_label)?; // if e1 == 0 (false), jump to else (e3)

            generate_stmt(g, then)?; // evaluate e2
            writeln!(g.output, "b\t{}", post_conditional)?; // skip e3

            writeln!(g.output, "{}:", else_label)?;
            if let Some(els) = els {
                generate_stmt(g, els)?; // evaluate else (e3)
            }
            // if els is None, it would just go to post_conditional
            // TODO: do not emit else_label if els.is_none()

            writeln!(g.output, "{}:", post_conditional)?;
            Ok(())
        }
    }
}

fn generate_block_item(g: &mut Generator, block_item: &BlockItem) -> Result<(), Box<dyn Error>> {
    match block_item {
        Stmt(stmt) => generate_stmt(g, stmt),
        Decl(Declare(name, expr)) => {
            if g.allocator.get(name).is_some() {
                return Err(format!("variable {} is already declared", name).into());
            }

            let var = g.allocator.allocate(name.clone(), 4);
            println!("var {var:?} allocated");
            if let Some(expr) = expr {
                generate_expr(g, expr)?;
                var.emit_store_from_w0(g.output)?;
            }
            Ok(())
        }
    }
}

pub fn generate(program: &Program, platform: &str) -> Result<String, Box<dyn std::error::Error>> {
    let function = &program.function;
    let mut output = String::new();

    let prefix = match platform {
        "macos" => "_",
        "linux" => "",
        _ => return Err(format!("Unsupported platform {platform}").into()),
    };

    let free_use_registers = vec![
        "w19", "w20", "w21", "w22", "w23", "w24", "w25", "w26", "w27", "w28",
    ];

    let mut dry_allocator = Allocator::new(free_use_registers.clone());

    let mut bingus_used = false;

    for stmt in &function.block_items {
        if let Stmt(Statement::Bingus(_)) = stmt {
            bingus_used = true;
        }
        if let Decl(Declare(name, _)) = stmt {
            dry_allocator.allocate(name.clone(), 4);
        }
    }
    let stack_size = ((dry_allocator.total_stack_size() + 15) / 16) * 16; // alignment

    if bingus_used {
        match platform {
            "macos" => {
                let bingus = include_bytes!("bingus_arm64_macos.s");
                let bingus_s = std::str::from_utf8(bingus).expect("bingus.s not UTF-8");
                output.write_str(bingus_s)?;
            }
            _ => return Err(format!("bingus is not supported on platform {platform}").into()),
        }
    }

    writeln!(output, ".global {}main", prefix)?;
    writeln!(output, "{}main:", prefix)?;

    // function prologue
    writeln!(output, "stp\tx29, x30, [sp, #-16]!")?;
    writeln!(output, "mov\tx29, sp")?;
    if stack_size > 0 {
        writeln!(output, "sub\tsp, sp, #{}", stack_size)?;
    }

    let mut labels = LabelGenerator::new();
    let epilogue = labels.next("func_epilogue");

    // codegen pass
    let mut generator = Generator {
        output: &mut output,
        labels: &mut labels,
        allocator: &mut Allocator::new(free_use_registers),
        epilogue: epilogue.clone(),
    };

    let mut saw_return = false;
    for block_item in &function.block_items {
        if matches!(block_item, Stmt(Statement::Return(_))) {
            saw_return = true;
        }
        generate_block_item(&mut generator, block_item)?;
    }

    // emit default return if none provided
    if !saw_return {
        writeln!(generator.output, "mov\tw0, #0")?;
        // fallthrough to epilogue
    }

    // function epilogue
    writeln!(output, "{}:", epilogue)?;
    if stack_size > 0 {
        writeln!(output, "add\tsp, sp, #{}", stack_size)?;
    }
    writeln!(output, "ldp\tx29, x30, [sp], #16")?;
    writeln!(output, "ret")?;

    Ok(output)
}
