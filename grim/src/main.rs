use std::{
    fs::File,
    io::{self, Read, Result, Write},
    process::exit,
};
mod compiler;
mod err;
mod lang_core;
mod vm;

use vm::interpret;
fn run_repl() -> Result<()> {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;
        if line.len() == 0 {
            println!();
            return Ok(());
        }
        if let Err(err) = interpret(&line) {
            eprintln!("{}", err);
            vm::VM.lock().reset_stack();
        }
        line = String::new();
    }
}

fn run_file(file: &str) -> Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    if let Err(err) = interpret(&buffer) {
        eprintln!("{}", err);
        exit(err.1);
    }
    Ok(())
}

fn main() -> Result<()> {
    vm::VM.lock().init();
    let opts = std::env::args().collect::<Vec<String>>();
    if opts.len() == 2 {
        run_file(&opts[0])
    } else if opts.len() == 1 {
        run_repl()
    } else {
        eprintln!("[usage] grim <file>");
        exit(1)
    }
}
