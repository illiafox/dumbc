use crate::lexer::Token;
use std::iter::Peekable;
use std::str::Chars;

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn consume_until<F>(chars: &mut Peekable<Chars>, condition: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut ident = String::new();

    while let Some(next_ch) = chars.peek() {
        if !condition(*next_ch) {
            break;
        }
        ident.push(*next_ch);
        chars.next();
    }

    ident
}

fn skip_comment_if_present(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    if chars.peek() != Some(&'/') {
        return Ok(false);
    }

    let mut lookahead = chars.clone();
    lookahead.next();
    if let Some(&next_ch) = lookahead.peek() {
        match next_ch {
            '/' => {
                // Line comment
                chars.next(); // consume '/'
                chars.next(); // consume second '/'
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    chars.next();
                }
                return Ok(true);
            }
            '*' => {
                // Block comment
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
                        Some(_) => {}
                        None => return Err("Unterminated block comment".into()),
                    }
                }
                return Ok(true);
            }
            _ => {}
        }
    }

    Ok(false)
}

fn match_operator(chars: &mut Peekable<Chars>, matches: &[(&str, Token)]) -> Option<Token> {
    for (symbol, token) in matches {
        let mut lookahead = chars.clone();

        let matched = symbol
            .chars()
            .all(|expected| lookahead.next().is_some_and(|actual| expected == actual));

        if matched {
            for _ in 0..symbol.len() {
                chars.next(); // consume the matched characters
            }
            return Some(token.clone());
        }
    }

    None
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
                "bingus" => tokens.push(Token::KeywordBingus),
                "int" => tokens.push(Token::KeywordInt),
                "return" => tokens.push(Token::KeywordReturn),
                "if" => tokens.push(Token::KeywordIf),
                "else" => tokens.push(Token::KeywordElse),
                _ => tokens.push(Token::Identifier(ident)),
            }
            continue;
        }

        if ch.is_ascii_digit() {
            let num = consume_until(&mut chars, |c: char| c.is_ascii_digit());
            let value: i32 = num.parse().map_err(|_| "Invalid integer")?;
            tokens.push(Token::IntLiteral(value));
            continue;
        }

        if skip_comment_if_present(&mut chars)? {
            continue;
        }

        if let Some(token) = match_operator(
            &mut chars,
            &[
                (">>=", Token::ShiftRightEqual),
                ("<<=", Token::ShiftLeftEqual),
                ("++", Token::PlusPlus),
                ("--", Token::MinusMinus),
                ("+=", Token::PlusEqual),
                ("-=", Token::MinusEqual),
                ("/=", Token::SlashEqual),
                ("*=", Token::AsteriskEqual),
                ("==", Token::EqualEqual),
                ("!=", Token::BangEqual),
                (">=", Token::GreaterEqual),
                ("<=", Token::LessEqual),
                ("&&", Token::AndAnd),
                ("||", Token::OrOr),
                ("%=", Token::ModuloEqual),
                ("&=", Token::AndEqual),
                ("^=", Token::XorEqual),
                (">>", Token::ShiftRight),
                ("<<", Token::ShiftLeft),
                ("|=", Token::OrEqual),
                ("%", Token::Modulo),
                ("&", Token::And),
                ("|", Token::Or),
                ("^", Token::Xor),
                ("+", Token::Plus),
                ("-", Token::Minus),
                ("*", Token::Asterisk),
                ("/", Token::Slash),
                ("=", Token::Equal),
                ("!", Token::Bang),
                ("~", Token::Tilde),
                (">", Token::Greater),
                ("<", Token::Less),
                (",", Token::Comma),
                (";", Token::Semicolon),
                ("(", Token::LParen),
                (")", Token::RParen),
                ("{", Token::LBrace),
                ("}", Token::RBrace),
                ("%", Token::Modulo),
                ("&", Token::And),
                ("|", Token::Or),
                ("^", Token::Or),
                ("?", Token::QuestionMark),
                (":", Token::Colon),
            ],
        ) {
            tokens.push(token);
        } else {
            return Err(format!("Unrecognized character '{}'", ch));
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
            &tokens,
            &[
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
            &tokens,
            &[
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
            &tokens,
            &[
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
