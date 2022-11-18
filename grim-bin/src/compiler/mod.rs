use crate::core::{
    chunk::{Chunk, OpCode},
    Value,
};
use std::{fmt::Display, result};
mod functions;
mod rules;
pub mod scanner;
use functions::*;

pub use scanner::{Error as ScannError, Scanner, Token};

use self::scanner::TokenType;
pub type Result<T> = result::Result<T, Error>;
#[derive(Debug)]
pub struct Error(String, usize);
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error {}", self.1, self.0)
    }
}
impl From<ScannError> for Error {
    fn from(e: ScannError) -> Self {
        Self(format!(":{}", e), e.line)
    }
}

struct Parser<'a> {
    previous: Token,
    current: Token,
    scanner: Scanner<'a>,
    chunk: Chunk,
}
impl Iterator for Parser<'_> {
    type Item = Result<()>;
    fn next(&mut self) -> Option<Self::Item> {
        self.previous = self.current;
        self.current = match self.scanner.next() {
            Some(Err(err)) => return Some(Err(err.into())),
            Some(Ok(token)) => token,
            None => return None,
        };
        Some(Ok(()))
    }
}
impl Parser<'_> {
    fn new(source: &str) -> Self {
        Self {
            previous: Token::default(),
            current: Token::default(),
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
        }
    }
}
impl Parser<'_> {
    fn error_at_current<T>(&self, message: &str) -> Result<T> {
        self.error_at(self.current, message)
    }
    fn error<T>(&self, message: &str) -> Result<T> {
        self.error_at(self.previous, message)
    }
    fn error_at<T>(&self, token: Token, message: &str) -> Result<T> {
        let mut out = match token.id {
            TokenType::EOF => " at end".into(),
            _ => format!(" at '{}'", token.extract()),
        };
        out.push_str(&format!(": {}\n", message));
        Err(Error(out, token.line))
    }
    fn consume(&mut self, id: TokenType, message: &str) -> Result<()> {
        if self.current.id == id {
            self.next();
            return Ok(());
        }
        self.error_at_current(message)
    }
    fn emit_byte<T: Into<u8>>(&mut self, byte: T) {
        let line = self.previous.line as u32;
        self.current_chunk().write(byte, line);
    }
    fn emit_bytes<T: Into<U>, U: Into<u8>>(&mut self, byte1: T, byte2: U) {
        self.emit_byte(byte1.into());
        self.emit_byte(byte2);
    }
    fn emit_constant<T: Into<Value>>(&mut self, value: T) {
        let loc = self.current_chunk().constant(value);
        self.emit_bytes(OpCode::Constant, loc);
    }
    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }
    fn end_compiler(&mut self) {
        self.emit_return();
        #[cfg(feature = "print_code")]
        println!("{}", self.chunk);
    }
}

pub fn compile(source: &str) -> Result<Chunk> {
    let mut parser = Parser::new(source);
    // Prime the pump.
    parser.next();
    expression(&mut parser)?;
    parser.consume(TokenType::EOF, "Expected end of expression.")?;
    parser.end_compiler();
    Ok(parser.chunk)
}
