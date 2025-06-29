use crate::ast::{Expr, Function, Program, Stmt};
use crate::lexer::Token;

fn expect(tokens: &[Token], pos: &mut usize, expected: &Token) -> Result<(), String> {
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

fn expect_ident(tokens: &[Token], pos: &mut usize) -> Result<String, String> {
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
    expect(tokens, &mut pos, &Token::KeywordReturn)?;

    let value = match tokens.get(pos) {
        Some(Token::IntLiteral(n)) => {
            pos += 1;
            Expr::Const(*n)
        }
        other => return Err(format!("expected int literal, found {:?}", other)),
    };

    expect(tokens, &mut pos, &Token::Semicolon)?;
    expect(tokens, &mut pos, &Token::RBrace)?;

    Ok(Program {
        function: Function {
            name,
            body: Stmt::Return(value),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn test_parse_simple_return() {
        let input = "int main() { return 42; }";
        let tokens = lex(input).expect("Lexer failed");
        let program = parse(&tokens).expect("Parser failed");

        assert_eq!(program.function.name, "main");

        match program.function.body {
            Stmt::Return(Expr::Const(n)) => assert_eq!(n, 42),
        }
    }
}
