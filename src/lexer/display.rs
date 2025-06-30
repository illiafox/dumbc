use crate::lexer::Token;
use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}
