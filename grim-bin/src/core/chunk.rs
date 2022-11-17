use super::Value;
use crate::runtime::Ip;
use std::fmt::Display;
#[derive(Default, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Line,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write<T: Into<u8>>(&mut self, byte: T, line: u32) {
        self.code.push(byte.into());
        self.lines.push(line);
    }

    pub fn constant<T: Into<Value>>(&mut self, value: T) -> u8 {
        self.constants.push(value.into());
        (self.constants.len() - 1) as u8
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("== test ==\n");
        let mut ip = Ip::from(self);
        let mut pos = 0;
        loop {
            out.push_str(&format!("{:04} {:04} ", pos, ip.line(pos as u8)));
            let (count, string) = ip.dissasemble_instruction();
            out.push_str(&string);
            out.push('\n');
            for _ in 0..count {
                pos += 1;
            }
            if ip.current == ip.tail {
                break;
            }
        }
        write!(f, "{}", out)
    }
}

#[derive(Default, Debug)]
pub struct Line {
    pub lines: Vec<(u32, u32)>,
}

impl Line {
    pub fn push(&mut self, line: u32) {
        for (l, c) in &mut self.lines {
            if *l == line {
                *c += 1;
                return;
            }
        }
        self.lines.push((line, 1));
    }

    pub fn get_line(&self, loc: u8) -> u32 {
        let mut loc = loc as u32;
        for (l, c) in &self.lines {
            if loc < *c {
                return *l;
            } else {
                loc -= c;
            }
        }
        0
    }
}
macro_rules! op_code {
    ( $($code: tt, $value: literal),* ) => {
        #[derive(Debug)]
        pub enum OpCode {
            $($code,)*
        }

        impl From<OpCode> for u8 {
            fn from(code: OpCode) -> Self {
                match code {
                    $( OpCode::$code => $value, )*
                }
            }
        }

        impl From<u8> for OpCode {
            fn from(byte: u8) -> Self {
                match byte {
                    $( $value => OpCode::$code, )*
                    _ => panic!("unknown opcode."),
                }
            }
        }
    };
}

op_code! {Return, 0, Constant, 1, Add, 2, Subtract, 3, Divide, 4, Multiply, 5, Negate, 6 }
