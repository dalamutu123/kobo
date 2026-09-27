use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {

        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_line = if self.tokens.is_empty() {
            1
        } else {
            self.tokens.last().unwrap().line
        };

        self.start = self.current;
        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: eof_line,
        });
        
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Single-character tokens
            '(' => self.add(TokenType::LParen),
            ')' => self.add(TokenType::RParen),
            '{' => self.add(TokenType::LBrace),
            '}' => self.add(TokenType::RBrace),
            ',' => self.add(TokenType::Comma),
            '-' => self.add(TokenType::Minus),
            '+' => self.add(TokenType::Plus),
            ';' => self.add(TokenType::Semicolon),
            '*' => self.add(TokenType::Star),

            // Double-character tokens
            '!' => {
            if self.matches('=') {
                self.add(TokenType::BangEqual);
            } else {
                self.add(TokenType::Bang);
            }},

            '=' => {
            if self.matches('=') {
                self.add(TokenType::EqualEqual);
            } else {
                self.add(TokenType::Equal);
            }
            },

            '<' => {
            if self.matches('=') {
                self.add(TokenType::LessEqual);
            } else {
                self.add(TokenType::Less);
            }
            },

            '>' => {
            if self.matches('=') {
                self.add(TokenType::GreaterEqual);
            } else {
                self.add(TokenType::Greater);
            }
            },

            // Comment
            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash);
                }
            },

            // Whitespace
            ' ' | '\t' | '\r' => {},

            // Newline
            '\n' => self.line += 1,
            
            // String
            '"' => self.string(), 

            // Number
            c if c.is_ascii_digit() => self.number(),

            // Identifier
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),

            _ => self.error(self.line, "Character is not part of any token."),


            
        }
        
    }

    fn string(&mut self) {
        // TODO(you): scan a string literal. A string may span lines, and an unterminated one is
        //            reported at the line it opened on (1.5; the message is in 5.1).
        todo!("string")
    }

    fn number(&mut self) {
        // TODO(you): scan a number literal: digits, then a fractional part only when a digit
        //            follows the dot (1.4).
        todo!("number")
    }

    fn identifier(&mut self) {
        // TODO(you): scan an identifier, then decide whether it is a keyword; keyword() in
        //            token.rs does the lookup (1.2, 1.3).
        todo!("identifier")
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(),
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
}
