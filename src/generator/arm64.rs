use crate::ast::BlockItem::{Decl, Stmt};
use crate::ast::Declaration::Declare;
use crate::ast::Expr::{Assign, BinOp, Conditional, FunCall};
use crate::ast::Statement::Continue;
use crate::ast::{BinaryOp, BlockItem, Declaration, Expr, Function, Program, Statement, UnaryOp};
use crate::generator::allocator::{Allocator, Variable};
use crate::generator::bingus::is_bingus_used;
use crate::generator::label::LabelGenerator;
use crate::generator::stack::simulate_stack_usage;
use std::collections::HashSet;
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
    allocator: Allocator,
    epilogue: String,
    debug_enabled: bool,

    platform: String,
}

impl Generator<'_> {
    pub fn debug(&self, msg: impl std::fmt::Display) {
        if self.debug_enabled {
            println!("{msg}");
        }
    }
}

struct StackTracker {
    // start_offset: i32,
}

impl StackTracker {
    fn begin(_g: &Generator) -> Self {
        Self {
            // start_offset: g.allocator.total_stack_size(),
        }
    }

    fn end_scope(self, _g: &mut Generator) -> Result<(), std::fmt::Error> {
        // let end_offset = g.allocator.total_stack_size();
        // let diff = end_offset - self.start_offset;
        // println!("diff: {}", diff);
        // if diff > 0 {
        //     writeln!(g.output, "add\tsp, sp, #{}", diff)?;
        // }
        // TODO: add dynamic scope
        Ok(())
    }
}

/// Returns the symbol prefix (e.g. "_" on macOS) used when generating labels for functions.
fn function_label_prefix(platform: &str) -> Result<&str, Box<dyn Error>> {
    match platform {
        "macos" => Ok("_"),
        "linux" => Ok(""),
        _ => Err(format!("Unsupported platform {}", platform).into()),
    }
}

struct Context {
    break_label: Option<String>,
    continue_label: Option<String>,
}

/// Emit *one* arithmetic / logical binary operator.
/// Assumes:
///   • right  operand is already in **w0**
///   • left   operand is already in **w11**
/// Leaves the result in **w0**.
fn emit_binop(g: &mut Generator, op: BinaryOp) -> fmt::Result {
    use BinaryOp::*;
    match op {
        Add => writeln!(g.output, "add\tw0, w11, w0"),
        Sub => writeln!(g.output, "sub\tw0, w11, w0"),
        Multiply => writeln!(g.output, "mul\tw0, w11, w0"),
        Divide => writeln!(g.output, "sdiv\tw0, w11, w0"),
        And => writeln!(g.output, "and\tw0, w11, w0"),
        Or => writeln!(g.output, "orr\tw0, w11, w0"),
        Xor => writeln!(g.output, "eor\tw0, w11, w0"),
        ShiftLeft => writeln!(g.output, "lsl\tw0, w11, w0"),
        ShiftRight => writeln!(g.output, "lsr\tw0, w11, w0"),

        Modulo => {
            writeln!(g.output, "udiv\tw12, w11, w0")?;
            writeln!(g.output, "msub\tw0, w12, w0, w11")
        }

        Equal => {
            writeln!(g.output, "cmp\tw11, w0")?;
            writeln!(g.output, "cset\tw0, eq")
        }
        NotEqual => {
            writeln!(g.output, "cmp\tw11, w0")?;
            writeln!(g.output, "cset\tw0, ne")
        }
        Less => {
            writeln!(g.output, "cmp\tw11, w0")?;
            writeln!(g.output, "cset\tw0, lt")
        }
        LessEqual => {
            writeln!(g.output, "cmp\tw11, w0")?;
            writeln!(g.output, "cset\tw0, le")
        }
        Greater => {
            writeln!(g.output, "cmp\tw11, w0")?;
            writeln!(g.output, "cset\tw0, gt")
        }
        GreaterEqual => {
            writeln!(g.output, "cmp\tw11, w0")?;
            writeln!(g.output, "cset\tw0, ge")
        }

        // logical‑and / or are handled earlier in generate_expr
        LogicalAnd | LogicalOr => unreachable!("short‑circuited ops never reach emit_binop"),
    }
}

