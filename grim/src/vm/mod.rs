use crate::{
    compiler::compile,
    err::VmError,
    lang_core::{objects::Pointable, prelude::*},
};
use std::{pin::Pin, result};

pub mod ip;
pub mod memory;
pub use ip::Ip;
use spin::Mutex;
pub static VM: Mutex<Vm> = Mutex::new(Vm::new());
use self::memory::Memory;

pub type Result<T> = result::Result<T, VmError>;

macro_rules! error {
    ($string: tt, $($var: expr),*) => {
      VmError::new(format!($string, $($var,)*))
    };
    ($string: tt) => {
        VmError::new($string.into())
    }
}

const STACK_MAX: usize = 255;
pub struct Vm {
    ip: Ip,
    stack: [Type; STACK_MAX],
    stack_top: usize,
    pub memory: Memory,
    chunk: Option<Pin<Box<Chunk>>>,
}
unsafe impl Send for Vm {}
unsafe impl Sync for Vm {}

impl Vm {
    pub const fn new() -> Self {
        Self {
            ip: Ip::null(),
            stack: [Type::Nil; STACK_MAX],
            stack_top: 0,
            memory: Memory::new(),
            chunk: None,
        }
    }

    pub fn init(&mut self) {
        self.memory.initialize_memory();
    }
    fn push<T: Into<Type>>(&mut self, val: T) {
        self.stack_top += 1;
        self.stack[self.stack_top - 1] = val.into();
    }

    fn pop(&mut self) -> Type {
        if self.stack_top == 0 {
            return Type::Nil;
        }
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn read_byte(&mut self) -> u8 {
        self.ip.next().expect("end of file")
    }

    fn read_constant(&mut self) -> Type {
        let loc = self.read_byte();
        self.ip.constant(loc)
    }
    fn read_string(&mut self) -> StringPointer {
        let Type::Object(ObjectPointer::String(name)) = self.read_constant() else {
            panic!("Unrecoverable compiler error.");
        };
        name
    }
    fn peek(&self, distance: usize) -> Type {
        self.stack[self.stack_top - distance - 1]
    }
    pub fn reset_stack(&mut self) {
        self.stack_top = 0;
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
                OpCode::DefineGlobal => {
                    let name = self.read_string();
                    self.memory.set_global(name, self.peek(0));
                    self.pop();
                }
                OpCode::SetGlobal => {
                    let name = self.read_string();
                    self.memory.assign_global(name, self.peek(0))?;
                }
                OpCode::GetGlobal => {
                    let name = self.read_string();
                    let Some(value) = self.memory.get_global(name) else {
                        return error!("Undefined variable '{}'", name);
                    };
                    self.push(value);
                }
                OpCode::Return => {
                    return Ok(());
                }
                OpCode::Constant => {
                    let val = self.read_constant();
                    self.push(val);
                }
                OpCode::Subtract
                | OpCode::Divide
                | OpCode::Multiply
                | OpCode::Greater
                | OpCode::Less => match (self.pop(), self.pop()) {
                    (Type::Number(b), Type::Number(a)) => {
                        let n: Type = match byte {
                            OpCode::Less => (a < b).into(),
                            OpCode::Greater => (a > b).into(),
                            OpCode::Subtract => (a - b).into(),
                            OpCode::Divide => (a / b).into(),
                            OpCode::Multiply => (a * b).into(),
                            _ => unreachable!(),
                        };
                        self.push(n);
                    }
                    _ => return error!("Operands must be two numbers"),
                },
                OpCode::Add => match (self.peek(0), self.peek(1)) {
                    (
                        Type::Object(ObjectPointer::String(b)),
                        Type::Object(ObjectPointer::String(a)),
                    ) if a.get_ref().is_some() && b.get_ref().is_some() => {
                        let b = b.get_ref().unwrap();
                        let a = a.get_ref().unwrap();
                        // remove the leading '"'
                        let b = &b[1..];
                        // remove the trailing '"'
                        let a = &a[..a.len() - 1];

                        let s = [a, b].concat();
                        let s = self.memory.allocate_string(&s);
                        self.pop();
                        self.pop();

                        self.push(s);
                    }

                    (Type::Number(b), Type::Number(a)) => {
                        self.pop();
                        self.pop();
                        self.push(a + b);
                    }

                    _ => {
                        return error!("Operands must be two numbers or two strings");
                    }
                },
                OpCode::Negate => {
                    let val: i32 = self.pop().try_into()?;
                    self.push(-val);
                }
                OpCode::True => self.push(true),
                OpCode::False => self.push(false),
                OpCode::Nil => self.push(Type::Nil),
                OpCode::Not => {
                    let val = self.pop().is_falsy();
                    self.push(val);
                }
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a == b);
                }
                OpCode::Print => {
                    println!("{}", self.pop());
                }
                OpCode::Pop => {
                    self.pop();
                }
            }
        }
    }
}

pub fn interpret(source: &str) -> Result<()> {
    let chunk = compile(source)?;
    let mut vm = VM.lock();
    vm.ip = Ip::from(vm.chunk.insert(Box::pin(chunk)).as_ref().get_ref());
    vm.run()
}
