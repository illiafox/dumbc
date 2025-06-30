use crate::ast::Expr::BinOp;
use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::lexer::Token;
use crate::parser::parse::{expect, expect_ident};
// From highest to lowest (tighter binding first):
// parse_factor – literals, parentheses, unary operators
// parse_term – *, /
// parse_additive_exp – +, -
// parse_relational_exp – <, >, <=, >=
// parse_equality_exp – ==, !=
// parse_logical_and_exp – &&
// parse_expr – || (top-level)

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
            Ok(Expr::Const(*n))
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
            Ok(Expr::Var(name.clone()))
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

fn parse_relational_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_additive_exp, |tok| match tok {
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

fn parse_logical_and_exp(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_binary_chain(tokens, pos, parse_equality_exp, |tok| match tok {
        Token::AndAnd => Some(BinaryOp::LogicalAnd),
        _ => None,
    })
}

pub fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if let Some(Token::Identifier(name)) = tokens.get(*pos) {
        if tokens.get(*pos + 1) == Some(&Token::Equal) {
            let name = name.clone();
            *pos += 2;
            let rhs = parse_expr(tokens, pos)?;
            return Ok(Expr::Assign(name, Box::new(rhs)));
        }
    }

    parse_binary_chain(tokens, pos, parse_logical_and_exp, |tok| match tok {
        Token::OrOr => Some(BinaryOp::LogicalOr),
        _ => None,
    })
}

pub fn parse_statement(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    match tokens.get(*pos) {
        Some(Token::KeywordReturn) => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Stmt::Return(expr))
        }
        Some(Token::KeywordInt) => {
            *pos += 1;
            let name = expect_ident(tokens, pos)?;
            let expr = if tokens.get(*pos) == Some(&Token::Equal) {
                *pos += 1;
                Some(parse_expr(tokens, pos)?)
            } else {
                None
            };
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Stmt::Declare(name, expr))
        }
        _ => {
            let expr = parse_expr(tokens, pos)?;
            expect(tokens, pos, &Token::Semicolon)?;
            Ok(Stmt::Expr(expr))
        }
    }
}
