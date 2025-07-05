use crate::ast::BlockItem::Decl;
use crate::ast::Declaration::Declare;
use crate::ast::Expr::{Assign, BinOp, Const, Var};
use crate::ast::{BinaryOp, BlockItem, Declaration, Expr, Statement, UnaryOp};
use crate::lexer::Token;
use crate::parser::parse::{expect, expect_ident};

// From highest to lowest precedence (tighter binding first):
// parse_factor             – literals, variables, parentheses, unary operators (-, ~, !)
// parse_term               – *, /, %
// parse_additive_exp       – +, -
// parse_shift_exp          – <<, >>
// parse_relational_exp     – <, >, <=, >=
// parse_equality_exp       – ==, !=
// parse_bitwise_and_exp    – &
// parse_bitwise_xor_exp    – ^
// parse_bitwise_or_exp     – |
// parse_logical_and_exp    – &&
// parse_logical_or_exp     – ||
// parse_conditional_expr   – e1 ? e2 : e3
// parse_expr               – assignment (=, +=, -=, etc.), ++, --
// parse_statements         – return, if, expression, block, etc.

// helper
fn parse_binary_chain(
    tokens: &[Token],
    pos: &mut usize,
    parse_operand: fn(&[Token], &mut usize) -> Result<Expr, String>,
    match_op: fn(&Token) -> Option<BinaryOp>,
) -> Result<Expr, String> {
    let mut left = parse_operand(tokens, pos)?;

    while let Some(token) = tokens.get(*pos) {
        if let Some(op) = match_op(token) {
            *pos += 1;
            let right = parse_operand(tokens, pos)?;
            left = BinOp(op, Box::new(left), Box::new(right));
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    match tokens.get(*pos) {
        Some(Token::IntLiteral(n)) => {
            *pos += 1;
            Ok(Const(*n))
        }

        Some(Token::LParen) => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            if tokens.get(*pos) == Some(&Token::RParen) {
                *pos += 1;
                Ok(expr)
            } else {
                Err("expected ')'".to_string())
            }
        }

        Some(Token::Minus | Token::Tilde | Token::Bang) => {
            let op = match tokens[*pos] {
                Token::Minus => UnaryOp::Neg,
                Token::Tilde => UnaryOp::BitNot,
                Token::Bang => UnaryOp::Not,
                _ => unreachable!(),
            };
            *pos += 1;
            let inner = parse_factor(tokens, pos)?;
            Ok(Expr::UnOp(op, Box::new(inner)))
        }

        Some(Token::Identifier(name)) => {
            *pos += 1;
            Ok(Var(name.clone()))
        }

        other => Err(format!("expected factor, found {:?}", other)),
    }
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut expr = parse_factor(tokens, pos)?;

    while let Some(op_token) = tokens.get(*pos) {
        let op = match op_token {
            Token::Asterisk => BinaryOp::Multiply,
            Token::Slash => BinaryOp::Divide,
            Token::Modulo => BinaryOp::Modulo,
            _ => break, // not a term-level operator, stop looping
        };

        *pos += 1;
        let rhs = parse_factor(tokens, pos)?;
        expr = BinOp(op, Box::new(expr), Box::new(rhs));
    }

    Ok(expr)
}

fn parse_additive_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_term, |tok| match tok {
        Token::Plus => Some(BinaryOp::Add),
        Token::Minus => Some(BinaryOp::Sub),
        _ => None,
    })
}

fn parse_shift_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_additive_exp, |tok| match tok {
        Token::ShiftRight => Some(BinaryOp::ShiftRight),
        Token::ShiftLeft => Some(BinaryOp::ShiftLeft),
        _ => None,
    })
}

fn parse_relational_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_shift_exp, |tok| match tok {
        Token::Less => Some(BinaryOp::Less),
        Token::LessEqual => Some(BinaryOp::LessEqual),
        Token::Greater => Some(BinaryOp::Greater),
        Token::GreaterEqual => Some(BinaryOp::GreaterEqual),
        _ => None,
    })
}

fn parse_equality_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_relational_exp, |tok| match tok {
        Token::EqualEqual => Some(BinaryOp::Equal),
        Token::BangEqual => Some(BinaryOp::NotEqual),
        _ => None,
    })
}

fn parse_bitwise_and_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_equality_exp, |tok| match tok {
        Token::And => Some(BinaryOp::And),
        _ => None,
    })
}

fn parse_bitwise_xor_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_bitwise_and_exp, |tok| match tok {
        Token::Xor => Some(BinaryOp::Xor),
        _ => None,
    })
}

fn parse_bitwise_or_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_bitwise_xor_exp, |tok| match tok {
        Token::Or => Some(BinaryOp::Or),
        _ => None,
    })
}

