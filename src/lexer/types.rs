/// Token type, emitted by the lexer.
/// Semantic meaning is further documented in the [AST] section,
/// unless it's not; some unambiguous tokens are explained here anyway.
///
/// [AST]: crate::ast::types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Literal "int"
    KeywordInt,
    /// Literal "return"
    KeywordReturn,
    /// Literal "bingus"
    KeywordBingus,
    /// Literal "if"
    KeywordIf,
    /// Literal "else"
    KeywordElse,
    /// Literal "while"
    KeywordWhile,
    /// Literal "for"
    KeywordFor,
    /// Literal "do"
    KeywordDo,
    /// Literal "break"
    KeywordBreak,
    /// Literal "continue"
    KeywordContinue,

    /// Identifier, such as "main"
    Identifier(String),
    /// An `int` literal, such as 123
    IntLiteral(i32),

    /// Literal "("
    LParen,
    /// Literal ")"
    RParen,
    /// Literal "{"
    LBrace,
    /// Literal "}"
    RBrace,
    /// Literal ";"
    Semicolon,

    /// Literal "-"
    Minus,
    /// Literal "~"
    Tilde,
    /// Literal "!"
    Bang,

    /// Literal "+"
    Plus,
    /// Literal "*"
    Asterisk,
    /// Literal "/"
    Slash,

    /// Literal "&"
    And,
    /// Literal "&="
    AndEqual,
    /// Literal "*"
    Or,
    /// Literal "*"
    OrEqual,
    /// Literal "*"
    Xor,
    /// Literal "*"
    XorEqual,

    /// Literal "<<"
    ShiftLeft,
    /// Literal "<<=".
    /// Binary shorthand left shift assignment
    ShiftLeftEqual,
    /// Literal ">>"
    ShiftRight,
    /// Literal ">>=".
    /// Binary shorthand right shift assignment
    ShiftRightEqual,

    /// Literal "%"
    Modulo,
    /// Literal "%=".
    /// Binary shorthand modulo assignment
    ModuloEqual,

    /// Literal "&&"
    AndAnd,
    /// Literal "||"
    OrOr,

    /// Literal "=="
    EqualEqual,
    /// Literal "!="
    BangEqual,
    /// Literal "<"
    Less,
    /// Literal "<="
    LessEqual,
    /// Literal ">"
    Greater,
    /// Literal ">="
    GreaterEqual,

    /// Literal "+=".
    /// Binary shorthand addition assignment
    PlusEqual,
    /// Literal "-=".
    /// Binary shorthand substraction assignment
    MinusEqual,
    /// Literal "*=".
    /// Binary shorthand multiplication assignment
    AsteriskEqual,
    /// Literal "/=".
    /// Binary shorthand division assignment
    SlashEqual,

    /// Literal ","
    Comma,

    /// Literal "++".
    /// Unary increment operator
    PlusPlus,
    /// Literal "--".
    /// Unary decrement operator
    MinusMinus,

    /// Literal "=".
    Equal,

    /// Literal "?" (part of the ternary expression)
    QuestionMark,
    /// Literal ":" (part of the ternary expression)
    Colon,
}
