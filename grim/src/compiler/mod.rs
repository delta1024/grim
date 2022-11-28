use crate::{allocate_string, lang_core::prelude::*};

use std::result;
mod functions;
mod rules;
pub mod scanner;
use functions::*;

use self::scanner::TokenType;
use crate::err::CompilerError;
pub use scanner::{Scanner, Token};
pub type Result<T> = result::Result<T, CompilerError>;

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
        Err(CompilerError::new(&out, token.line))
    }

    fn synchronize(&mut self) {
        while self.current.id != TokenType::EOF {
            if self.previous.id == TokenType::Semicolon {
                return;
            }

            match self.current.id {
                TokenType::If
                | TokenType::Def
                | TokenType::Bind
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.next();
                }
            }
        }
    }
}

impl Parser<'_> {
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
    fn emit_constant<T: Into<Type>>(&mut self, value: T) {
        let loc = self.current_chunk().constant(value);
        self.emit_bytes(OpCode::Constant, loc);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn matches(&mut self, id: TokenType) -> bool {
        if self.current.id != id {
            return false;
        }
        self.next();
        true
    }
    fn end_compiler(&mut self) {
        self.emit_return();
        #[cfg(feature = "print_code")]
        println!("{}", self.current_chunk());
    }
    fn identifier_constant(&mut self, name: Token) -> u8 {
        let string = allocate_string!(name.extract());
        self.current_chunk().constant(string)
    }
    fn define_variable(&mut self, global: u8) {
        self.emit_bytes(OpCode::DefineGlobal, global);
    }
    fn named_variable(&mut self, name: Token, can_assign: bool) -> Result<()> {
        let op = if can_assign && self.matches(TokenType::Equal) {
            expression(self)?;
            OpCode::SetGlobal
        } else {
            OpCode::GetGlobal
        };
        let arg = self.identifier_constant(name);
        self.emit_bytes(op, arg);
        Ok(())
    }
}

pub fn compile(source: &str) -> Result<Chunk> {
    let mut parser = Parser::new(source);
    // Prime the pump.
    parser.next();
    while !parser.matches(TokenType::EOF) {
        declaration(&mut parser)?;
    }
    parser.end_compiler();
    Ok(parser.chunk)
}