/// Emit a *complete* function call, including alignment padding,
/// argument evaluation, the `bl`, and stack clean‑up.
///
///  • `args` are evaluated **left‑to‑right** exactly once each.
///  • The first 8 results go to  w0…w7, the rest are pushed (right‑to‑left).
fn emit_fun_call(g: &mut Generator, name: &str, args: &[Expr]) -> Result<(), Box<dyn Error>> {
    // On OS X, the stack needs to be 16-byte aligned when the call instruction is issued

    // align stack before pushing args
    // push args (right-to-left)
    // emit bl _func
    // cleanup stack (args + padding)

    let n = args.len();
    // # stack args = anything beyond the first 8
    let num_stack_args = n.saturating_sub(8);
    let arg_stack_size = 16 * num_stack_args;

    writeln!(g.output, "mov\tx9, sp")?; // x9 will track the future sp
    writeln!(g.output, "sub\tx9, x9, #{arg_stack_size}")?; // simulate: sp - (args + padding marker)
    writeln!(g.output, "and\tx10, x9, #15")?; // x10 = misalignment = (sp - size) % 16
    writeln!(g.output, "sub\tsp, sp, x10")?; // subtract misalignment to align
    writeln!(g.output, "str\tx10, [sp, #-16]!")?; // save the padding value (push it)

    for (rev_i, param) in args.iter().rev().enumerate() {
        generate_expr(g, param)?; // result in w0

        // real register index for this argument
        let reg = n - 1 - rev_i; // 0 = first param, 1 = second, …

        if reg < 8 {
            writeln!(g.output, "mov\tw{}, w0", reg)?; // copy into its ABI register
        } else {
            // 9‑th and later: go on the stack, highest‑index first
            writeln!(g.output, "str\tw0, [sp, #-16]!")?;
        }
    }

    let prefix = function_label_prefix(&g.platform)?;

    writeln!(g.output, "bl\t{prefix}{name}")?;

    writeln!(g.output, "add\tsp, sp, #{arg_stack_size}")?; // remove args

    writeln!(g.output, "ldr\tx9, [sp], #16")?; // pop off the padding
    writeln!(g.output, "add\tsp, sp, x9")?; // apply padding

    Ok(())
}

