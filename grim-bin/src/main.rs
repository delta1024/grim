use std::process::exit;
mod core;
mod runtime;
use crate::core::chunk::{Chunk, OpCode};
use runtime::Vm;
fn main() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let constant = chunk.constant(25 as u32);
    chunk.write(OpCode::Constant, 123);
    chunk.write(constant, 123);
    chunk.write(OpCode::Return, 123);

    println!("{}", chunk);
    if let Err(err) = vm.interpret(chunk) {
        eprintln!("{}", err);
        exit(1);
    }
}
