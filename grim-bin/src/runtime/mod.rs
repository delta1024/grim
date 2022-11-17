use crate::core::{
    chunk::{Chunk, OpCode},
    Value,
};
use std::{fmt::Display, pin::Pin, result};

pub mod ip;
pub use ip::Ip;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(pub String);
impl Error {
    fn new<T>(message: &str) -> Result<T> {
        Err(Self(message.to_owned()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

macro_rules! error {
    ($string: tt, $($var: expr),*) => {
      Err(format!($string, $($var,)*).into())
    };
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

    pub fn interpret(&mut self, chunk: Chunk) -> Result<()> {
        let chunk = self.chunk.insert(Box::pin(chunk));
        let chunk = chunk.as_ref().get_ref();
        self.ip = chunk.into();
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
            }
        }
    }
}