fn generate_expr(g: &mut Generator, expr: &Expr) -> Result<(), Box<dyn Error>> {
    match expr {
        Expr::Const(n) => {
            writeln!(g.output, "mov\tw0, #{n}")?;
        }
        Expr::Var(name) => {
            let var = g
                .allocator
                .get(name)
                .ok_or_else(|| format!("variable {name} not found"))?;
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
            writeln!(g.output, "b.ne\t{true_clause}",)?; // if lhs != 0, short-circuit: result is true

            generate_expr(g, rhs)?; // result in w0
            writeln!(g.output, "cmp\tw0, #0")?; // check if rhs is true (non-zero)
            writeln!(g.output, "cset\tw0, ne")?; // w0 = 1 if rhs != 0, else 0
            writeln!(g.output, "b\t{end_clause}",)?;

            writeln!(g.output, "{true_clause}:",)?;
            writeln!(g.output, "mov\tw0, #1")?; // result is 1
            writeln!(g.output, "{end_clause}:",)?;
        }
        BinOp(BinaryOp::LogicalAnd, lhs, rhs) => {
            let false_clause = g.labels.next("and_false");
            let end_clause = g.labels.next("and_end");

            generate_expr(g, lhs)?; // result in w0

            writeln!(g.output, "cmp\tw0, #0")?; // check if lhs is false (zero)
            writeln!(g.output, "b.eq\t{false_clause}",)?; // if lhs == 0, short-circuit: result is false

            generate_expr(g, rhs)?; // result in w0
            writeln!(g.output, "cmp\tw0, #0")?; // check if rhs is true (non-zero)
            writeln!(g.output, "cset\tw0, ne")?; // w0 = 1 if rhs != 0, else 0
            writeln!(g.output, "b\t{end_clause}",)?;

            writeln!(g.output, "{false_clause}:",)?;
            writeln!(g.output, "mov\tw0, #0")?; // result is 0
            writeln!(g.output, "{end_clause}:",)?;
        }
        BinOp(op, lhs, rhs) => {
            // because registers w0–w7 are reserved for the argument list,
            // we must not use any of them as temporaries
            // use w11

            generate_expr(g, lhs)?;
            writeln!(g.output, "str\tw0, [sp, #-16]!")?; // push lhs (keep 16-byte align)

            generate_expr(g, rhs)?;
            writeln!(g.output, "ldr\tw11, [sp], #16")?; /* lhs → w1 */

            // w0 - result of evaluating rhs
            // w11 - result of evaluating lhs

            emit_binop(g, *op)?;
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

        Conditional { cond, then, els } => {
            let else_label = g.labels.next("_else");
            let post_conditional = g.labels.next("_post_conditional");

            generate_expr(g, cond)?; // evaluate cond (e1)
            writeln!(g.output, "cmp\tw0, #0")?; // compare e1 cond zero
            writeln!(g.output, "beq\t{else_label}")?; // if e1 == 0 (false), jump to else (e3)

            generate_expr(g, then)?; // evaluate e2
            writeln!(g.output, "b\t{post_conditional}")?; // skip e3

            writeln!(g.output, "{else_label}:")?;
            generate_expr(g, els)?; // evaluate else (e3)

            writeln!(g.output, "{post_conditional}:")?;
        }

        FunCall { name, parameters } => emit_fun_call(g, name, parameters)?,
    }

    Ok(())
}

fn generate_stmt(
    ctx: &mut Context,
    g: &mut Generator,
    stmt: &Statement,
) -> Result<(), Box<dyn Error>> {
    match stmt {
        Statement::Expr(Some(e)) => generate_expr(g, e),
        Statement::Expr(None) => Ok(()),

        Statement::Return(r) => {
            generate_expr(g, r)?;
            writeln!(g.output, "b\t{}", g.epilogue).map_err(Into::into)
        }
        Statement::Bingus(expr) => {
            generate_expr(g, expr)?;
            writeln!(g.output, "bl\tbingus")?;
            Ok(())
        }
        Statement::If { cond, then, els } => {
            let else_label = g.labels.next("_else");
            let post_conditional = g.labels.next("_post_conditional");

            generate_expr(g, cond)?; // evaluate cond (e1)
            writeln!(g.output, "cmp\tw0, #0")?; // compare e1 cond zero
            writeln!(g.output, "beq\t{}", else_label)?; // if e1 == 0 (false), jump to else (e3)

            generate_stmt(ctx, g, then)?; // evaluate e2
            writeln!(g.output, "b\t{}", post_conditional)?; // skip e3

            writeln!(g.output, "{}:", else_label)?;
            if let Some(els) = els {
                generate_stmt(ctx, g, els)?; // evaluate else (e3)
            }
            // if els is None, it would just go to post_conditional
            // TODO: do not emit else_label if els.is_none()

            writeln!(g.output, "{}:", post_conditional)?;
            Ok(())
        }
        Statement::Compound(block_items) => generate_block(ctx, g, block_items, None),

        Statement::While { cond, body } => {
            let start = g.labels.next("_while");
            let continue_label = g.labels.next("_while_continue");
            let finish = g.labels.next("_while_end");

            writeln!(g.output, "{}:", start)?;
            generate_expr(g, cond)?; // evaluate cond

            writeln!(g.output, "cmp\tw0, #0")?; // compare cond with zero
            writeln!(g.output, "beq\t{}", finish)?; // if cond == 0 (false), jump to finish

            ctx.break_label = Some(finish.clone());
            ctx.continue_label = Some(continue_label.clone());

            writeln!(g.output, "{}:", continue_label)?;
            generate_statement_in_new_scope(ctx, g, body)?; // evaluate body

            writeln!(g.output, "b\t{}", start)?; // jump back to start

            writeln!(g.output, "{}:", finish)?;
            Ok(())
        }

        Statement::Do { cond, body } => {
            let start = g.labels.next("_do_while");
            let continue_label = g.labels.next("_do_while_continue");
            let finish = g.labels.next("_do_while_end");

            writeln!(g.output, "{}:", start)?;

            ctx.break_label = Some(finish.clone());
            ctx.continue_label = Some(continue_label.clone());

            writeln!(g.output, "{}:", continue_label)?;
            generate_statement_in_new_scope(ctx, g, body)?; // evaluate body

            generate_expr(g, cond)?; // evaluate cond
            writeln!(g.output, "cmp\tw0, #0")?; // compare cond with zero
            writeln!(g.output, "beq\t{}", finish)?; // if cond == 0 (false), jump to finish
            writeln!(g.output, "b\t{}", start)?; // jump back to start

            writeln!(g.output, "{}:", finish)?;
            Ok(())
        }

        Statement::For {
            init,
            cond,
            post,
            body,
        } => {
            let start = g.labels.next("_for");
            let continue_label = g.labels.next("_for_continue");
            let finish = g.labels.next("_for_end");

            if let Some(init) = init {
                generate_expr(g, init)?;
            }

            writeln!(g.output, "{}:", start)?;
            generate_expr(g, cond)?; // evaluate cond

            writeln!(g.output, "cmp\tw0, #0")?; // compare cond with zero
            writeln!(g.output, "beq\t{}", finish)?; // if cond == 0 (false), jump to finish

            ctx.break_label = Some(finish.clone());
            ctx.continue_label = Some(continue_label.clone());
            generate_statement_in_new_scope(ctx, g, body)?; // evaluate body

            writeln!(g.output, "{}:", continue_label)?;
            if let Some(post) = post {
                generate_expr(g, post)?; // evaluate post expression
            }
            writeln!(g.output, "b\t{}", start)?; // jump back to start

            writeln!(g.output, "{}:", finish)?;
            Ok(())
        }
        Statement::ForDecl {
            decl,
            cond,
            post,
            body,
        } => {
            let old_allocator = g.allocator.clone();
            let tracker = StackTracker::begin(g); // keep stack accounting

            generate_declaration(g, decl)?; // alloc ‘i’ inside *this* scope

            let start = g.labels.next("_for_decl");
            let continue_label = g.labels.next("_for_decl_continue");
            let finish = g.labels.next("_for_decl_end");

            writeln!(g.output, "{}:", start)?;
            generate_expr(g, cond)?;
            writeln!(g.output, "cmp\tw0, #0")?;
            writeln!(g.output, "beq\t{}", finish)?;

            ctx.break_label = Some(finish.clone());
            ctx.continue_label = Some(continue_label.clone());
            generate_statement_in_new_scope(ctx, g, body)?;

            writeln!(g.output, "{}:", continue_label)?;
            if let Some(post) = post {
                generate_expr(g, post)?;
            }
            writeln!(g.output, "b\t{}", start)?;
            writeln!(g.output, "{}:", finish)?;

            tracker.end_scope(g)?;
            g.allocator = old_allocator;
            Ok(())
        }

        Statement::Break => {
            let label = ctx
                .break_label
                .as_deref()
                .ok_or("`break` used outside of loop")?;
            writeln!(g.output, "b\t{}", label)?;
            Ok(())
        }

        Continue => {
            let label = ctx
                .continue_label
                .as_deref()
                .ok_or("`continue` used outside of loop")?;
            writeln!(g.output, "b\t{}", label)?;
            Ok(())
        }
    }
}

fn generate_declaration(g: &mut Generator, decl: &Declaration) -> Result<(), Box<dyn Error>> {
    match decl {
        Declare(name, expr) => {
            let var = g.allocator.allocate(name.clone(), 4);
            g.debug(format!("var {var:?} allocated"));
            if let Some(expr) = expr {
                generate_expr(g, expr)?;
                var.emit_store_from_w0(g.output)?;
            }
            Ok(())
        }
    }
}

fn generate_block_item(
    ctx: &mut Context,
    g: &mut Generator,
    block_item: &BlockItem,
) -> Result<(), Box<dyn Error>> {
    match block_item {
        Stmt(stmt) => generate_stmt(ctx, g, stmt),
        Decl(decl) => generate_declaration(g, decl),
    }
}

fn generate_statement_in_new_scope(
    ctx: &mut Context,
    g: &mut Generator,
    stmt: &Statement,
) -> Result<(), Box<dyn Error>> {
    let old_allocator = g.allocator.clone();

    let tracker = StackTracker::begin(g);

    generate_stmt(ctx, g, stmt)?;

    tracker.end_scope(g)?;
    g.allocator = old_allocator;

    Ok(())
}

fn generate_block(
    ctx: &mut Context,
    g: &mut Generator,
    items: &[BlockItem],
    outer_scope: Option<&HashSet<String>>,
) -> Result<(), Box<dyn Error>> {
    let old_allocator = g.allocator.clone();

    let mut current_scope = HashSet::new();
    if let Some(outer) = outer_scope {
        current_scope.extend(outer.clone());
    }

    let tracker = StackTracker::begin(g);

    for item in items {
        match item {
            Decl(Declare(name, _)) => {
                if !current_scope.insert(name.clone()) {
                    return Err(format!("variable {} redeclared in same block", name).into());
                }
                generate_block_item(ctx, g, item)?;
            }
            Stmt(stmt) => {
                generate_stmt(ctx, g, stmt)?;
            }
        }
    }

    tracker.end_scope(g)?;
    g.allocator = old_allocator;

    Ok(())
}

fn ends_with_return(item: &BlockItem) -> bool {
    match item {
        Stmt(stmt) => stmt_ends_with_return(stmt),
        Decl(Declare(_, _)) => false,
    }
}

fn stmt_ends_with_return(stmt: &Statement) -> bool {
    match stmt {
        Statement::Return(_) => true,
        Statement::If {
            cond: _,
            then,
            els: Some(else_),
        } => stmt_ends_with_return(then) && stmt_ends_with_return(else_),
        Statement::Compound(items) => items
            .iter()
            .rev()
            .find(|item| matches!(item, Stmt(_)))
            .is_some_and(ends_with_return),
        _ => false,
    }
}

/// Generates
pub fn generate_function(
    function: &Function,
    labels: &mut LabelGenerator,
    platform: &str,
    debug: bool,
) -> Result<String, Box<dyn Error>> {
    if function.block_items.is_none() {
        return Err("cannot generate function declaration".into());
    }

    let mut output = String::new();

    let prefix = function_label_prefix(platform)?;

    let free_use_registers = &[
        "w19", "w20", "w21", "w22", "w23", "w24", "w25", "w26", "w27", "w28",
    ];

    let mut dry_allocator = Allocator::new(free_use_registers);
    let mut max_stack = 0;

    let block_items = function.block_items.as_ref().unwrap();
    simulate_stack_usage(block_items, &mut dry_allocator, &mut max_stack);

    let stack_size = ((max_stack + 15) / 16) * 16; // alignment

    if debug {
        println!("stack size: {stack_size}");
    }

    writeln!(output, ".global {}{}", prefix, function.name)?;
    writeln!(output, "{}{}:", prefix, function.name)?;

    // ---------- function prologue ----------
    writeln!(output, "stp\tx29, x30, [sp, #-16]!")?; // save frame-pointer (x29) and link-register (x30).
    writeln!(output, "mov\tx29, sp")?; // establish new frame pointer

    // save all callee-saved registers (x19-x28) we plan to use for locals
    // (five 128-bit pushes = 80 bytes, keep the order!)

    let x_registers = &[
        // why x19-x28 and not w19-w28? because they are the same physical register, but have different view
        // x19	64 bit	the whole general-purpose register 19
        // w19	32 bit	lower half of that same register
        ["x19", "x20"],
        ["x21", "x22"],
        ["x23", "x24"],
        ["x25", "x26"],
        ["x27", "x28"],
    ];
    for [ra, rb] in x_registers {
        writeln!(output, "stp\t{}, {}, [sp, #-16]!", ra, rb)?;
    }

    if stack_size > 0 {
        writeln!(output, "sub\tsp, sp, #{}", stack_size)?;
    }

    let epilogue = labels.next("func_epilogue");

    // codegen pass
    let mut generator = Generator {
        output: &mut output,
        labels,
        allocator: Allocator::new(free_use_registers.as_slice()),
        epilogue: epilogue.clone(),
        debug_enabled: debug,
        platform: platform.to_string(),
    };

    let saw_return = block_items
        .iter()
        .rev()
        .find(|item| matches!(item, Stmt(_)))
        .is_some_and(ends_with_return);

    let mut ctx = Context {
        break_label: None,
        continue_label: None,
    };

    // assign incoming parameters to allocator and move them from w0–w7 into locals
    for (i, param) in function.params.iter().enumerate() {
        if i >= 8 {
            return Err("more than 8 parameters not supported".into());
        }

        let var = generator.allocator.allocate(param.clone(), 4);
        generator.debug(format!("param {param} -> {var:?}"));

        match var {
            Variable::Register(reg) => {
                writeln!(generator.output, "mov\t{}, w{}", reg, i)?;
            }
            Variable::Stack(offset) => {
                writeln!(generator.output, "str\tw{}, [x29, #{:+}]", i, offset)?;
            }
        }
    }

    let mut top_scope_names = HashSet::new();
    for param in &function.params {
        if !top_scope_names.insert(param.clone()) {
            return Err(format!("duplicate parameter '{}'", param).into());
        }
    }

    generate_block(
        &mut ctx,
        &mut generator,
        block_items,
        Some(&top_scope_names),
    )?;

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

    // restore x27-x28 … x19-x20 (reverse order!)
    for [ra, rb] in x_registers.iter().rev() {
        writeln!(output, "ldp\t{}, {}, [sp], #16", ra, rb)?;
    }

    writeln!(output, "ldp\tx29, x30, [sp], #16")?;
    writeln!(output, "ret")?;

    Ok(output)
}

pub fn generate(program: &Program, platform: &str, debug: bool) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    let bingus_used = program
        .functions
        .iter()
        .filter_map(|f| f.block_items.as_ref())
        .flatten()
        .any(is_bingus_used);
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

    let mut labels = LabelGenerator::new();

    for function in &program.functions {
        if function.block_items.is_none() {
            continue;
        }
        output += &generate_function(function, &mut labels, platform, debug)?;
        output.write_str("\n")?;
    }

    Ok(output)
}
