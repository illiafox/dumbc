/// Token type, emitted by the lexer.
/// Semantic meaning is further documented in the [AST] section,
/// unless it's not; some unambiguous tokens are explained here anyway.
///
/// [AST]: crate::ast::types
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// A character literal, such as 'k', '\n'
    CharLiteral(char),

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

impl TryFrom<&str> for Token {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            ">>=" => Token::ShiftRightEqual,
            "<<=" => Token::ShiftLeftEqual,
            "++" => Token::PlusPlus,
            "--" => Token::MinusMinus,
            "+=" => Token::PlusEqual,
            "-=" => Token::MinusEqual,
            "/=" => Token::SlashEqual,
            "*=" => Token::AsteriskEqual,
            "==" => Token::EqualEqual,
            "!=" => Token::BangEqual,
            ">=" => Token::GreaterEqual,
            "<=" => Token::LessEqual,
            "&&" => Token::AndAnd,
            "||" => Token::OrOr,
            "%=" => Token::ModuloEqual,
            "&=" => Token::AndEqual,
            "^=" => Token::XorEqual,
            ">>" => Token::ShiftRight,
            "<<" => Token::ShiftLeft,
            "|=" => Token::OrEqual,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Asterisk,
            "/" => Token::Slash,
            "=" => Token::Equal,
            "!" => Token::Bang,
            "~" => Token::Tilde,
            ">" => Token::Greater,
            "<" => Token::Less,
            "," => Token::Comma,
            ";" => Token::Semicolon,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "%" => Token::Modulo,
            "&" => Token::And,
            "|" => Token::Or,
            "^" => Token::Xor,
            "?" => Token::QuestionMark,
            ":" => Token::Colon,
            _ => return Err(()),
        })
    }
}
