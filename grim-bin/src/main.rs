use std::{
    fs::File,
    io::{self, Read, Result, Write},
    process::exit,
};
mod compiler;
mod core;
mod runtime;
use runtime::Vm;
fn run_repl(vm: &mut Vm) -> Result<()> {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;
        if line.len() == 0 {
            println!();
            return Ok(());
        }
        if let Err(err) = vm.interpret(&line) {
            eprintln!("{}", err);
        }
        line = String::new();
    }
}
fn run_file(vm: &mut Vm, file: &str) -> Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    if let Err(err) = vm.interpret(&buffer) {
        eprintln!("{}", err);
        exit(err.1);
    }
    Ok(())
}
fn main() -> Result<()> {
    let mut vm = Vm::new();
    let opts = std::env::args().collect::<Vec<String>>();
    if opts.len() == 2 {
        run_file(&mut vm, &opts[0])
    } else if opts.len() == 1 {
        run_repl(&mut vm)
    } else {
        eprintln!("[usage] grim <file>");
        exit(1)
    }
}
