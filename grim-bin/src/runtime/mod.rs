use crate::{
    compiler::{compile, scanner::Error as ErrorToken, Error as CompilerError},
    core::{
        chunk::{Chunk, OpCode},
        Value,
    },
};
use std::{error, fmt::Display, pin::Pin, result};

pub mod ip;
pub use ip::Ip;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(pub String, pub i32);
impl Error {
    pub fn new<T>(message: String) -> Result<T> {
        Err(Self(message, 70))
    }
}
impl error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<ErrorToken> for Error {
    fn from(e: ErrorToken) -> Self {
        Self(format!("{}", e), 65)
    }
}
impl From<CompilerError> for Error {
    fn from(e: CompilerError) -> Self {
        Self(format!("{}", e), 65)
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s, 70)
    }
}

macro_rules! error {
    ($string: tt, $($var: expr),*) => {
      Error::new(format!($string, $($var,)*))
    };
    ($string: tt) => {
        Error::new($string.into())
    }
}
const STACK_MAX: usize = 255;
pub struct Vm {
    chunk: Option<Pin<Box<Chunk>>>,
    ip: Ip,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: Ip::null(),
            stack: [0; STACK_MAX],
            stack_top: 0,
        }
    }

    fn push(&mut self, val: Value) {
        self.stack_top += 1;
        self.stack[self.stack_top - 1] = val;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn read_byte(&mut self) -> u8 {
        self.ip.next().expect("end of file")
    }

    fn read_constant(&mut self) -> Value {
        let loc = self.read_byte();
        self.ip.constant(loc)
    }

    fn binary_operation(&mut self, code: OpCode) -> Result<()> {
        let b = self.pop();
        let a = self.pop();
        let result: Value = match code {
            OpCode::Add => a + b,
            OpCode::Subtract => a - b,
            OpCode::Divide => a / b,
            OpCode::Multiply => a * b,
            _ => unreachable!(),
        };
        self.push(result);
        Ok(())
    }
    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let chunk = self.chunk.insert(Box::pin(compile(source)?));
        self.ip = chunk.as_ref().get_ref().into();
        self.run()
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            #[cfg(feature = "trace_execution")]
            {
                let mut new_ip = self.ip;
                for i in &self.stack[..self.stack_top] {
                    print!("[ {} ]", i);
                }
                print!("\n");
                let (_, out) = new_ip.dissasemble_instruction();
                println!("{}", out);
            }
            let byte = OpCode::from(self.read_byte());
            match byte {
                OpCode::Return => {
                    if self.stack_top > 0 {
                        println!("{}", self.pop());
                    }
                    return Ok(());
                }
                OpCode::Constant => {
                    let val = self.read_constant();
                    self.push(val);
                }
                OpCode::Add | OpCode::Subtract | OpCode::Divide | OpCode::Multiply => {
                    self.binary_operation(byte)?;
                }
                OpCode::Negate => {
                    let val = self.pop();
                    self.push(-val);
                }
            }
        }
    }
}
