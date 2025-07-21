use crate::ast::*;
use crate::optimizer::evaluate_expr_compile_time::evaluate_compile_time_expr;

/// Fold as much as possible in an `Expr`.
fn fold_expr(e: &Expr) -> Expr {
    // try constant‑evaluate first
    if let Ok(value) = evaluate_compile_time_expr(e) {
        return Expr::Const(value);
    }

    // otherwise recurse and rebuild only the affected branches
    match e {
        Expr::UnOp(op, inner) => Expr::UnOp(*op, Box::new(fold_expr(inner))),
        Expr::BinOp(op, lhs, rhs) => {
            Expr::BinOp(*op, Box::new(fold_expr(lhs)), Box::new(fold_expr(rhs)))
        }
        Expr::Assign(name, rhs) => Expr::Assign(name.clone(), Box::new(fold_expr(rhs))),
        Expr::Conditional { cond, then, els } => Expr::Conditional {
            cond: Box::new(fold_expr(cond)),
            then: Box::new(fold_expr(then)),
            els: Box::new(fold_expr(els)),
        },
        Expr::FunCall { name, parameters } => Expr::FunCall {
            name: name.clone(),
            parameters: parameters.iter().map(fold_expr).collect(),
        },
        Expr::Var(_) | Expr::Const(_) => e.clone(),
    }
}

/// Fold inside a `Statement`.
fn fold_stmt(s: &Statement) -> Statement {
    match s {
        Statement::Return(e) => Statement::Return(fold_expr(e)),
        Statement::Expr(Some(e)) => Statement::Expr(Some(fold_expr(e))),
        Statement::Expr(None) => s.clone(),

        Statement::If { cond, then, els } => Statement::If {
            cond: fold_expr(cond),
            then: Box::new(fold_stmt(then)),
            els: els.as_ref().map(|st| Box::new(fold_stmt(st))),
        },

        Statement::Compound(items) => {
            Statement::Compound(items.iter().map(fold_block_item).collect())
        }

        Statement::Bingus(e) => Statement::Bingus(fold_expr(e)),

        Statement::For {
            init,
            cond,
            post,
            body,
        } => Statement::For {
            init: init.as_ref().map(fold_expr),
            cond: fold_expr(cond),
            post: post.as_ref().map(fold_expr),
            body: Box::new(fold_stmt(body)),
        },

        Statement::ForDecl {
            decl,
            cond,
            post,
            body,
        } => Statement::ForDecl {
            decl: fold_decl(decl),
            cond: fold_expr(cond),
            post: post.as_ref().map(fold_expr),
            body: Box::new(fold_stmt(body)),
        },

        Statement::While { cond, body } => Statement::While {
            cond: fold_expr(cond),
            body: Box::new(fold_stmt(body)),
        },

        Statement::Do { body, cond } => Statement::Do {
            body: Box::new(fold_stmt(body)),
            cond: fold_expr(cond),
        },

        // simple control‑flow terminals
        Statement::Break | Statement::Continue => s.clone(),
    }
}

fn fold_decl(d: &Declaration) -> Declaration {
    match d {
        Declaration::Declare(name, Some(init)) => {
            Declaration::Declare(name.clone(), Some(fold_expr(init)))
        }
        _ => d.clone(),
    }
}

fn fold_block_item(item: &BlockItem) -> BlockItem {
    match item {
        BlockItem::Stmt(st) => BlockItem::Stmt(fold_stmt(st)),
        BlockItem::Decl(dec) => BlockItem::Decl(fold_decl(dec)),
    }
}

/// Fold inside a `Function`.
fn fold_function(f: &Function) -> Function {
    let new_body = f
        .block_items
        .as_ref()
        .map(|items| items.iter().map(fold_block_item).collect());

    Function {
        name: f.name.clone(),
        params: f.params.clone(),
        block_items: new_body,
    }
}

/// Fold as much as possible in program
pub fn constant_fold(program: &Program) -> Program {
    let items = program
        .toplevel_items
        .iter()
        .map(|tl| match tl {
            TopLevel::Function(func) => TopLevel::Function(fold_function(func)),
            TopLevel::GlobalVariable(dec) => TopLevel::GlobalVariable(fold_decl(dec)),
        })
        .collect();

    Program {
        toplevel_items: items,
    }
}
