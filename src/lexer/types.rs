#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeywordInt,         // "int"
    KeywordReturn,      // "return"
    Identifier(String), // e.g. "main"
    IntLiteral(i32),    // e.g. 123
    LParen,             // (
    RParen,             // )
    LBrace,             // {
    RBrace,             // }
    Semicolon,          // ;
    Minus,              // -
    Tilde,              // ~
    Bang,               // !
    Plus,               // +
    Asterisk,           // *
    Slash,              // /
}
