use std::fmt;

use crate::{Object, TokenType};

pub struct Token {
    typ: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(
        typ: TokenType,
        lexeme: impl Into<String>,
        literal: Option<Object>,
        line: usize,
    ) -> Self {
        Self {
            typ,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = {
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        };
        write!(f, "{} {} {}", self.typ, self.lexeme, literal)
    }
}
