use crate::core::{
    chunk::{Chunk, Line, OpCode},
    Value,
};
use std::ptr;

impl From<&Chunk> for Ip {
    fn from(chunk: &Chunk) -> Self {
        unsafe {
            Ip {
                head: chunk.code.as_ptr(),
                tail: chunk.code.as_ptr().add(chunk.code.len()),
                current: chunk.code.as_ptr(),
                lines: &chunk.lines,
                constants: chunk.constants.as_ptr(),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Ip {
    pub head: *const u8,
    pub tail: *const u8,
    pub current: *const u8,
    pub lines: *const Line,
    pub constants: *const Value,
}

impl Iterator for Ip {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.tail {
            return None;
        }
        unsafe {
            self.current = self.current.add(1);
            Some(self.current.sub(1).read())
        }
    }
}

impl Ip {
    pub fn constant(&self, loc: u8) -> Value {
        let loc = loc as usize;
        unsafe { self.constants.add(loc).read() }
    }

    pub fn line(&self, loc: u8) -> u32 {
        let line = unsafe { self.lines.as_ref().expect("initialized chunk.") };
        line.get_line(loc)
    }
    pub fn null() -> Self {
        let null = ptr::null();
        Self {
            head: null,
            tail: null,
            current: null,
            lines: ptr::null(),
            constants: ptr::null(),
        }
    }
    pub fn dissasemble_instruction(&mut self) -> (usize, String) {
        let code = OpCode::from(self.next().expect("end of file"));
        match code {
            OpCode::Constant => {
                let pos = self.next().expect("end of file");
                (2, format!("{:?}    {} '{}'", code, pos, self.constant(pos)))
            }
            _ => (1, format!("{:?}", code)),
        }
    }
}
