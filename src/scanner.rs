use phf::phf_map;

use crate::{LoxError, Object, Token, TokenType};

static KEYWORDS: phf::Map<&str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" =>  TokenType::This,
    "true" => TokenType::True,
    "var" =>  TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn from_source(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => return, // Ignore whiespace.
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    LoxError::error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn add_token(&mut self, typ: TokenType) {
        self.add_token_literal(typ, None)
    }

    fn add_token_literal(&mut self, typ: TokenType, literal: Option<Object>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(typ, text, literal, self.line))
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        let Some(&c) = &self.source.get(self.current) else {
            return false
        };
        if c != expected {
            return false;
        };

        self.current += 1;
        true
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(&typ) = KEYWORDS.get(text.as_str()) {
            self.add_token(typ);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        // look for a fractional part.
        if self.peek() == '.' && is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
            while is_digit(self.advance()) {
                self.advance();
            }
        }

        let double_string: String = self.source[self.start..self.current].iter().collect();
        let double: f64 = double_string.parse().unwrap();
        self.add_token_literal(TokenType::Number, Some(double.into()));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            LoxError::error(self.line, "Unexpected string.");
            return;
        }

        // The closing.
        self.advance();

        // Trim the surrounding quotes.
        let string: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, Some(string.into()));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        };

        *self.source.get(self.current + 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        *self.source.get(self.current).unwrap()
    }

    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}