fn parse_logical_and_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_bitwise_or_exp, |tok| match tok {
        Token::AndAnd => Some(BinaryOp::LogicalAnd),
        _ => None,
    })
}

fn parse_logical_or_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_logical_and_exp, |tok| match tok {
        Token::OrOr => Some(BinaryOp::LogicalOr),
        _ => None,
    })
}

fn parse_conditional_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let condition = parse_logical_or_exp(tokens, pos)?;

    if tokens.get(*pos) == Some(&Token::QuestionMark) {
        *pos += 1;
        let then_expr = parse_expr(tokens, pos)?;
        expect(tokens, pos, &Token::Colon)?;
        let else_expr = parse_conditional_expr(tokens, pos)?; // right-associative
        Ok(Expr::Conditional {
            cond: Box::new(condition),
            then: Box::new(then_expr),
            els: Box::new(else_expr),
        })
    } else {
        Ok(condition)
    }
}

fn assign_bin_op(op: BinaryOp, var_name: &str, expr: Expr) -> Expr {
    Assign(
        var_name.to_string(),
        Box::new(BinOp(
            op,
            Box::new(Var(var_name.to_string())),
            Box::new(expr),
        )),
    )
}

fn token_to_binop(token: &Token) -> Option<BinaryOp> {
    match token {
        Token::PlusEqual => Some(BinaryOp::Add),
        Token::MinusEqual => Some(BinaryOp::Sub),
        Token::AsteriskEqual => Some(BinaryOp::Multiply),
        Token::SlashEqual => Some(BinaryOp::Divide),
        Token::ModuloEqual => Some(BinaryOp::Modulo),
        Token::OrEqual => Some(BinaryOp::Or),
        Token::AndEqual => Some(BinaryOp::And),
        Token::XorEqual => Some(BinaryOp::Xor),
        Token::ShiftLeftEqual => Some(BinaryOp::ShiftLeft),
        Token::ShiftRightEqual => Some(BinaryOp::ShiftRight),
        _ => None,
    }
}

pub fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if let Some(Token::Identifier(name)) = tokens.get(*pos) {
        match tokens.get(*pos + 1) {
            Some(&Token::Equal) => {
                *pos += 2;
                let rhs = parse_expr(tokens, pos)?;
                return Ok(Assign(name.to_string(), Box::new(rhs)));
            }
            Some(&Token::PlusPlus) => {
                *pos += 2;
                return Ok(assign_bin_op(BinaryOp::Add, name, Const(1)));
            }
            Some(&Token::MinusMinus) => {
                *pos += 2;
                return Ok(assign_bin_op(BinaryOp::Sub, name, Const(1)));
            }

            Some(token) => {
                if let Some(bin_op) = token_to_binop(token) {
                    *pos += 2;
                    let rhs = parse_expr(tokens, pos)?;
                    return Ok(assign_bin_op(bin_op, name, rhs));
                }
            }

            _ => {}
        }
    }

    parse_conditional_expr(tokens, pos)
}

fn parse_declaration(tokens: &[Token], pos: &mut usize) -> Result<Declaration, String> {
    let name = expect_ident(tokens, pos)?;
    let expr = if tokens.get(*pos) == Some(&Token::Equal) {
        *pos += 1;
        Some(parse_expr(tokens, pos)?)
    } else {
        None
    };
    Ok(Declare(name, expr))
}

fn parse_for_declaration(tokens: &[Token], pos: &mut usize) -> Result<Declaration, String> {
    let decl = parse_declaration(tokens, pos)?;

    match tokens.get(*pos) {
        Some(Token::Semicolon) => {
            *pos += 1;
            Ok(decl)
        }
        Some(Token::Comma) => Err("Only one declaration allowed in for-loop initializer".into()),
        Some(other) => Err(format!("Unexpected token after declaration: {:?}", other)),
        None => Err("Unexpected end of input after declaration".into()),
    }
}

fn parse_declaration_list(tokens: &[Token], pos: &mut usize) -> Result<Vec<BlockItem>, String> {
    let mut decls = Vec::new();

    loop {
        let decl = parse_declaration(tokens, pos)?;
        decls.push(Decl(decl));

        match tokens.get(*pos) {
            Some(Token::Comma) => {
                *pos += 1;
            }
            Some(Token::Semicolon) => {
                *pos += 1;
                break;
            }
            Some(other) => {
                return Err(format!("Unexpected token in declaration list: {:?}", other));
            }
            None => {
                return Err("Unexpected end of input in declaration list".into());
            }
        }
    }

    Ok(decls)
}

/// Used in `do { ... } while ( <expr> )`
fn parse_expr_in_parens(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let e = parse_expr(tokens, pos)?;
    expect(tokens, pos, &Token::RParen)?;
    Ok(e)
}

