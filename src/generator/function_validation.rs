use crate::ast::{BlockItem, Declaration, Expr, Program, TopLevel};
use std::collections::HashMap;

enum FuncKind {
    Decl(usize), // parameters
    Def(usize),  // parameters
}

/// Validates the semantic correctness of function declarations, definitions, and calls.
///
/// This pass checks for the following errors:
/// - Multiple definitions of the same function.
/// - Inconsistent parameter counts across declarations and definitions.
/// - Function calls with the wrong number of arguments.
/// - Calls to undefined functions.
///   Should be called after parsing and before code generation.
pub fn validate_functions_declarations(program: &Program) -> Result<(), String> {
    let mut function_map: HashMap<String, FuncKind> = HashMap::new();

    for item in &program.toplevel_items {
        match item {
            TopLevel::Function(func) => {
                let arity = func.params.len();

                match function_map.get(&func.name) {
                    Some(FuncKind::Def(_)) if func.block_items.is_some() => {
                        return Err(format!("function {} defined multiple times", func.name));
                    }

                    Some(FuncKind::Decl(existing_arity)) | Some(FuncKind::Def(existing_arity)) => {
                        if *existing_arity != arity {
                            return Err(format!(
                                "function {} declared/defined with inconsistent parameter counts ({:?} vs {:?})",
                                func.name, existing_arity, arity
                            ));
                        }
                        // if consistent, do nothing
                    }

                    None => {
                        let kind = if func.block_items.is_some() {
                            FuncKind::Def(func.params.len())
                        } else {
                            FuncKind::Decl(func.params.len())
                        };
                        function_map.insert(func.name.clone(), kind);
                    }
                }
            }
            TopLevel::GlobalVariable(_) => {}
        }
    }

    // Second pass: check function calls
    for item in &program.toplevel_items {
        match item {
            TopLevel::Function(func) => {
                if let Some(body) = &func.block_items {
                    validate_function_body(body, &function_map)?;
                }
            }
            TopLevel::GlobalVariable(_) => {}
        }
    }

    Ok(())
}

/// Validates the function body to ensure all function calls are semantically correct.
///
/// Checks every expression in the given block for:
/// - Calls to functions that have not been declared or defined.
/// - Calls with an incorrect number of arguments (arity mismatch).
///
/// This function is called once per function that has a body (i.e., not just a declaration).
fn validate_function_body(
    block_items: &[BlockItem],
    function_map: &HashMap<String, FuncKind>,
) -> Result<(), String> {
    use crate::ast::{BlockItem, Expr, Statement};

    fn check_expr(expr: &Expr, function_map: &HashMap<String, FuncKind>) -> Result<(), String> {
        match expr {
            Expr::FunCall { name, parameters } => {
                match function_map.get(name) {
                    Some(FuncKind::Decl(arity)) | Some(FuncKind::Def(arity)) => {
                        if parameters.len() != *arity {
                            return Err(format!(
                                "function call to `{}` has wrong number of arguments: expected {}, got {}",
                                name,
                                arity,
                                parameters.len()
                            ));
                        }
                    }
                    None => {
                        return Err(format!("call to undefined function `{}`", name));
                    }
                }
                for arg in parameters {
                    check_expr(arg, function_map)?; // recurse
                }
            }

            Expr::Assign(_, e) => check_expr(e, function_map)?,
            Expr::UnOp(_, e) => check_expr(e, function_map)?,
            Expr::BinOp(_, l, r) => {
                check_expr(l, function_map)?;
                check_expr(r, function_map)?;
            }
            Expr::Conditional { cond, then, els } => {
                check_expr(cond, function_map)?;
                check_expr(then, function_map)?;
                check_expr(els, function_map)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn check_stmt(
        stmt: &Statement,
        function_map: &HashMap<String, FuncKind>,
    ) -> Result<(), String> {
        match stmt {
            Statement::Expr(Some(e)) => check_expr(e, function_map),
            Statement::Expr(None) => Ok(()),
            Statement::Return(e) => check_expr(e, function_map),
            Statement::If { cond, then, els } => {
                check_expr(cond, function_map)?;
                check_stmt(then, function_map)?;
                if let Some(els) = els {
                    check_stmt(els, function_map)?;
                }
                Ok(())
            }
            Statement::While { cond, body } | Statement::Do { cond, body } => {
                check_expr(cond, function_map)?;
                check_stmt(body, function_map)
            }
            Statement::For {
                init,
                cond,
                post,
                body,
            } => {
                if let Some(init) = init {
                    check_expr(init, function_map)?;
                }
                check_expr(cond, function_map)?;
                if let Some(post) = post {
                    check_expr(post, function_map)?;
                }
                check_stmt(body, function_map)
            }
            Statement::ForDecl {
                decl: _,
                cond,
                post,
                body,
            } => {
                check_expr(cond, function_map)?;
                if let Some(post) = post {
                    check_expr(post, function_map)?;
                }
                check_stmt(body, function_map)
            }
            Statement::Compound(items) => {
                for item in items {
                    match item {
                        BlockItem::Stmt(s) => check_stmt(s, function_map)?,
                        BlockItem::Decl(_) => {}
                    }
                }
                Ok(())
            }
            Statement::Break | Statement::Continue | Statement::Bingus(_) => Ok(()),
        }
    }

    for item in block_items {
        if let BlockItem::Stmt(stmt) = item {
            check_stmt(stmt, function_map)?;
        }
    }

    Ok(())
}

pub fn check_global_name_conflicts(program: &Program) -> Result<(), String> {
    let mut function_names: HashMap<&String, bool> = HashMap::new(); // func name -> has_definition
    let mut global_var_names: HashMap<&String, &Option<Expr>> = HashMap::new();

    for item in &program.toplevel_items {
        match item {
            TopLevel::Function(func) => {
                let has_definition = func.block_items.is_some();

                if let Some(&defined) = function_names.get(&func.name) {
                    if defined && has_definition {
                        return Err(format!("function '{}' defined twice", func.name));
                    }
                } else {
                    function_names.insert(&func.name, has_definition);
                }
            }
            TopLevel::GlobalVariable(Declaration::Declare(name, expr)) => {
                if let Some(&var) = global_var_names.get(name) {
                    if var.is_some() {
                        return Err(format!("Duplicate global variable definition: '{}'", name));
                    }
                }

                global_var_names.insert(name, expr);
            }
        }
    }

    // Check for conflicts
    for (name, _) in global_var_names {
        if function_names.contains_key(name) {
            return Err(format!(
                "Name conflict: '{}' are used as both a global variable and a function",
                name
            ));
        }
    }

    Ok(())
}
