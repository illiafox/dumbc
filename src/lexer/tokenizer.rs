use crate::lexer::Token;
use std::iter::Peekable;
use std::str::Chars;

fn is_identifier_char(c: &char) -> bool {
    c.is_alphabetic() || *c == '_'
}

fn consume_until<F>(chars: &mut Peekable<Chars>, condition: F) -> String
where
    F: Fn(&char) -> bool,
{
    let mut ident = String::new();

    while let Some(next_ch) = chars.peek() {
        if !condition(next_ch) {
            break;
        }
        ident.push(*next_ch);
        chars.next();
    }

    ident
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if is_identifier_char(&ch) {
            let ident = consume_until(&mut chars, is_identifier_char);
            match ident.as_str() {
                "int" => tokens.push(Token::KeywordInt),
                "return" => tokens.push(Token::KeywordReturn),
                _ => tokens.push(Token::Identifier(ident)),
            }
            continue;
        }

        if ch.is_ascii_digit() {
            let num = consume_until(&mut chars, char::is_ascii_digit);
            let value: i32 = num.parse().map_err(|_| "Invalid integer")?;
            tokens.push(Token::IntLiteral(value));
            continue;
        }

        match ch {
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RBrace);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            _ => return Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    Ok(tokens)
}

#[test]
fn test_lexer() {
    let code = "int main() { return 42; }";
    let tokens = lex(code).unwrap();

    assert_eq!(
        tokens,
        vec![
            Token::KeywordInt,
            Token::Identifier("main".into()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::KeywordReturn,
            Token::IntLiteral(42),
            Token::Semicolon,
            Token::RBrace,
        ]
    );
}