fn parse_for_cond(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    match tokens.get(*pos) {
        Some(Token::Semicolon) => {
            *pos += 1;
            Ok(Const(1))
        }
        _ => {
            let e = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(e)
        }
    }
}

pub fn parse_statement(tokens: &[Token], pos: &mut usize) -> Result<Statement, String> {
    match tokens.get(*pos) {
        Some(Token::KeywordReturn) => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Statement::Return(expr))
        }
        Some(Token::KeywordBingus) => {
            *pos += 1;
            expect(tokens, pos, &Token::LParen)?;
            let expr = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::RParen)?;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Statement::Bingus(expr))
        }
        Some(Token::KeywordIf) => {
            *pos += 1;
            expect(tokens, pos, &Token::LParen)?;
            let condition = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::RParen)?;
            let if_branch = Box::new(parse_statement(tokens, pos)?);

            let else_branch = if let Some(Token::KeywordElse) = tokens.get(*pos) {
                *pos += 1;
                Some(Box::new(parse_statement(tokens, pos)?))
            } else {
                None
            };

            Ok(Statement::If {
                cond: condition,
                then: if_branch,
                els: else_branch,
            })
        }
        Some(Token::KeywordFor) => {
            *pos += 1;
            expect(tokens, pos, &Token::LParen)?;

            // ForDecl case
            if tokens.get(*pos) == Some(&Token::KeywordInt) {
                *pos += 1;
                let decl = parse_for_declaration(tokens, pos)?; // this consumes semicolon

                let cond = parse_for_cond(tokens, pos)?;

                let post = if tokens.get(*pos) != Some(&Token::RParen) {
                    Some(parse_expr(tokens, pos)?)
                } else {
                    None
                };
                expect(tokens, pos, &Token::RParen)?;

                let body = Box::new(parse_statement(tokens, pos)?);

                Ok(Statement::ForDecl {
                    decl,
                    cond,
                    post,
                    body,
                })
            } else {
                // For (expression-based)
                let init = if tokens.get(*pos) == Some(&Token::Semicolon) {
                    *pos += 1;
                    None
                } else {
                    let e = parse_expr(tokens, pos)?;
                    expect(tokens, pos, &Token::Semicolon)?;
                    Some(e)
                };

                let cond = parse_for_cond(tokens, pos)?;

                let post = if tokens.get(*pos) != Some(&Token::RParen) {
                    Some(parse_expr(tokens, pos)?)
                } else {
                    None
                };
                expect(tokens, pos, &Token::RParen)?;

                let body = Box::new(parse_statement(tokens, pos)?);

                Ok(Statement::For {
                    init,
                    cond,
                    post,
                    body,
                })
            }
        }
        Some(Token::KeywordWhile) => {
            *pos += 1;
            expect(tokens, pos, &Token::LParen)?;

            let cond = if tokens.get(*pos) == Some(&Token::RParen) {
                Const(1)
            } else {
                parse_expr(tokens, pos)?
            };

            expect(tokens, pos, &Token::RParen)?;

            let body = Box::new(parse_statement(tokens, pos)?);
            Ok(Statement::While { cond, body })
        }

        Some(Token::KeywordDo) => {
            *pos += 1;

            let body = Box::new(parse_statement(tokens, pos)?);

            expect(tokens, pos, &Token::KeywordWhile)?;
            expect(tokens, pos, &Token::LParen)?;
            let cond = parse_expr_in_parens(tokens, pos)?;
            expect(tokens, pos, &Token::Semicolon)?;

            Ok(Statement::Do { body, cond })
        }
        Some(Token::KeywordContinue) => {
            *pos += 1;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Statement::Continue)
        }
        Some(Token::KeywordBreak) => {
            *pos += 1;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Statement::Break)
        }
        Some(Token::LBrace) => {
            // begin compound block
            *pos += 1;
            let mut block_items = Vec::new();
            while tokens.get(*pos) != Some(&Token::RBrace) {
                let mut parsed = parse_block_items(tokens, pos)?;
                block_items.append(&mut parsed);
            }
            expect(tokens, pos, &Token::RBrace)?;
            Ok(Statement::Compound(block_items))
        }
        Some(Token::RBrace) => Err("Unexpected }".into()),
        Some(Token::Semicolon) => {
            *pos += 1;
            Ok(Statement::Expr(None))
        }
        _ => {
            let expr = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Statement::Expr(Some(expr)))
        }
    }
}

pub fn parse_block_items(tokens: &[Token], pos: &mut usize) -> Result<Vec<BlockItem>, String> {
    match tokens.get(*pos) {
        Some(Token::KeywordInt) => {
            *pos += 1;
            let decls = parse_declaration_list(tokens, pos)?;
            Ok(decls)
        }

        _ => {
            let stmt = parse_statement(tokens, pos)?;
            Ok(vec![BlockItem::Stmt(stmt)])
        }
    }
}
