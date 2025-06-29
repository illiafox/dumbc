use crate::ast::{Function, Program, Stmt};
use crate::lexer::Token;
use crate::parser::expr::parse_expr;

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

    let value = parse_expr(tokens, &mut pos)?;

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
    use crate::ast::Expr;
    use crate::lexer::lex;

    #[test]
    fn test_parse_simple_return() {
        let input = "int main() { return 42; }";
        let tokens = lex(input).expect("Lexer failed");
        let program = parse(&tokens).expect("Parser failed");

        assert_eq!(program.function.name, "main");

        match program.function.body {
            Stmt::Return(Expr::Const(n)) => assert_eq!(n, 42),
            other => panic!("unexpected AST: {:?}", other),
        }
    }

    #[test]
    fn test_return_large_number() {
        let tokens = lex("int main() { return 123456; }").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.function.body {
            Stmt::Return(Expr::Const(n)) => assert_eq!(n, 123456),
            other => panic!("unexpected AST: {:?}", other),
        }
    }

    #[test]
    fn test_missing_semicolon() {
        let tokens = lex("int main() { return 42 }").unwrap();
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_missing_return_value() {
        let tokens = lex("int main() { return ; }").unwrap();
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_unexpected_token() {
        let tokens = lex("int main() { return xyz; }").unwrap();
        assert!(parse(&tokens).is_err()); // 'xyz' is not a valid literal
    }

    #[test]
    fn test_weird_spacing() {
        let tokens = lex("int    main   (  )  {   return    1 ; }").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.function.body {
            Stmt::Return(Expr::Const(n)) => assert_eq!(n, 1),
            other => panic!("unexpected AST: {:?}", other),
        }
    }

    #[test]
    fn test_newlines() {
        let code = r#"
        int main()
        {
            return 5;
        }
    "#;
        let tokens = lex(code).unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.function.body {
            Stmt::Return(Expr::Const(n)) => assert_eq!(n, 5),
            other => panic!("unexpected AST: {:?}", other),
        }
    }

    #[test]
    fn test_large_integer() {
        let tokens = lex(&format!("int main() {{ return {}; }}", i32::MAX)).unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.function.body {
            Stmt::Return(Expr::Const(n)) => assert_eq!(n, i32::MAX),
            other => panic!("unexpected AST: {:?}", other),
        }
    }

    #[test]
    fn test_too_large_integer() {
        let result = lex("int main() { return 99999999999; }");
        assert!(result.is_err()); // Lexing should fail on overflow
    }

    #[test]
    fn test_precedence() {
        let tokens = lex("int main() { return 2 + 3 * 4; }").unwrap();
        let ast = parse(&tokens).unwrap();
        println!("{:#?}", ast);
    }
}
