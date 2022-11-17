use std::{fmt::Display, marker::PhantomData, result};
#[derive(Debug)]
pub struct Error {
    message: String,
    line: usize,
}
impl Error {
    fn new(message: &str, line: usize) -> Result<Token> {
        Err(Self {
            message: message.into(),
            line,
        })
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {}", self.line, self.message)
    }
}
macro_rules! error {
    ( $line: expr, $message: tt, $( $value: expr ),* ) => {
        {

            let message = format!($message, $($value,)*);
            Error::new(&message, $line)
        }
    };

    ( $line: expr, $message: tt ) => {
        Error::new($message, $line)
    };
}
type Result<T> = result::Result<T, Error>;
pub struct Scanner<'a> {
    start: *const u8,
    current: *const u8,
    tail: *const u8,
    at_end: bool,
    line: usize,
    _marker: PhantomData<&'a str>,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Self {
        Self {
            start: source.as_ptr(),
            current: source.as_ptr(),
            tail: unsafe { source.as_ptr().add(source.len()) },
            at_end: false,
            line: 1,
            _marker: PhantomData,
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end {
            return None;
        }
        self.skip_whitespace();
        self.start = self.current;
        let Some(char) = self.advance() else {
            // Signel the end of file.
            self.at_end = true;
            return Some(Ok(self.make_token(TokenType::EOF)));
        };
        if is_alpha_numer(char) {
            return Some(self.identifier());
        }
        if char.is_ascii_digit() {
            return Some(self.number());
        }
        let id = match char {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '+' => TokenType::Plus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,
            '=' if self.peek() == Some('=') => {
                self.advance();
                TokenType::EqualEqual
            }
            '=' => TokenType::Equal,
            '>' if self.peek() == Some('=') => {
                self.advance();
                TokenType::GreaterEqual
            }
            '>' => TokenType::Greater,
            '<' if self.peek() == Some('=') => {
                self.advance();
                TokenType::LessEqual
            }
            '<' => TokenType::Less,
            '!' if self.peek() == Some('=') => {
                self.advance();
                TokenType::BangEqual
            }
            '!' => TokenType::Bang,
            '.' if self.peek() == Some('.') => {
                self.advance();
                TokenType::DotDot
            }
            '.' => TokenType::Dot,
            '-' if self.peek() == Some(':') => {
                self.advance();
                TokenType::MinusColon
            }

            '-' => TokenType::Minus,
            '"' => return Some(self.string()),
            '\'' => return Some(self.char()),
            _ => return Some(error!(self.line, "unexpected character '{}'", char)),
        };
        Some(Ok(self.make_token(id)))
    }
}
fn is_alpha_numer(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
impl Scanner<'_> {
    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        unsafe {
            self.current = self.current.add(1);
            Some(self.current.sub(1).read() as char)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tail
    }

    fn make_token(&self, id: TokenType) -> Token {
        Token {
            id,
            start: self.start,
            len: unsafe { self.current.offset_from(self.start) as usize },
            line: self.line,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        unsafe { Some(self.current.read() as char) }
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        } else if unsafe { self.current.add(1) == self.tail } {
            return None;
        }

        unsafe { Some(self.current.add(1).read() as char) }
    }
    fn string(&mut self) -> Result<Token> {
        while !self.is_at_end() && self.peek() != Some('"') {
            if let Some(n) = self.advance() {
                if n == '\n' {
                    self.line += 1;
                }
            }
        }
        // Consume the second '"'
        self.advance();
        if self.is_at_end() {
            return error!(self.line, "unterminated string.");
        }
        Ok(self.make_token(TokenType::String))
    }

    fn char(&mut self) -> Result<Token> {
        if self.peek_next() != Some('\'') {
            return error!(self.line, "unterminated character.");
        }
        self.advance();
        self.advance();
        Ok(self.make_token(TokenType::CharLit))
    }

    fn number(&mut self) -> Result<Token> {
        while let Some(n) = self.peek() {
            if !n.is_ascii_digit() {
                break;
            }
            self.advance();
        }
        Ok(self.make_token(TokenType::Number))
    }
    fn skip_whitespace(&mut self) {
        loop {
            let Some(char) = self.peek() else {
                break;
            };
            match char {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    continue;
                }
                '/' if self.peek_next() == Some('/') => {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                }
                '/' if self.peek_next() == Some('*') => {
                    while !(self.peek() == Some('*') && self.peek_next() == Some('/'))
                        && !self.is_at_end()
                    {
                        self.advance();
                    }
                    // Consume the '*'
                    self.advance();
                    // Consume the '/'
                    self.advance();
                }
                _ => break,
            }
        }
    }
    fn check_identifier(&self, start: usize, len: usize, rest: &str, id: TokenType) -> TokenType {
        let slice = unsafe { std::slice::from_raw_parts(self.start.add(start), len) };
        let str = unsafe { std::str::from_utf8_unchecked(slice) };
        if str == rest {
            id
        } else {
            TokenType::Identifier
        }
    }
    fn id_type(&self) -> TokenType {
        let (start, len, rest, id) = match unsafe { self.start.read() as char } {
            'b' => (1, 3, "ind", TokenType::Bind),
            'c' => (1, 3, "har", TokenType::Char),
            'd' => (1, 2, "ef", TokenType::Def),
            'e' => (1, 3, "num", TokenType::Enum),
            'f' => (1, 4, "alse", TokenType::False),
            'i' => (1, 2, "nt", TokenType::Int),
            'n' => (1, 2, "il", TokenType::Nil),
            't' => match unsafe { self.start.add(1).read() as char } {
                'r' => (2, 2, "ue", TokenType::True),
                'y' => (2, 5, "pedef", TokenType::Typedef),
                _ => return TokenType::Identifier,
            },
            's' => (1, 4, "ruct", TokenType::Struct),

            // struct
            _ => return TokenType::Identifier,
        };
        self.check_identifier(start, len, rest, id)
    }
    fn identifier(&mut self) -> Result<Token> {
        while is_alpha_numer(self.peek().unwrap()) || self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }
        let id = self.id_type();
        Ok(self.make_token(id))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub id: TokenType,
    start: *const u8,
    len: usize,
    pub line: usize,
}
impl Token {
    pub fn extract(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.start, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}
impl Default for Token {
    fn default() -> Self {
        Self {
            start: std::ptr::null(),
            id: TokenType::default(),
            len: usize::default(),
            line: usize::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    RightBracket,
    LeftBracket,
    Plus,
    Star,
    Slash,
    Comma,
    Semicolon,
    // One or more character tokens
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Bang,
    BangEqual,
    Dot,
    DotDot,
    Minus,
    MinusColon,
    // Literals
    Number,
    String,
    Identifier,
    CharLit,
    // Keywords
    True,
    False,
    Struct,
    Enum,
    Char,
    Int,
    Nil,
    Typedef,
    Bind,
    Def,
    #[default]
    EOF,
}
