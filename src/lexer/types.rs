#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    KeywordInt,    // "int"
    KeywordReturn, // "return"

    // Identifiers and literals
    Identifier(String), // e.g. "main"
    IntLiteral(i32),    // e.g. 123

    // Punctuation
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Semicolon, // ;

    // Unary operators
    Minus, // -
    Tilde, // ~
    Bang,  // !

    // Binary arithmetic operators
    Plus,     // +
    Asterisk, // *
    Slash,    // /

    // Logical operators
    AndAnd, // &&
    OrOr,   // ||

    // Comparison operators
    EqualEqual,   // ==
    BangEqual,    // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    Equal,
}
