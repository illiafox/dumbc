use crate::ast::Declaration::Declare;
use crate::ast::{BlockItem, Statement};
use crate::generator::allocator::Allocator;

pub fn simulate_stack_usage(items: &[BlockItem], allocator: &mut Allocator, max: &mut i32) {
    allocator.enter_scope();

    for item in items {
        match item {
            BlockItem::Decl(Declare(name, _)) => {
                allocator.allocate(name.clone(), 4);
                *max = (*max).max(allocator.total_stack_size());
            }
            BlockItem::Stmt(stmt) => simulate_stmt_stack(stmt, allocator, max),
        }
    }

    allocator.exit_scope();
}

fn simulate_stmt_stack(stmt: &Statement, allocator: &mut Allocator, max: &mut i32) {
    match stmt {
        Statement::If { cond: _, then, els } => {
            simulate_stmt_stack(then, allocator, max);
            if let Some(els) = els {
                simulate_stmt_stack(els, allocator, max);
            }
        }
        Statement::Compound(items) => simulate_stack_usage(items, allocator, max),
        _ => {}
    }
}
