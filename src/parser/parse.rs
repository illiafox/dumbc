use crate::ast::{Function, Program};
use crate::lexer::Token;
use crate::parser::expr::parse_block_items;

pub fn expect(tokens: &[Token], pos: &mut usize, expected: &Token) -> Result<(), String> {
    if tokens.get(*pos) == Some(expected) {
        *pos += 1;
        Ok(())
    } else {
        Err(format!(
            "expected {}, found {}",
            expected,
            tokens
                .get(*pos)
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOF".to_string())
        ))
    }
}

pub fn expect_ident(tokens: &[Token], pos: &mut usize) -> Result<String, String> {
    match tokens.get(*pos) {
        Some(Token::Identifier(name)) => {
            *pos += 1;
            Ok(name.clone())
        }
        other => Err(format!("expected identifier, found {:?}", other)),
    }
}

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    let mut pos = 0;

    // match: int main ( ) { return 42 ; }
    expect(tokens, &mut pos, &Token::KeywordInt)?;
    let name = expect_ident(tokens, &mut pos)?;
    expect(tokens, &mut pos, &Token::LParen)?;
    expect(tokens, &mut pos, &Token::RParen)?;

    expect(tokens, &mut pos, &Token::LBrace)?;

    let mut body = Vec::new();

    while tokens.get(pos) != Some(&Token::RBrace) {
        let statements = parse_block_items(tokens, &mut pos)?;
        body.extend(statements);
    }

    expect(tokens, &mut pos, &Token::RBrace)?;

    Ok(Program {
        function: Function {
            name,
            block_items: body,
        },
    })
}
