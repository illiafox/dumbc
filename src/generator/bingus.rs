use crate::ast::BlockItem::Stmt;
use crate::ast::{BlockItem, Statement};

pub fn is_bingus_used(block_item: &BlockItem) -> bool {
    match block_item {
        Stmt(Statement::Bingus(_)) => true,

        Stmt(Statement::For { body, .. }) => is_bingus_used(&Stmt(*body.clone())),
        Stmt(Statement::ForDecl { body, .. }) => is_bingus_used(&Stmt(*body.clone())),
        Stmt(Statement::While { body, .. }) => is_bingus_used(&Stmt(*body.clone())),
        Stmt(Statement::Do { body, .. }) => is_bingus_used(&Stmt(*body.clone())),

        Stmt(Statement::Compound(block_items)) => block_items.iter().any(is_bingus_used),

        _ => false,
    }
}
