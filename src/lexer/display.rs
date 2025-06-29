use crate::lexer::Token;
use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::KeywordInt => write!(f, "Keyword<int>"),
            Token::KeywordReturn => write!(f, "Keyword<return>"),
            Token::Identifier(name) => write!(f, "Identifier<{}>", name),
            Token::IntLiteral(n) => write!(f, "IntLiteral<{}>", n),
            Token::LParen => write!(f, "Symbol<(>"),
            Token::RParen => write!(f, "Symbol<)>"),
            Token::LBrace => write!(f, "Symbol<{{>"),
            Token::RBrace => write!(f, "Symbol<}}>"),
            Token::Semicolon => write!(f, "Symbol<;>"),
            Token::Minus => write!(f, "Symbol<->"),
            Token::Tilde => write!(f, "Symbol<~>"),
            Token::Bang => write!(f, "Symbol<!>"),
        }
    }
}
