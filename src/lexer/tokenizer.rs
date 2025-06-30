use crate::lexer::Token;
use std::iter::Peekable;
use std::str::Chars;

fn is_identifier_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_'
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

        if ch.is_alphabetic() || ch == '_' {
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

        // skip line comments: //...
        if ch == '/' {
            let mut lookahead = chars.clone();
            lookahead.next();
            if let Some(&next_ch) = lookahead.peek() {
                if next_ch == '/' {
                    // consume both slashes
                    chars.next();
                    chars.next();
                    // skip until newline or end
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                    continue;
                } else if next_ch == '*' {
                    // skip block comment: /* ... */
                    chars.next(); // consume '/'
                    chars.next(); // consume '*'
                    loop {
                        match chars.next() {
                            Some('*') => {
                                if chars.peek() == Some(&'/') {
                                    chars.next(); // consume '/'
                                    break;
                                }
                            }
                            Some(_) => continue,
                            None => return Err("Unterminated block comment".into()),
                        }
                    }
                    continue;
                }
            }
        }

        match ch {
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            '{' => {
                chars.next();
                tokens.push(Token::LBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RBrace);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '~' => {
                chars.next();
                tokens.push(Token::Tilde);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Asterisk);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }

            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }

            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }

            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::BangEqual);
                } else {
                    tokens.push(Token::Bang);
                }
            }

            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }

            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(Token::AndAnd);
                } else {
                    return Err("Unexpected character: '&'".to_string());
                }
            }

            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(Token::OrOr);
                } else {
                    return Err("Unexpected character: '|'".to_string());
                }
            }

            _ => return Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

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

    #[test]
    fn test_lexer_whitespace_variants() {
        let code = " int\tmain (  ) { \nreturn\t42 ; } ";
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

    #[test]
    fn test_lexer_unknown_symbol() {
        let code = "int main() { return 42$; }";
        let result = lex(code);
        assert!(result.is_err());
    }

    #[test]
    fn test_lexer_identifier_with_underscore() {
        let code = "int _main_123() { return 1; }";
        let tokens = lex(code).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::KeywordInt,
                Token::Identifier("_main_123".into()),
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::KeywordReturn,
                Token::IntLiteral(1),
                Token::Semicolon,
                Token::RBrace,
            ]
        );
    }
}
